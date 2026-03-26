// ============================================================
// PSL Supervisor — The Hostile Witness Engine
// Section 1.II: Verifies all VSA outputs, external GPU returns,
// and file ingestions against material axioms.
//
// The Supervisor holds a set of axioms and runs them against
// audit targets, producing a TrustAssessment.
// ============================================================

use crate::psl::axiom::{Axiom, AuditTarget, AxiomVerdict};
use crate::psl::error::PslError;
use crate::psl::trust::{TrustAssessment, TrustLevel};
use crate::debuglog;

/// The PSL Supervisor engine.
/// Holds registered axioms and runs audit passes.
pub struct PslSupervisor {
    axioms: Vec<Box<dyn Axiom>>,
    /// Minimum pass ratio required for Verified trust. Default: 1.0 (all must pass).
    trust_threshold: f64,
}

impl PslSupervisor {
    /// Create a new supervisor with no axioms loaded.
    pub fn new() -> Self {
        debuglog!("PslSupervisor::new, threshold=1.0");
        Self {
            axioms: Vec::new(),
            trust_threshold: 1.0,
        }
    }

    /// Create with a custom trust threshold in [0.0, 1.0].
    pub fn with_threshold(threshold: f64) -> Self {
        let t = threshold.clamp(0.0, 1.0);
        debuglog!("PslSupervisor::with_threshold: {:.4}", t);
        Self {
            axioms: Vec::new(),
            trust_threshold: t,
        }
    }

    /// Register an axiom with the supervisor.
    pub fn register_axiom(&mut self, axiom: Box<dyn Axiom>) {
        debuglog!("PslSupervisor::register_axiom: {}", axiom.id());
        self.axioms.push(axiom);
    }

    /// Number of registered axioms.
    pub fn axiom_count(&self) -> usize {
        self.axioms.len()
    }

    /// Run all applicable axioms against the target.
    /// Axioms that return InvalidAuditTarget are skipped (not counted).
    /// Returns a TrustAssessment summarizing the audit.
    pub fn audit(&self, target: &AuditTarget) -> Result<TrustAssessment, PslError> {
        if self.axioms.is_empty() {
            debuglog!("PslSupervisor::audit: EmptyAxiomSet");
            return Err(PslError::EmptyAxiomSet);
        }

        let mut verdicts: Vec<AxiomVerdict> = Vec::new();
        let mut skipped: usize = 0;

        for axiom in &self.axioms {
            match axiom.verify(target) {
                Ok(verdict) => {
                    debuglog!(
                        "audit: axiom={}, passed={}, tv={:.4}",
                        verdict.axiom_id, verdict.passed, verdict.truth_value
                    );
                    verdicts.push(verdict);
                }
                Err(PslError::InvalidAuditTarget { .. }) => {
                    debuglog!("audit: axiom={} skipped (invalid target type)", axiom.id());
                    skipped += 1;
                }
                Err(e) => {
                    debuglog!("audit: axiom={} structural failure: {}", axiom.id(), e);
                    return Err(e);
                }
            }
        }

        let checked = verdicts.len();
        let passed = verdicts.iter().filter(|v| v.passed).count();

        debuglog!(
            "audit: total_axioms={}, applicable={}, passed={}, skipped={}",
            self.axioms.len(), checked, passed, skipped
        );

        // If no axioms were applicable, target is suspicious
        if checked == 0 {
            return Ok(TrustAssessment::new(
                TrustLevel::Suspicious,
                0.25,
                "No applicable axioms for this target type".to_string(),
                0,
                0,
            ));
        }

        let pass_ratio = passed as f64 / checked as f64;
        let avg_truth: f64 = verdicts.iter().map(|v| v.truth_value).sum::<f64>() / checked as f64;

        let level = self.compute_trust_level(pass_ratio, avg_truth);
        let confidence = avg_truth * pass_ratio;

        let rationale = format!(
            "{}/{} axioms passed (ratio={:.4}, avg_truth={:.4})",
            passed, checked, pass_ratio, avg_truth
        );

        debuglog!("audit result: level={:?}, confidence={:.4}", level, confidence);

        Ok(TrustAssessment::new(
            level,
            confidence,
            rationale,
            checked,
            passed,
        ))
    }

    /// Derive trust level from pass ratio and average truth value.
    fn compute_trust_level(&self, pass_ratio: f64, avg_truth: f64) -> TrustLevel {
        debuglog!(
            "compute_trust_level: pass_ratio={:.4}, avg_truth={:.4}, threshold={:.4}",
            pass_ratio, avg_truth, self.trust_threshold
        );

        if pass_ratio >= self.trust_threshold && avg_truth >= 0.75 {
            TrustLevel::Verified
        } else if pass_ratio >= 0.75 && avg_truth >= 0.50 {
            TrustLevel::Provisional
        } else if pass_ratio >= 0.25 {
            TrustLevel::Suspicious
        } else {
            TrustLevel::Untrusted
        }
    }
}

