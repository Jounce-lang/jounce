// Jounce Package Registry Server
// Enhanced version with JWT auth, rate limiting, and owner management

require('dotenv').config();
const express = require('express');
const multer = require('multer');
const fs = require('fs').promises;
const path = require('path');
const crypto = require('crypto');
const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');
const rateLimit = require('express-rate-limit');

const app = express();
const PORT = process.env.PORT || 4000;
const JWT_SECRET = process.env.JWT_SECRET || 'dev-secret-key-change-in-production';
const SALT_ROUNDS = 10;

// Storage directories
const REGISTRY_DIR = path.join(__dirname, 'registry');
const PACKAGES_DIR = path.join(REGISTRY_DIR, 'packages');
const METADATA_DIR = path.join(REGISTRY_DIR, 'metadata');
const AUTH_DIR = path.join(REGISTRY_DIR, 'auth');
const USERS_DIR = path.join(AUTH_DIR, 'users');
const OWNERS_DIR = path.join(REGISTRY_DIR, 'owners');

// Middleware
app.use(express.json());
app.use(express.static('public'));

// Rate limiting
const apiLimiter = rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // Limit each IP to 100 requests per windowMs
    message: { error: 'Too many requests, please try again later' }
});

const publishLimiter = rateLimit({
    windowMs: 60 * 60 * 1000, // 1 hour
    max: 10, // Limit publishing to 10 per hour
    message: { error: 'Too many publish requests, please try again later' }
});

app.use('/api/', apiLimiter);

// Configure multer for file uploads
const storage = multer.diskStorage({
    destination: async (req, file, cb) => {
        const packageName = req.body.name;
        const version = req.body.version;
        const packageDir = path.join(PACKAGES_DIR, packageName, version);
        await fs.mkdir(packageDir, { recursive: true });
        cb(null, packageDir);
    },
    filename: (req, file, cb) => {
        cb(null, file.originalname);
    }
});

const upload = multer({ storage });

// Initialize registry directories
async function initRegistry() {
    await fs.mkdir(PACKAGES_DIR, { recursive: true });
    await fs.mkdir(METADATA_DIR, { recursive: true });
    await fs.mkdir(AUTH_DIR, { recursive: true });
    await fs.mkdir(USERS_DIR, { recursive: true });
    await fs.mkdir(OWNERS_DIR, { recursive: true });
    console.log('✅ Registry directories initialized');
}

// Authentication middleware
async function requireAuth(req, res, next) {
    try {
        const authHeader = req.headers['authorization'];

        if (!authHeader || !authHeader.startsWith('Bearer ')) {
            return res.status(401).json({ error: 'Authentication required' });
        }

        const token = authHeader.substring(7); // Remove 'Bearer ' prefix

        // Verify JWT token
        const decoded = jwt.verify(token, JWT_SECRET);

        // Load user data
        const userFile = path.join(USERS_DIR, `${decoded.username}.json`);
        const userData = await fs.readFile(userFile, 'utf-8');
        req.user = JSON.parse(userData);

        next();
    } catch (error) {
        if (error.name === 'TokenExpiredError') {
            return res.status(401).json({ error: 'Token expired' });
        }
        return res.status(401).json({ error: 'Invalid token' });
    }
}

// Routes

// Health check
app.get('/health', (req, res) => {
    res.json({ status: 'ok', version: '1.0.0' });
});

// Register a new user
app.post('/api/v1/auth/register', async (req, res) => {
    try {
        const { username, email, password } = req.body;

        if (!username || !email || !password) {
            return res.status(400).json({ error: 'Username, email, and password required' });
        }

        // Validate username format (lowercase, alphanumeric, hyphens)
        if (!/^[a-z0-9-]+$/.test(username)) {
            return res.status(400).json({
                error: 'Username must be lowercase alphanumeric with hyphens only'
            });
        }

        // Check if user already exists
        const userFile = path.join(USERS_DIR, `${username}.json`);
        try {
            await fs.access(userFile);
            return res.status(409).json({ error: 'Username already exists' });
        } catch {
            // User doesn't exist, proceed
        }

        // Hash password
        const passwordHash = await bcrypt.hash(password, SALT_ROUNDS);

        // Create user data
        const userData = {
            username,
            email,
            passwordHash,
            createdAt: new Date().toISOString(),
            packages: []
        };

        await fs.writeFile(userFile, JSON.stringify(userData, null, 2));

        // Generate JWT token
        const token = jwt.sign(
            { username, email },
            JWT_SECRET,
            { expiresIn: '30d' }
        );

        res.json({
            success: true,
            token,
            username,
            message: 'Registration successful'
        });
    } catch (error) {
        console.error('Registration error:', error);
        res.status(500).json({ error: error.message });
    }
});

