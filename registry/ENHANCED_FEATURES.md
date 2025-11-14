# Jounce Registry Server - Enhanced Features

## Overview

The Jounce Package Registry Server has been enhanced with production-ready features including JWT authentication, rate limiting, and comprehensive owner management.

---

## New Features (v1.0.0)

### 1. JWT Authentication

**Previous**: Simple token hashing with SHA256
**Now**: Industry-standard JWT tokens with expiration

**Benefits**:
- Tokens expire after 30 days (configurable)
- More secure than simple hashing
- Tokens can be verified without database lookup
- Standard Bearer token format

**Example**:
```bash
# Register
curl -X POST http://localhost:4000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"secret123"}'

# Response includes JWT token
{"success":true,"token":"eyJhbGciOiJIUzI1NiIs...","username":"alice"}

# Use token in subsequent requests
curl -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIs..." \
  http://localhost:4000/api/v1/packages/my-package
```

---

### 2. Password-Based Authentication

**Previous**: Register generates random token (no password)
**Now**: Username/password authentication with bcrypt

**Benefits**:
- Users can login from multiple devices
- Passwords hashed with bcrypt (SALT_ROUNDS=10)
- Secure password storage
- Standard login/register flow

**Example**:
```bash
# Register with password
curl -X POST http://localhost:4000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "my-secure-password"
  }'

# Login to get new token
curl -X POST http://localhost:4000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "my-secure-password"
  }'
```

---

### 3. Rate Limiting

**Previous**: No rate limiting
**Now**: Express rate limiter protecting all endpoints

**Limits**:
- **General API**: 100 requests per 15 minutes
- **Publishing**: 10 publishes per hour
- **Per IP address**

**Benefits**:
- Prevents abuse and DDoS attacks
- Protects server resources
- Configurable limits

**Response when limited**:
```json
{
  "error": "Too many requests, please try again later"
}
```

---

### 4. Package Owner Management

**Previous**: No owner tracking
**Now**: Full owner management with access control

**Features**:
- First publisher becomes owner
- Owners can add/remove other owners
- Only owners can publish new versions
- Cannot remove last owner

**Endpoints**:
```bash
# List owners
GET /api/v1/packages/:name/owners

# Add owner (requires auth)
PUT /api/v1/packages/:name/owners
Body: {"username": "bob"}

# Remove owner (requires auth)
DELETE /api/v1/packages/:name/owners/:username
```

**Example**:
```bash
# Add collaborator
curl -X PUT http://localhost:4000/api/v1/packages/my-package/owners \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"username":"bob"}'

# Response
{"success":true,"message":"Added bob as owner of my-package","owners":["alice","bob"]}
```

---

### 5. Enhanced Security

**Improvements**:
1. **Username validation**: Must be lowercase alphanumeric with hyphens
2. **Package name validation**: Same format as usernames
3. **Duplicate prevention**: Cannot register same username/package twice
4. **JWT expiration**: Tokens expire and must be refreshed
5. **Bcrypt hashing**: Passwords hashed with 10 salt rounds
6. **Owner verification**: Access control on publish endpoint

**Validation Examples**:
```bash
# ✅ Valid username
username: "alice-smith"

# ❌ Invalid (uppercase)
username: "Alice"

# ❌ Invalid (special chars)
username: "alice_smith"

# ✅ Valid package name
name: "my-ui-lib"

# ❌ Invalid (uppercase)
name: "My-UI-Lib"
```

---

### 6. Better Error Messages

**Previous**: Generic error messages
**Now**: Specific, actionable error messages

**Examples**:
```json
// Duplicate package version
{"error":"Version already exists"}

// Invalid credentials
{"error":"Invalid credentials"}

// Not an owner
{"error":"You are not an owner of this package"}

// Token expired
{"error":"Token expired"}

// Rate limited
{"error":"Too many requests, please try again later"}
```

---

## API Changes

### Breaking Changes