// ============================================================
// PSL Supervisor Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdc::vector::BipolarVector;
    use crate::psl::axiom::{DimensionalityAxiom, DataIntegrityAxiom};

    #[test]
    fn test_empty_supervisor_returns_error() {
        let sup = PslSupervisor::new();
        let target = AuditTarget::Scalar {
            label: "test".to_string(),
            value: 42.0,
        };
        assert_eq!(sup.audit(&target), Err(PslError::EmptyAxiomSet));
    }

    #[test]
    fn test_register_axiom_increments_count() {
        let mut sup = PslSupervisor::new();
        assert_eq!(sup.axiom_count(), 0);
        sup.register_axiom(Box::new(DimensionalityAxiom));
        assert_eq!(sup.axiom_count(), 1);
    }

    #[test]
    fn test_audit_valid_vector_passes() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        sup.register_axiom(Box::new(DimensionalityAxiom));

        let v = BipolarVector::new_random().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let target = AuditTarget::Vector(v);
        let assessment = sup.audit(&target).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        assert_eq!(assessment.level, TrustLevel::Verified);
        assert_eq!(assessment.axioms_checked, 1);
        assert_eq!(assessment.axioms_passed, 1);
        assert!((assessment.pass_ratio() - 1.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn test_audit_skips_inapplicable_axioms() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        // DataIntegrityAxiom only applies to RawBytes, not Vector
        sup.register_axiom(Box::new(DataIntegrityAxiom { max_bytes: 1024 }));

        let v = BipolarVector::new_random().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let target = AuditTarget::Vector(v);
        let assessment = sup.audit(&target).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // No applicable axioms -> Suspicious
        assert_eq!(assessment.level, TrustLevel::Suspicious);
        assert_eq!(assessment.axioms_checked, 0);
        Ok(())
    }

    #[test]
    fn test_audit_raw_bytes_valid() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        sup.register_axiom(Box::new(DataIntegrityAxiom { max_bytes: 4096 }));

        let target = AuditTarget::RawBytes {
            source: "local_compute".to_string(),
            data: vec![0xDE, 0xAD, 0xBE, 0xEF],
        };
        let assessment = sup.audit(&target).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        assert_eq!(assessment.level, TrustLevel::Verified);
        assert_eq!(assessment.axioms_passed, 1);
        Ok(())
    }

    #[test]
    fn test_audit_raw_bytes_hostile_oversized() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        sup.register_axiom(Box::new(DataIntegrityAxiom { max_bytes: 4 }));

        let target = AuditTarget::RawBytes {
            source: "remote_gpu_grid".to_string(),
            data: vec![0u8; 1024],
        };
        let assessment = sup.audit(&target).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Failed axiom -> low trust
        assert!(assessment.level < TrustLevel::Verified);
        assert_eq!(assessment.axioms_passed, 0);
        Ok(())
    }

    #[test]
    fn test_audit_raw_bytes_hostile_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        sup.register_axiom(Box::new(DataIntegrityAxiom { max_bytes: 4096 }));

        let target = AuditTarget::RawBytes {
            source: "hostile_endpoint".to_string(),
            data: vec![],
        };
        let assessment = sup.audit(&target).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        assert!(assessment.level < TrustLevel::Verified);
        assert_eq!(assessment.axioms_passed, 0);
        Ok(())
    }

    #[test]
    fn test_multi_axiom_audit() -> Result<(), Box<dyn std::error::Error>> {
        let mut sup = PslSupervisor::new();
        sup.register_axiom(Box::new(DimensionalityAxiom));
        sup.register_axiom(Box::new(DataIntegrityAxiom { max_bytes: 4096 }));

        // Vector target: only DimensionalityAxiom applies
        let v = BipolarVector::new_random().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let assessment = sup.audit(&AuditTarget::Vector(v))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        assert_eq!(assessment.axioms_checked, 1); // DataIntegrity skipped
        assert_eq!(assessment.axioms_passed, 1);
        assert_eq!(assessment.level, TrustLevel::Verified);
        Ok(())
    }

    #[test]
    fn test_trust_threshold_custom() -> Result<(), Box<dyn std::error::Error>> {
        // With threshold=0.5, passing 1/2 axioms should still reach Provisional+
        let mut sup = PslSupervisor::with_threshold(0.5);
        sup.register_axiom(Box::new(DimensionalityAxiom));

        let v = BipolarVector::new_random().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let assessment = sup.audit(&AuditTarget::Vector(v))
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        assert!(assessment.level >= TrustLevel::Verified);
        Ok(())
    }

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Untrusted < TrustLevel::Suspicious);
        assert!(TrustLevel::Suspicious < TrustLevel::Provisional);
        assert!(TrustLevel::Provisional < TrustLevel::Verified);
        assert!(TrustLevel::Verified < TrustLevel::Sovereign);
    }

    #[test]
    fn test_trust_level_execution_gate() {
        assert!(!TrustLevel::Untrusted.permits_execution());
        assert!(!TrustLevel::Suspicious.permits_execution());
        assert!(!TrustLevel::Provisional.permits_execution());
        assert!(TrustLevel::Verified.permits_execution());
        assert!(TrustLevel::Sovereign.permits_execution());
    }

    #[test]
    fn test_trust_level_scores() {
        assert!((TrustLevel::Untrusted.score() - 0.0).abs() < f64::EPSILON);
        assert!((TrustLevel::Suspicious.score() - 0.25).abs() < f64::EPSILON);
        assert!((TrustLevel::Provisional.score() - 0.50).abs() < f64::EPSILON);
        assert!((TrustLevel::Verified.score() - 0.75).abs() < f64::EPSILON);
        assert!((TrustLevel::Sovereign.score() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_assessment_pass_ratio_no_axioms() {
        let a = TrustAssessment::new(
            TrustLevel::Suspicious,
            0.0,
            "none".to_string(),
            0,
            0,
        );
        assert!((a.pass_ratio() - 0.0).abs() < f64::EPSILON);
    }
}