// Login endpoint
app.post('/api/v1/auth/login', async (req, res) => {
    try {
        const { username, password } = req.body;

        if (!username || !password) {
            return res.status(400).json({ error: 'Username and password required' });
        }

        // Load user data
        const userFile = path.join(USERS_DIR, `${username}.json`);
        let userData;
        try {
            const data = await fs.readFile(userFile, 'utf-8');
            userData = JSON.parse(data);
        } catch {
            return res.status(401).json({ error: 'Invalid credentials' });
        }

        // Verify password
        const validPassword = await bcrypt.compare(password, userData.passwordHash);
        if (!validPassword) {
            return res.status(401).json({ error: 'Invalid credentials' });
        }

        // Generate JWT token
        const token = jwt.sign(
            { username: userData.username, email: userData.email },
            JWT_SECRET,
            { expiresIn: '30d' }
        );

        res.json({
            success: true,
            token,
            username: userData.username,
            message: 'Login successful'
        });
    } catch (error) {
        console.error('Login error:', error);
        res.status(500).json({ error: error.message });
    }
});

// Publish a package
app.post('/api/v1/packages/publish', publishLimiter, requireAuth, upload.single('package'), async (req, res) => {
    try {
        const { name, version, description, keywords } = req.body;

        if (!name || !version) {
            return res.status(400).json({ error: 'Package name and version required' });
        }

        // Validate package name format
        if (!/^[a-z0-9-]+$/.test(name)) {
            return res.status(400).json({
                error: 'Package name must be lowercase alphanumeric with hyphens only'
            });
        }

        // Validate semantic versioning
        const versionRegex = /^\d+\.\d+\.\d+$/;
        if (!versionRegex.test(version)) {
            return res.status(400).json({
                error: 'Invalid version format. Use semantic versioning (e.g., 1.0.0)'
            });
        }

        // Check if version already exists
        const versionFile = path.join(METADATA_DIR, name, `${version}.json`);
        try {
            await fs.access(versionFile);
            return res.status(409).json({ error: 'Version already exists' });
        } catch {
            // Version doesn't exist, proceed
        }

        // Check package ownership
        const ownerFile = path.join(OWNERS_DIR, `${name}.json`);
        let owners;
        try {
            const data = await fs.readFile(ownerFile, 'utf-8');
            owners = JSON.parse(data);

            // Verify user is an owner
            if (!owners.owners.includes(req.user.username)) {
                return res.status(403).json({ error: 'You are not an owner of this package' });
            }
        } catch {
            // Package doesn't exist yet, user becomes first owner
            owners = {
                package: name,
                owners: [req.user.username],
                createdAt: new Date().toISOString()
            };
            await fs.writeFile(ownerFile, JSON.stringify(owners, null, 2));
        }

        // Save package metadata
        const metadata = {
            name,
            version,
            description: description || '',
            author: req.user.username,
            keywords: keywords ? keywords.split(',').map(k => k.trim()) : [],
            publishedAt: new Date().toISOString(),
            downloads: 0
        };

        const metadataDir = path.join(METADATA_DIR, name);
        await fs.mkdir(metadataDir, { recursive: true });
        await fs.writeFile(versionFile, JSON.stringify(metadata, null, 2));

        // Update package index
        await updatePackageIndex(name, version, metadata);

        // Update user's package list
        if (!req.user.packages.includes(name)) {
            req.user.packages.push(name);
            const userFile = path.join(USERS_DIR, `${req.user.username}.json`);
            await fs.writeFile(userFile, JSON.stringify(req.user, null, 2));
        }

        res.json({
            success: true,
            message: `Package ${name}@${version} published successfully`
        });
    } catch (error) {
        console.error('Publish error:', error);
        res.status(500).json({ error: error.message });
    }
});

