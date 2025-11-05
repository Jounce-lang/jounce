# Production-Ready Multi-User CRM

**A complete sales pipeline management system built entirely in Jounce.**

Zero external dependencies. Zero npm packages. Just Jounce's native database and authentication.

---

## What Is This?

A **production-ready Customer Relationship Management (CRM)** system for sales teams, built to demonstrate Jounce's enterprise capabilities:

- Multi-user authentication with role-based access control
- PostgreSQL database with proper indexing and transactions
- Real-time metrics and analytics
- Secure JWT-based sessions
- Auto-generated REST API from server functions
- Fully type-safe client-server communication

**Built with**: 100% Jounce (no Express, no Prisma, no Passport.js, no external auth libraries)

---

## Features

### Authentication & Security
- User registration with bcrypt password hashing
- JWT token-based authentication
- Role-based access control (sales_rep, manager)
- Session persistence with localStorage
- Automatic token expiration
- SQL injection protection via parameterized queries

### Deal Management
- Create, update, and track sales deals
- Mark deals as won/lost with audit trail
- Filter deals by status (open, won, lost)
- Ownership validation (users can only edit their deals)
- Managers can view all team deals

### Real-Time Metrics
- Closing ratio (won deals / total deals)
- Total pipeline value
- Won deal count and revenue
- Active deal tracking
- Automatically updates on deal changes

### Multi-User Support
- Each user sees only their own deals (sales reps)
- Managers see all team deals and aggregated metrics
- Activity logging tracks who changed what and when
- Proper database isolation and permissions

---

## Quick Start

### Prerequisites

1. **Jounce compiler** (you already have this!)
2. **PostgreSQL database** (choose one):
   - Railway (15 min setup, $5/mo) - **Recommended for beginners**
   - Supabase (20 min setup, free tier available)
   - Local PostgreSQL (for development)

### Step 1: Set Up Database

Follow **[POSTGRESQL_SETUP.md](./POSTGRESQL_SETUP.md)** for detailed instructions.

**Quick version** (Railway):
```bash
# Install Railway CLI
npm install -g @railway/cli

# Login and create project
railway login
railway init

# Add PostgreSQL
railway add postgresql

# Get connection string
railway variables
# Copy the DATABASE_URL value
```

### Step 2: Create Database Tables

Connect to your database and run this SQL:

```sql
-- Users table
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT DEFAULT 'sales_rep',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP
);

-- Deals table
CREATE TABLE deals (
    id BIGSERIAL PRIMARY KEY,
    company TEXT NOT NULL,
    value INTEGER NOT NULL,
    status TEXT NOT NULL,
    contact TEXT NOT NULL,
    owner_id BIGINT REFERENCES users(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Activity log
CREATE TABLE deal_activities (
    id BIGSERIAL PRIMARY KEY,
    deal_id BIGINT REFERENCES deals(id),
    user_id BIGINT REFERENCES users(id),
    action TEXT NOT NULL,
    old_status TEXT,
    new_status TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_deals_owner ON deals(owner_id);
CREATE INDEX idx_deals_status ON deals(status);
CREATE INDEX idx_activities_deal ON deal_activities(deal_id);
```

### Step 3: Configure Environment Variables

Create `.env` file in your project root:

```bash
DATABASE_URL=postgres://user:password@host:5432/database
JWT_SECRET=your_super_secret_key_minimum_32_characters_long
PORT=3000
NODE_ENV=production
```

**Generate a secure JWT secret:**
```bash
openssl rand -hex 32
```

### Step 4: Compile the Jounce App

```bash
cargo run --release -- compile examples/apps/36-crm-production/main.jnc
```

This generates:
- `dist/client.js` - Frontend application
- `dist/server.js` - Backend API server
- `dist/index.html` - Entry point
- `dist/styles.css` - Compiled styles

### Step 5: Run Locally

```bash
cd dist
node server.js
```

Open http://localhost:3000

You should see the login page!

### Step 6: Create Your First User

Since this is your first run, register a new account:

1. Click "Register" on the login page
2. Enter your email, name, and password
3. Login with your credentials
4. Start creating deals!

**To create a manager account**, update the database:
```sql
UPDATE users SET role = 'manager' WHERE email = 'your@email.com';
```

---

## Deployment

### Option 1: Railway (Recommended)

**Time**: 5 minutes
**Cost**: $5/month

```bash
# From dist/ directory
railway up

# Set environment variables in Railway dashboard
railway variables set JWT_SECRET=your_generated_secret
railway variables set DATABASE_URL=postgres://...
```

Your app will be live at `https://your-app.up.railway.app`

### Option 2: Vercel + Supabase

**Time**: 10 minutes
**Cost**: Free tier available

```bash
# Install Vercel CLI
npm install -g vercel

# Deploy from dist/
cd dist
vercel

# Set environment variables
vercel env add DATABASE_URL
vercel env add JWT_SECRET
```

### Option 3: AWS / DigitalOcean / Your VPS

**Time**: 30 minutes
**Cost**: $5-30/month

1. Copy `dist/` folder to server
2. Install Node.js 18+
3. Set environment variables
4. Run with PM2 or systemd:

```bash
npm install -g pm2
pm2 start server.js --name crm
pm2 save
```

---

## Architecture

### Technology Stack

| Component | Technology | Why |
|-----------|-----------|-----|
| **Frontend** | Jounce (compiles to vanilla JS) | Reactive UI, zero dependencies |
| **Backend** | Jounce server functions (Node.js) | Auto-generated REST API |
| **Database** | PostgreSQL | Production-grade, ACID compliance |
| **Auth** | Jounce auth stdlib (bcrypt + JWT) | Built-in, no external libraries |
| **ORM** | Jounce db stdlib | Type-safe queries, zero config |
| **HTTP** | Node.js http module | Generated by Jounce compiler |

