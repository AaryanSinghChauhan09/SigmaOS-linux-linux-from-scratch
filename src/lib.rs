#[derive(Debug, Clone, PartialEq)]
pub enum BuildStage {
    CrossToolchain,
    TemporaryTools,
    FinalSystem,
}

#[derive(Debug, Clone)]
pub struct LfsPackageRecipe {
    pub name: String,
    pub version: String,
    pub sha256: String,
    pub stage: BuildStage,
}

pub struct LfsBuildPipeline {
    pub recipes: Vec<LfsPackageRecipe>,
}

impl Default for LfsBuildPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl LfsBuildPipeline {
    pub fn new() -> Self {
        Self { recipes: Vec::new() }
    }

    pub fn register(&mut self, recipe: LfsPackageRecipe) {
        self.recipes.push(recipe);
    }

    pub fn count_stage(&self, stage: BuildStage) -> usize {
        self.recipes.iter().filter(|r| r.stage == stage).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfs_pipeline() {
        // Arrange
        let mut pipeline = LfsBuildPipeline::new();
        // Act
        pipeline.register(LfsPackageRecipe {
            name: "binutils".into(),
            version: "2.41".into(),
            sha256: "deadbeef".into(),
            stage: BuildStage::CrossToolchain,
        });
        // Assert
        assert_eq!(pipeline.count_stage(BuildStage::CrossToolchain), 1);
    }
}
