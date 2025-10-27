// Jounce Server Runtime
// Provides HTTP server and RPC infrastructure for server-side code

const http = require('http');
const url = require('url');
const fs = require('fs');
const path = require('path');
const Database = require('better-sqlite3');

class HttpServer {
    constructor(port = 3000) {
        this.port = port;
        this.rpcHandlers = new Map();
        this.server = null;
    }

    // Register an RPC handler
    rpc(name, handler) {
        this.rpcHandlers.set(name, handler);
    }

    // Start the HTTP server
    start() {
        this.server = http.createServer(async (req, res) => {
            const parsedUrl = url.parse(req.url, true);
            const pathname = parsedUrl.pathname;

            // Serve static files
            if (pathname === '/' || pathname === '/index.html') {
                this.serveFile(res, 'index.html', 'text/html');
            } else if (pathname === '/client.js') {
                this.serveFile(res, 'client.js', 'application/javascript');
            } else if (pathname === '/client-runtime.js') {
                this.serveFile(res, 'client-runtime.js', 'application/javascript');
            } else if (pathname === '/reactivity.js') {
                this.serveFile(res, 'reactivity.js', 'application/javascript');
            } else if (pathname === '/styles.css') {
                this.serveFile(res, 'styles.css', 'text/css');
            } else if (pathname === '/app.wasm') {
                this.serveFile(res, 'app.wasm', 'application/wasm');
            } else if (pathname.startsWith('/rpc/')) {
                // Handle RPC calls
                const rpcName = pathname.slice(5); // Remove '/rpc/' prefix
                await this.handleRPC(rpcName, req, res);
            } else {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('Not Found');
            }
        });

        this.server.listen(this.port, () => {
            console.log(`Server running at http://localhost:${this.port}`);
        });
    }

    // Serve a static file
    serveFile(res, filename, contentType) {
        const filePath = path.join(__dirname, filename);
        fs.readFile(filePath, (err, data) => {
            if (err) {
                res.writeHead(404, { 'Content-Type': 'text/plain' });
                res.end('File not found');
            } else {
                res.writeHead(200, { 'Content-Type': contentType });
                res.end(data);
            }
        });
    }

    // Handle RPC call
    async handleRPC(name, req, res) {
        const handler = this.rpcHandlers.get(name);
        if (!handler) {
            res.writeHead(404, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'RPC handler not found' }));
            return;
        }

        // Read request body
        let body = '';
        req.on('data', chunk => {
            body += chunk.toString();
        });

        req.on('end', async () => {
            try {
                const params = JSON.parse(body || '{}');
                const result = await handler(params);
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify(result));
            } catch (error) {
                res.writeHead(500, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ error: error.message }));
            }
        });
    }
}

// WebAssembly loading utility
function loadWasm(wasmPath) {
    const wasmBytes = fs.readFileSync(wasmPath);
    const wasmModule = new WebAssembly.Module(wasmBytes);
    return new WebAssembly.Instance(wasmModule, {});
}

// ============================================================================
// Database Utilities
// ============================================================================

class DB {
    constructor(filename = 'app.db') {
        // Create database file in dist directory
        const dbPath = path.join(__dirname, filename);
        this.db = new Database(dbPath);
        this.db.pragma('journal_mode = WAL'); // Better concurrency
    }

    // Execute a SQL query (SELECT)
    query(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.all(...params);
        } catch (error) {
            console.error('Query error:', error.message);
            throw error;
        }
    }

    // Execute a SQL command (INSERT, UPDATE, DELETE)
    execute(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.run(...params);
        } catch (error) {
            console.error('Execute error:', error.message);
            throw error;
        }
    }

    // Get a single row
    queryOne(sql, params = []) {
        try {
            const stmt = this.db.prepare(sql);
            return stmt.get(...params);
        } catch (error) {
            console.error('QueryOne error:', error.message);
            throw error;
        }
    }

    // Close database connection
    close() {
        this.db.close();
    }

    // Begin transaction
    transaction(fn) {
        const txn = this.db.transaction(fn);
        return txn();
    }
}

// Global database instance (lazy initialization)
let globalDB = null;

// Get or create global database instance
function getDB(filename = 'app.db') {
    if (!globalDB) {
        globalDB = new DB(filename);
    }
    return globalDB;
}

// Helper functions for common operations
const dbHelpers = {
    // Create a table
    createTable(tableName, columns) {
        const db = getDB();
        const columnDefs = Object.entries(columns)
            .map(([name, type]) => `${name} ${type}`)
            .join(', ');
        const sql = `CREATE TABLE IF NOT EXISTS ${tableName} (${columnDefs})`;
        db.execute(sql);
    },

    // Insert a record
    insert(tableName, data) {
        const db = getDB();
        const keys = Object.keys(data);
        const values = Object.values(data);
        const placeholders = keys.map(() => '?').join(', ');
        const sql = `INSERT INTO ${tableName} (${keys.join(', ')}) VALUES (${placeholders})`;
        const result = db.execute(sql, values);
        return result.lastInsertRowid;
    },

    // Update records
    update(tableName, data, where, whereParams = []) {
        const db = getDB();
        const sets = Object.keys(data).map(key => `${key} = ?`).join(', ');
        const values = [...Object.values(data), ...whereParams];
        const sql = `UPDATE ${tableName} SET ${sets} WHERE ${where}`;
        const result = db.execute(sql, values);
        return result.changes;
    },

    // Delete records
    delete(tableName, where, whereParams = []) {
        const db = getDB();
        const sql = `DELETE FROM ${tableName} WHERE ${where}`;
        const result = db.execute(sql, whereParams);
        return result.changes;
    },

    // Select records
    select(tableName, where = null, whereParams = []) {
        const db = getDB();
        let sql = `SELECT * FROM ${tableName}`;
        if (where) {
            sql += ` WHERE ${where}`;
        }
        return db.query(sql, whereParams);
    },

    // Select one record
    selectOne(tableName, where, whereParams = []) {
        const db = getDB();
        const sql = `SELECT * FROM ${tableName} WHERE ${where}`;
        return db.queryOne(sql, whereParams);
    },

    // Count records
    count(tableName, where = null, whereParams = []) {
        const db = getDB();
        let sql = `SELECT COUNT(*) as count FROM ${tableName}`;
        if (where) {
            sql += ` WHERE ${where}`;
        }
        const result = db.queryOne(sql, whereParams);
        return result.count;
    }
};

module.exports = {
    HttpServer,
    loadWasm,
    DB,
    getDB,
    dbHelpers
};
