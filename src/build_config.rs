// Build Configuration for Jounce Compiler
// Phase 17: Production Build Optimizations

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    None,       // Level 0: No optimization (dev builds)
    Basic,      // Level 1: Minification only
    Standard,   // Level 2: DCE + Minification
    Aggressive, // Level 3: DCE + Tree Shaking + Minification + Code Splitting
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub optimization_level: OptimizationLevel,
    pub minify: bool,
    pub source_maps: bool,
    pub dead_code_elimination: bool,
    pub tree_shaking: bool,
    pub code_splitting: bool,
    pub analyze_bundle: bool,
    pub target_env: TargetEnvironment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TargetEnvironment {
    Development,
    Production,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl BuildConfig {
    /// Development build configuration (fast, unoptimized)
    pub fn development() -> Self {
        Self {
            optimization_level: OptimizationLevel::None,
            minify: false,
            source_maps: true,
            dead_code_elimination: false,
            tree_shaking: false,
            code_splitting: false,
            analyze_bundle: false,
            target_env: TargetEnvironment::Development,
        }
    }

    /// Production build configuration (slow, fully optimized)
    pub fn production() -> Self {
        Self {
            optimization_level: OptimizationLevel::Aggressive,
            minify: true,
            source_maps: false,
            dead_code_elimination: true,
            tree_shaking: true,
            code_splitting: true,
            analyze_bundle: false,
            target_env: TargetEnvironment::Production,
        }
    }

    /// Custom optimization level
    pub fn with_optimization_level(level: OptimizationLevel) -> Self {
        let mut config = match level {
            OptimizationLevel::None => Self::development(),
            OptimizationLevel::Basic => Self {
                optimization_level: OptimizationLevel::Basic,
                minify: true,
                source_maps: true,
                dead_code_elimination: false,
                tree_shaking: false,
                code_splitting: false,
                analyze_bundle: false,
                target_env: TargetEnvironment::Production,
            },
            OptimizationLevel::Standard => Self {
                optimization_level: OptimizationLevel::Standard,
                minify: true,
                source_maps: false,
                dead_code_elimination: true,
                tree_shaking: false,
                code_splitting: false,
                analyze_bundle: false,
                target_env: TargetEnvironment::Production,
            },
            OptimizationLevel::Aggressive => Self::production(),
        };
        config.optimization_level = level;
        config
    }

    /// Enable source maps
    pub fn with_source_maps(mut self) -> Self {
        self.source_maps = true;
        self
    }

    /// Enable bundle analysis
    pub fn with_analysis(mut self) -> Self {
        self.analyze_bundle = true;
        self
    }

    /// Enable only minification (useful for debugging optimization issues)
    pub fn minify_only(mut self) -> Self {
        self.minify = true;
        self.dead_code_elimination = false;
        self.tree_shaking = false;
        self.code_splitting = false;
        self
    }

    /// Check if any optimization is enabled
    pub fn is_optimized(&self) -> bool {
        self.optimization_level != OptimizationLevel::None
    }

    /// Get optimization summary for logging
    pub fn summary(&self) -> String {
        let mut features = Vec::new();

        if self.minify {
            features.push("minify");
        }
        if self.dead_code_elimination {
            features.push("DCE");
        }
        if self.tree_shaking {
            features.push("tree-shaking");
        }
        if self.code_splitting {
            features.push("code-splitting");
        }
        if self.source_maps {
            features.push("sourcemaps");
        }

        if features.is_empty() {
            "No optimizations".to_string()
        } else {
            format!("Optimizations: {}", features.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_development_config() {
        let config = BuildConfig::development();
        assert_eq!(config.optimization_level, OptimizationLevel::None);
        assert!(!config.minify);
        assert!(config.source_maps);
        assert!(!config.dead_code_elimination);
    }

    #[test]
    fn test_production_config() {
        let config = BuildConfig::production();
        assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
        assert!(config.minify);
        assert!(!config.source_maps);
        assert!(config.dead_code_elimination);
        assert!(config.tree_shaking);
    }

    #[test]
    fn test_with_source_maps() {
        let config = BuildConfig::production().with_source_maps();
        assert!(config.source_maps);
        assert!(config.minify);  // Still has other production features
    }

    #[test]
    fn test_optimization_levels() {
        let none = BuildConfig::with_optimization_level(OptimizationLevel::None);
        assert!(!none.is_optimized());

        let basic = BuildConfig::with_optimization_level(OptimizationLevel::Basic);
        assert!(basic.is_optimized());
        assert!(basic.minify);
        assert!(!basic.dead_code_elimination);

        let standard = BuildConfig::with_optimization_level(OptimizationLevel::Standard);
        assert!(standard.dead_code_elimination);
        assert!(!standard.tree_shaking);

        let aggressive = BuildConfig::with_optimization_level(OptimizationLevel::Aggressive);
        assert!(aggressive.tree_shaking);
        assert!(aggressive.code_splitting);
    }
}
