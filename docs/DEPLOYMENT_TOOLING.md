# Jounce Deployment Tooling Design

**Version**: v0.11.0 (Phase 17)
**Last Updated**: November 1, 2025

---

## 🎯 Goal

**One-command deployment to major platforms**: Vercel, Fly.io, Docker, AWS, etc.

```bash
jnc deploy         # Auto-detect platform
jnc deploy vercel  # Explicit platform
```

---

## 🏗️ Architecture

```
jnc deploy
    ↓
Platform Detection
    ├─ Check deploy.config.jnc
    ├─ Check environment variables
    ├─ Check git remotes
    └─ Ask user if ambiguous
    ↓
Platform Adapter
    ├─ Vercel Adapter
    ├─ Fly.io Adapter
    ├─ Docker Adapter
    ├─ AWS Adapter
    └─ Custom Adapter
    ↓
Build & Deploy
    ├─ Run production build
    ├─ Generate platform config
    ├─ Upload files
    ├─ Set environment variables
    └─ Trigger deployment
    ↓
Deployment URL
```

---

## 📦 Supported Platforms

### 1. Vercel
```bash
jnc deploy vercel
```

**Auto-generates**:
- `vercel.json` configuration
- Build command
- Output directory
- Environment variables

**Features**:
- Automatic HTTPS
- CDN distribution
- Preview deployments
- Custom domains

---

### 2. Fly.io
```bash
jnc deploy fly
```

**Auto-generates**:
- `fly.toml` configuration
- Dockerfile
- Health checks
- Scaling config

**Features**:
- Global edge deployment
- Auto-scaling
- Built-in database
- Monitoring

---

### 3. Docker
```bash
jnc deploy docker
```

**Auto-generates**:
- `Dockerfile`
- `.dockerignore`
- `docker-compose.yml`

**Output**:
- Docker image ready to deploy anywhere
- Can push to Docker Hub, AWS ECR, etc.

---

### 4. AWS (Coming Soon)
```bash
jnc deploy aws
```

**Auto-generates**:
- CloudFormation template
- Lambda functions
- API Gateway config
- S3 bucket for static assets

---

## ⚙️ Configuration File

### `deploy.config.jnc`

```jounce
// Deployment configuration
export let deployConfig = {
    // Platform to deploy to
    platform: "vercel",  // or "fly", "docker", "aws"

    // Build configuration
    build: {
        command: "jnc build --release",
        output_dir: "dist",
        node_version: "20"
    },

    // Environment variables
    env: {
        production: {
            API_URL: "https://api.example.com",
            DATABASE_URL: "${DATABASE_URL}",  // From platform secrets
            JWT_SECRET: "${JWT_SECRET}"
        },
        preview: {
            API_URL: "https://api-staging.example.com"
        }
    },

    // Platform-specific configuration
    vercel: {
        regions: ["sfo1", "iad1"],
        framework: "jounce"
    },

    fly: {
        app_name: "my-jounce-app",
        region: "sjc",
        vm_size: "shared-cpu-1x"
    },

    docker: {
        base_image: "node:20-alpine",
        port: 3000
    }
};
```

---

## 🔧 CLI Implementation

### Basic Command Structure

```rust
// src/cli/deploy.rs

pub struct DeployCommand {
    pub platform: Option<Platform>,
    pub config_file: Option<String>,
    pub env: Environment,
    pub dry_run: bool,
}

pub enum Platform {
    Vercel,
    Fly,
    Docker,
    AWS,
    Custom(String),
}

pub enum Environment {
    Production,
    Preview,
    Development,
}

impl DeployCommand {
    pub async fn execute(&self) -> Result<DeploymentResult, DeployError> {
        // 1. Load configuration
        let config = self.load_config()?;

        // 2. Detect or use specified platform
        let platform = self.detect_platform(&config)?;

        // 3. Create platform adapter
        let adapter = platform.create_adapter(&config)?;

        // 4. Build project
        println!("Building for production...");
        adapter.build()?;

        // 5. Deploy
        println!("Deploying to {}...", platform.name());
        let result = adapter.deploy().await?;

        // 6. Show result
        println!("✅ Deployed successfully!");
        println!("🌐 URL: {}", result.url);

        Ok(result)
    }

    fn detect_platform(&self, config: &DeployConfig) -> Result<Platform, DeployError> {
        // Check if explicitly specified
        if let Some(platform) = &self.platform {
            return Ok(platform.clone());
        }

        // Check config file
        if let Some(platform) = &config.platform {
            return Ok(platform.clone());
        }

        // Check environment
        if std::env::var("VERCEL").is_ok() {
            return Ok(Platform::Vercel);
        }
        if std::env::var("FLY_APP_NAME").is_ok() {
            return Ok(Platform::Fly);
        }

        // Ask user
        let platform = prompt_platform()?;
        Ok(platform)
    }
}
```