// Get package metadata
app.get('/api/v1/packages/:name', async (req, res) => {
    try {
        const { name } = req.params;
        const indexFile = path.join(METADATA_DIR, name, 'index.json');

        const data = await fs.readFile(indexFile, 'utf-8');
        const packageData = JSON.parse(data);

        // Add owners info
        try {
            const ownerFile = path.join(OWNERS_DIR, `${name}.json`);
            const ownerData = await fs.readFile(ownerFile, 'utf-8');
            packageData.owners = JSON.parse(ownerData).owners;
        } catch {
            packageData.owners = [];
        }

        res.json(packageData);
    } catch (error) {
        res.status(404).json({ error: 'Package not found' });
    }
});

// Get specific package version
app.get('/api/v1/packages/:name/:version', async (req, res) => {
    try {
        const { name, version } = req.params;
        const versionFile = path.join(METADATA_DIR, name, `${version}.json`);

        const data = await fs.readFile(versionFile, 'utf-8');
        res.json(JSON.parse(data));
    } catch (error) {
        res.status(404).json({ error: 'Package version not found' });
    }
});

// Download package
app.get('/api/v1/packages/:name/:version/download', async (req, res) => {
    try {
        const { name, version } = req.params;
        const packageDir = path.join(PACKAGES_DIR, name, version);

        // Increment download counter
        await incrementDownloads(name, version);

        // Send package files listing
        const files = await fs.readdir(packageDir);

        res.json({
            name,
            version,
            files: files.map(f => ({
                name: f,
                url: `/api/v1/packages/${name}/${version}/files/${f}`
            }))
        });
    } catch (error) {
        res.status(404).json({ error: 'Package not found' });
    }
});

// Download individual file
app.get('/api/v1/packages/:name/:version/files/:filename', async (req, res) => {
    try {
        const { name, version, filename } = req.params;
        const filePath = path.join(PACKAGES_DIR, name, version, filename);

        const content = await fs.readFile(filePath, 'utf-8');
        res.send(content);
    } catch (error) {
        res.status(404).json({ error: 'File not found' });
    }
});

