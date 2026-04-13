// ============================================================
// Weight Manager — Persistent Intelligence Backup & Restore
//
// LFI's "intelligence" = VSA vectors, not neural network weights.
// This module saves and restores the complete learned state:
//   - Knowledge store (concepts, facts, mastery levels)
//   - PSL feedback rejections (avoidance patterns)
//   - Planner solution patterns
//   - Analogy library
// ============================================================

use crate::hdc::error::HdcError;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

/// A complete intelligence checkpoint.
#[derive(Serialize, Deserialize)]
pub struct IntelligenceCheckpoint {
    pub version: u32,
    pub timestamp: String,
    pub description: String,
    pub episodes_completed: u64,
    pub concepts_count: usize,
    pub rejection_patterns: usize,
    pub solution_patterns: usize,
    pub knowledge_store_json: String,
    pub integrity_hash: String,
}

impl IntelligenceCheckpoint {
    pub fn capture(
        knowledge_json: &str, episodes: u64, concepts: usize,
        rejections: usize, solutions: usize, description: &str,
    ) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let integrity_hash = Self::compute_hash(knowledge_json, &timestamp);
        Self {
            version: 1, timestamp, description: description.into(),
            episodes_completed: episodes, concepts_count: concepts,
            rejection_patterns: rejections, solution_patterns: solutions,
            knowledge_store_json: knowledge_json.into(), integrity_hash,
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), HdcError> {
        debuglog!("IntelligenceCheckpoint::save: {}", path.display());
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| HdcError::PersistenceFailure {
                detail: format!("mkdir failed: {}", e),
            })?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| HdcError::PersistenceFailure {
            detail: format!("serialize failed: {}", e),
        })?;
        std::fs::write(path, json).map_err(|e| HdcError::PersistenceFailure {
            detail: format!("write failed: {}", e),
        })
    }

    pub fn load(path: &Path) -> Result<Self, HdcError> {
        debuglog!("IntelligenceCheckpoint::load: {}", path.display());
        let json = std::fs::read_to_string(path).map_err(|e| HdcError::PersistenceFailure {
            detail: format!("read failed: {}", e),
        })?;
        let cp: Self = serde_json::from_str(&json).map_err(|e| HdcError::PersistenceFailure {
            detail: format!("deserialize failed: {}", e),
        })?;
        let expected = Self::compute_hash(&cp.knowledge_store_json, &cp.timestamp);
        if expected != cp.integrity_hash {
            return Err(HdcError::PersistenceFailure {
                detail: "Integrity check FAILED — data corrupted".into(),
            });
        }
        Ok(cp)
    }

    pub fn default_dir() -> PathBuf { PathBuf::from("/root/.lfi/checkpoints") }

    pub fn generate_filename() -> String {
        format!("checkpoint_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    }

    fn compute_hash(data: &str, timestamp: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        data.hash(&mut h);
        timestamp.hash(&mut h);
        format!("{:016x}", h.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_and_save_load() -> Result<(), HdcError> {
        let cp = IntelligenceCheckpoint::capture(
            r#"{"concepts":["rust"]}"#, 100, 50, 10, 6, "Test",
        );
        let path = PathBuf::from("/tmp/lfi_test_cp.json");
        cp.save(&path)?;
        let loaded = IntelligenceCheckpoint::load(&path)?;
        assert_eq!(loaded.episodes_completed, 100);
        assert_eq!(loaded.concepts_count, 50);
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    #[test]
    fn test_integrity_catches_corruption() -> Result<(), HdcError> {
        let cp = IntelligenceCheckpoint::capture(r#"{"clean":true}"#, 10, 5, 1, 1, "Integrity");
        let path = PathBuf::from("/tmp/lfi_test_corrupt.json");
        cp.save(&path)?;
        let mut json = std::fs::read_to_string(&path).unwrap();
        json = json.replace("clean", "CORRUPT");
        std::fs::write(&path, json).unwrap();
        assert!(IntelligenceCheckpoint::load(&path).is_err());
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    #[test]
    fn test_generate_filename() {
        let name = IntelligenceCheckpoint::generate_filename();
        assert!(name.starts_with("checkpoint_") && name.ends_with(".json"));
    }
}