**Endpoint URLs**: All endpoints now use `/api/v1/` prefix
- Old: `/api/register`
- New: `/api/v1/auth/register`

**Authentication Header**: Now uses standard Bearer format
- Old: `Authorization: abc123token`
- New: `Authorization: Bearer eyJhbGciOiJIUzI1NiIs...`

### New Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/login` | Login with username/password |
| GET | `/api/v1/packages/:name/owners` | List package owners |
| PUT | `/api/v1/packages/:name/owners` | Add package owner (auth) |
| DELETE | `/api/v1/packages/:name/owners/:username` | Remove owner (auth) |

---

## Configuration

### Environment Variables

Create a `.env` file:

```env
# Server Port
PORT=4000

# JWT Secret (REQUIRED in production!)
JWT_SECRET=your-super-secret-key-min-32-chars

# Node Environment
NODE_ENV=production
```

**IMPORTANT**: Change `JWT_SECRET` in production to a random string!

---

## Migration from Old Server

### For Registry Operators

1. **Backup old data**:
```bash
cp -r registry/auth registry/auth.backup
```

2. **Update dependencies**:
```bash
cd registry
npm install
```

3. **Configure environment**:
```bash
cp .env.example .env
# Edit .env and set JWT_SECRET
```

4. **Start new server**:
```bash
npm start
```

### For Package Authors

1. **Re-register** (old tokens won't work):
```bash
curl -X POST http://localhost:4000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username":"yourname",
    "email":"you@example.com",
    "password":"yourpassword"
  }'
```

2. **Update publish scripts** to use new token format:
```bash
# Old
curl -H "Authorization: abc123" ...

# New
curl -H "Authorization: Bearer eyJhbGci..." ...
```

---

## Testing

Run comprehensive test suite:

```bash
./test-registry.sh
```

Tests cover:
- Health check
- User registration
- User login
- Package publishing
- Package listing
- Search functionality
- Owner management
- Rate limiting

---

## Performance

**Benchmarks** (tested on MacBook Pro M1):
- **Health check**: ~5ms
- **Registration**: ~150ms (bcrypt hashing)
- **Login**: ~150ms (bcrypt verification)
- **Package listing**: ~20ms
- **Search**: ~50ms (file-based search)

**Rate limits**:
- Can handle 100 req/15min per IP for general endpoints
- Publishing limited to 10/hour to prevent abuse

---

## Security Considerations

### Production Deployment

1. **Change JWT_SECRET**: Use a strong random key
```bash
# Generate secure secret
node -e "console.log(require('crypto').randomBytes(64).toString('hex'))"
```

2. **Enable HTTPS**: Use reverse proxy (Nginx, Caddy)
```nginx
server {
    listen 443 ssl;
    server_name registry.jounce.dev;

    location / {
        proxy_pass http://localhost:4000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

3. **Database backups**: Regular backups of registry/ directory
```bash
# Daily backup
tar -czf registry-backup-$(date +%Y%m%d).tar.gz registry/
```

4. **Monitor rate limits**: Adjust based on your needs
```javascript
// In registry-server.js
const apiLimiter = rateLimit({
    windowMs: 15 * 60 * 1000,
    max: 200, // Increase if needed
});
```

---

## Future Enhancements

Planned features for v2.0:

- [ ] Email verification
- [ ] Password reset flow
- [ ] 2FA authentication
- [ ] API key management (multiple tokens per user)
- [ ] Package analytics dashboard
- [ ] Webhook notifications
- [ ] Package badges
- [ ] README rendering
- [ ] Package categories/tags
- [ ] Full-text search (Elasticsearch)
- [ ] S3 storage integration
- [ ] CDN integration
- [ ] Redis caching

---

## Support

For issues or questions:
- GitHub Issues: https://github.com/Jounce-lang/jounce-pre-production/issues
- Email: support@jounce.dev

---

**Version**: 1.0.0
**Last Updated**: November 2025
**Status**: Production Ready ✅