// Search packages
app.get('/api/v1/search', async (req, res) => {
    try {
        const { q } = req.query;

        if (!q) {
            return res.status(400).json({ error: 'Search query required' });
        }

        const packages = await fs.readdir(METADATA_DIR);
        const results = [];

        for (const pkg of packages) {
            const indexFile = path.join(METADATA_DIR, pkg, 'index.json');
            try {
                const data = await fs.readFile(indexFile, 'utf-8');
                const packageData = JSON.parse(data);

                // Search in name, description, and keywords
                const searchTerm = q.toLowerCase();
                const name = packageData.name.toLowerCase();
                const desc = (packageData.description || '').toLowerCase();
                const keywords = (packageData.keywords || []).join(' ').toLowerCase();

                if (name.includes(searchTerm) ||
                    desc.includes(searchTerm) ||
                    keywords.includes(searchTerm)) {
                    results.push(packageData);
                }
            } catch {
                // Skip if index doesn't exist
            }
        }

        res.json({ results, count: results.length });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// List all packages
app.get('/api/v1/packages', async (req, res) => {
    try {
        const packages = await fs.readdir(METADATA_DIR);
        const results = [];

        for (const pkg of packages) {
            const indexFile = path.join(METADATA_DIR, pkg, 'index.json');
            try {
                const data = await fs.readFile(indexFile, 'utf-8');
                results.push(JSON.parse(data));
            } catch {
                // Skip if index doesn't exist
            }
        }

        res.json({ packages: results, count: results.length });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Get package owners
app.get('/api/v1/packages/:name/owners', async (req, res) => {
    try {
        const { name } = req.params;
        const ownerFile = path.join(OWNERS_DIR, `${name}.json`);

        const data = await fs.readFile(ownerFile, 'utf-8');
        res.json(JSON.parse(data));
    } catch (error) {
        res.status(404).json({ error: 'Package not found' });
    }
});

// Add package owner
app.put('/api/v1/packages/:name/owners', requireAuth, async (req, res) => {
    try {
        const { name } = req.params;
        const { username } = req.body;

        if (!username) {
            return res.status(400).json({ error: 'Username required' });
        }

        // Load current owners
        const ownerFile = path.join(OWNERS_DIR, `${name}.json`);
        const data = await fs.readFile(ownerFile, 'utf-8');
        const owners = JSON.parse(data);

        // Check if requester is an owner
        if (!owners.owners.includes(req.user.username)) {
            return res.status(403).json({ error: 'Only owners can add new owners' });
        }

        // Check if user exists
        const userFile = path.join(USERS_DIR, `${username}.json`);
        try {
            await fs.access(userFile);
        } catch {
            return res.status(404).json({ error: 'User not found' });
        }

        // Add new owner if not already an owner
        if (!owners.owners.includes(username)) {
            owners.owners.push(username);
            await fs.writeFile(ownerFile, JSON.stringify(owners, null, 2));
        }

        res.json({
            success: true,
            message: `Added ${username} as owner of ${name}`,
            owners: owners.owners
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Remove package owner
app.delete('/api/v1/packages/:name/owners/:username', requireAuth, async (req, res) => {
    try {
        const { name, username } = req.params;

        // Load current owners
        const ownerFile = path.join(OWNERS_DIR, `${name}.json`);
        const data = await fs.readFile(ownerFile, 'utf-8');
        const owners = JSON.parse(data);

        // Check if requester is an owner
        if (!owners.owners.includes(req.user.username)) {
            return res.status(403).json({ error: 'Only owners can remove owners' });
        }

        // Don't allow removing the last owner
        if (owners.owners.length === 1) {
            return res.status(400).json({ error: 'Cannot remove the last owner' });
        }

        // Remove owner
        owners.owners = owners.owners.filter(o => o !== username);
        await fs.writeFile(ownerFile, JSON.stringify(owners, null, 2));

        res.json({
            success: true,
            message: `Removed ${username} as owner of ${name}`,
            owners: owners.owners
        });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

// Helper functions

async function updatePackageIndex(name, version, metadata) {
    const indexFile = path.join(METADATA_DIR, name, 'index.json');

    let index;
    try {
        const data = await fs.readFile(indexFile, 'utf-8');
        index = JSON.parse(data);
    } catch {
        index = {
            name,
            versions: [],
            latestVersion: version,
            totalDownloads: 0
        };
    }

    // Add new version
    if (!index.versions.includes(version)) {
        index.versions.push(version);
    }
    index.latestVersion = version;
    index.description = metadata.description;
    index.author = metadata.author;
    index.keywords = metadata.keywords;
    index.updatedAt = metadata.publishedAt;

    await fs.writeFile(indexFile, JSON.stringify(index, null, 2));
}

async function incrementDownloads(name, version) {
    const versionFile = path.join(METADATA_DIR, name, `${version}.json`);

    try {
        const data = await fs.readFile(versionFile, 'utf-8');
        const metadata = JSON.parse(data);
        metadata.downloads = (metadata.downloads || 0) + 1;
        await fs.writeFile(versionFile, JSON.stringify(metadata, null, 2));

        // Also update package index
        const indexFile = path.join(METADATA_DIR, name, 'index.json');
        const indexData = await fs.readFile(indexFile, 'utf-8');
        const index = JSON.parse(indexData);
        index.totalDownloads = (index.totalDownloads || 0) + 1;
        await fs.writeFile(indexFile, JSON.stringify(index, null, 2));
    } catch (error) {
        console.error('Error incrementing downloads:', error);
    }
}

// Start server
initRegistry().then(() => {
    app.listen(PORT, () => {
        console.log(`\n🚀 Jounce Package Registry Server v1.0.0`);
        console.log(`   📦 Running on http://localhost:${PORT}`);
        console.log(`   📂 Registry: ${REGISTRY_DIR}`);
        console.log(`   🔐 JWT Authentication: Enabled`);
        console.log(`   🛡️  Rate Limiting: Enabled`);
        console.log(`\n   API Endpoints:`);
        console.log(`   Authentication:`);
        console.log(`   - POST /api/v1/auth/register - Register new user`);
        console.log(`   - POST /api/v1/auth/login - Login with username/password`);
        console.log(`\n   Package Management:`);
        console.log(`   - POST /api/v1/packages/publish - Publish package (auth required)`);
        console.log(`   - GET  /api/v1/packages - List all packages`);
        console.log(`   - GET  /api/v1/packages/:name - Get package info`);
        console.log(`   - GET  /api/v1/packages/:name/:version - Get version info`);
        console.log(`   - GET  /api/v1/packages/:name/:version/download - Download package`);
        console.log(`   - GET  /api/v1/search?q=term - Search packages`);
        console.log(`\n   Owner Management (auth required):`);
        console.log(`   - GET    /api/v1/packages/:name/owners - List owners`);
        console.log(`   - PUT    /api/v1/packages/:name/owners - Add owner`);
        console.log(`   - DELETE /api/v1/packages/:name/owners/:username - Remove owner\n`);
    });
});