---

## 📋 Platform Adapters

### Vercel Adapter

```rust
pub struct VercelAdapter {
    config: DeployConfig,
}

impl VercelAdapter {
    pub fn build(&self) -> Result<(), DeployError> {
        // Run production build
        Command::new("jnc")
            .args(&["build", "--release"])
            .status()?;

        // Generate vercel.json
        self.generate_vercel_config()?;

        Ok(())
    }

    pub async fn deploy(&self) -> Result<DeploymentResult, DeployError> {
        // Use Vercel CLI
        let output = Command::new("vercel")
            .args(&["--prod", "--yes"])
            .output()?;

        let url = String::from_utf8(output.stdout)?
            .lines()
            .find(|line| line.starts_with("https://"))
            .unwrap_or_default()
            .to_string();

        Ok(DeploymentResult {
            url,
            platform: "Vercel".to_string(),
            environment: "production".to_string(),
        })
    }

    fn generate_vercel_config(&self) -> Result<(), DeployError> {
        let config = json!({
            "buildCommand": "jnc build --release",
            "outputDirectory": "dist",
            "framework": "jounce",
            "env": self.config.env.get("production")
        });

        std::fs::write("vercel.json", serde_json::to_string_pretty(&config)?)?;
        Ok(())
    }
}
```

---

## 🔐 Environment Variables

### Setting Secrets

```bash
# Vercel
jnc deploy vercel --set-env JWT_SECRET=xxx

# Fly.io
jnc deploy fly --set-secret DATABASE_URL=xxx

# Docker
jnc deploy docker --env-file .env.production
```

### Accessing in Code

```jounce
// Environment variables are automatically available
let jwtSecret = process.env.JWT_SECRET;
let dbUrl = process.env.DATABASE_URL;
```

---

## 🚀 Deployment Workflow

### 1. Initial Setup
```bash
# Initialize deploy configuration
jnc deploy init

# Choose platform (interactive)
# Generates deploy.config.jnc
```

### 2. First Deployment
```bash
# Deploy to production
jnc deploy

# Follow prompts to set up platform account
# Deployment URL returned
```

### 3. Subsequent Deployments
```bash
# Just run deploy command
jnc deploy

# Automatically uses saved configuration
```

---

## 📊 Deployment Status

```bash
# Check deployment status
jnc deploy status

# Output:
# Platform: Vercel
# Environment: production
# URL: https://my-app.vercel.app
# Status: Live ✅
# Last deployed: 2025-11-01 12:34:56
# Build time: 8.2s
```

---

## 🎯 Implementation Phases

### Phase 1: Vercel Support (Week 1)
- [ ] Vercel adapter implementation
- [ ] Auto-generate vercel.json
- [ ] Environment variable management
- [ ] Test with real deployments

### Phase 2: Fly.io Support (Week 2)
- [ ] Fly.io adapter implementation
- [ ] Auto-generate fly.toml
- [ ] Dockerfile generation
- [ ] Health check configuration

### Phase 3: Docker Support (Week 2)
- [ ] Docker adapter implementation
- [ ] Multi-stage Dockerfile
- [ ] docker-compose.yml generation
- [ ] Image optimization

### Phase 4: CLI Polish (Week 3)
- [ ] Interactive platform selection
- [ ] Deployment status tracking
- [ ] Rollback functionality
- [ ] Deployment logs

---

**Last Updated**: November 1, 2025
**Phase**: 17 - Security & Production Features
**Sprint**: 17.3 - Deployment Tooling