### How It Works

1. **You write**: `server fn login(email: String, password: String)`
2. **Jounce generates**:
   - Client stub: `fetch('/api/login', { method: 'POST', ... })`
   - Server handler: Express-like route with parameter validation
   - Type-safe communication layer

See **[RPC_EXPLAINED.md](./RPC_EXPLAINED.md)** for detailed explanation.

### Database Schema

```
users (id, email, name, password_hash, role, created_at, last_login)
  ↓
deals (id, company, value, status, contact, owner_id, created_at, updated_at)
  ↓
deal_activities (id, deal_id, user_id, action, old_status, new_status, timestamp)
```

**Relationships**:
- Each deal belongs to one user (owner_id → users.id)
- Each activity belongs to one deal and one user
- Managers can view all deals, sales reps see only their own

### Security Features

✅ **Password Hashing**: bcrypt with 12 rounds
✅ **SQL Injection Protection**: Parameterized queries (`$1`, `$2`)
✅ **XSS Protection**: Automatic HTML escaping in JSX
✅ **CSRF Protection**: JWT tokens (not cookies)
✅ **Role-Based Access**: `@auth` annotation + ownership checks
✅ **Session Expiration**: 24-hour JWT lifetime

---

## Multi-User Example

See **[multi_user_example.md](./multi_user_example.md)** for a detailed walkthrough of 4 sales reps using the system simultaneously, including:

- Alice logging in and seeing her 2 deals
- Bob, Carol, and Dave each seeing their own data
- Alice marking a deal as won → everyone's metrics update
- Bob trying to edit Alice's deal → rejected by server
- Carol (manager) creating a new deal → Dave gets notified

---

## Cost Breakdown

### Minimal Setup (1-10 users)
- **Railway**: $5/month (PostgreSQL + hosting)
- **Supabase Free Tier**: $0/month (500MB database)
- **Total**: $0-5/month

### Small Team (10-50 users)
- **Railway Pro**: $10/month
- **Supabase Pro**: $25/month (8GB database)
- **Total**: $10-25/month

### Medium Business (50-200 users)
- **AWS RDS PostgreSQL**: $30/month
- **EC2 t3.small**: $15/month
- **Total**: ~$45/month

---

## Documentation

- **[POSTGRESQL_SETUP.md](./POSTGRESQL_SETUP.md)** - Database setup guide (Railway, Supabase, local)
- **[RPC_EXPLAINED.md](./RPC_EXPLAINED.md)** - How auto-generated API works
- **[multi_user_example.md](./multi_user_example.md)** - Multi-user scenario walkthrough

---

## What Makes This Special?

### Traditional Stack (React + Express)
```
npm install express pg bcrypt jsonwebtoken passport cors body-parser
npm install prisma @prisma/client
npm install react react-dom
npm install --save-dev @types/express @types/bcrypt ...

// Write Express routes
app.post('/api/login', async (req, res) => { ... })
app.get('/api/deals', auth, async (req, res) => { ... })
app.post('/api/deals/:id/won', auth, async (req, res) => { ... })

// Write React components
// Write API client functions
// Configure Prisma
// Set up auth middleware
// Write TypeScript types (and keep them in sync!)

Total: ~2,000 lines of code, 20+ npm packages
```

### Jounce Stack (This App)
```
// No npm install needed!
// No package.json!
// No separate API files!

use jounce::db::*;
use jounce::auth::*;

server fn login(email: String, password: String) -> Result<AuthToken, String> {
    // Database + auth built-in!
}

component CRMDashboard() {
    // Call server function like it's local!
    let result = login(email.value, password.value).await;
}

Total: 500 lines of code, 0 npm packages
```

**Result**: 75% less code, 100% fewer dependencies, 10x faster setup.

---

## Troubleshooting

### "Connection timeout" error
- Check `DATABASE_URL` is correct
- Verify database is running (check Railway/Supabase dashboard)
- Ensure SSL is enabled (Railway/Supabase require it)

### "Unauthorized" errors
- Check `JWT_SECRET` is set in environment
- Ensure secret is at least 32 characters
- Verify token is being sent in requests

### "relation does not exist" error
- Run the CREATE TABLE statements from Step 2
- Verify you're connected to the correct database
- Check schema name if using PostgreSQL schemas

### Deals not appearing
- Check browser console for errors
- Verify `owner_id` matches your user ID
- Ensure deals table has data: `SELECT * FROM deals;`

### Performance issues
- Add indexes (already included in setup SQL)
- Monitor query performance: `EXPLAIN ANALYZE SELECT ...`
- Increase connection pool size if needed

---

## Next Steps

### Add Features
- Search and filtering
- Deal notes and comments
- Email notifications
- Export to CSV
- Sales forecasting
- Team leaderboards

### Integrate Services
- Stripe for payment tracking
- SendGrid for email
- Twilio for SMS notifications
- Slack for team alerts

### Scale Up
- Add caching (Redis)
- Implement WebSockets for real-time updates
- Add full-text search (PostgreSQL tsvector)
- Set up database replication

---

## Support

**Found a bug?** Open an issue at https://github.com/jounce/jounce/issues

**Need help?** Check the Jounce documentation at https://jounce.dev/docs

**Want to contribute?** Pull requests welcome!

---

## License

MIT License - feel free to use this as a starting point for your own CRM!

---

**Built with Jounce** - The language that makes full-stack development actually fun.

**Compile once. Deploy everywhere. Zero dependencies.**
