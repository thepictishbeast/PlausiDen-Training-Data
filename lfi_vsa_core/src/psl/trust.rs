// ============================================================
// CARTA Trust Assessment — Continuous Adaptive Risk & Trust
// Section 2: Zero-Trust and Assume Breach protocols.
//
// Trust is never assumed. Every datum enters at Untrusted and
// must be promoted through verified axiom gates.
// ============================================================

use crate::debuglog;

/// Discrete trust levels under the CARTA model.
/// Progression: Untrusted -> Suspicious -> Provisional -> Verified -> Sovereign.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Default state. Hostile until proven otherwise.
    Untrusted = 0,
    /// Failed partial checks but not conclusively hostile.
    Suspicious = 1,
    /// Passed basic axioms, pending deeper verification.
    Provisional = 2,
    /// Passed all applicable axioms at the current gate.
    Verified = 3,
    /// Originates from the sovereign local compute chain.
    Sovereign = 4,
}

impl TrustLevel {
    /// Whether this trust level permits execution or integration.
    pub fn permits_execution(&self) -> bool {
        debuglog!("TrustLevel::permits_execution: level={:?}", self);
        matches!(self, TrustLevel::Verified | TrustLevel::Sovereign)
    }

    /// Whether this trust level requires further audit.
    pub fn requires_audit(&self) -> bool {
        debuglog!("TrustLevel::requires_audit: level={:?}", self);
        matches!(
            self,
            TrustLevel::Untrusted | TrustLevel::Suspicious | TrustLevel::Provisional
        )
    }

    /// Numeric score for threshold comparisons. Range: [0.0, 1.0].
    pub fn score(&self) -> f64 {
        let s = match self {
            TrustLevel::Untrusted => 0.0,
            TrustLevel::Suspicious => 0.25,
            TrustLevel::Provisional => 0.50,
            TrustLevel::Verified => 0.75,
            TrustLevel::Sovereign => 1.0,
        };
        debuglog!("TrustLevel::score: {:?} -> {:.2}", self, s);
        s
    }
}

/// Audit outcome for a single datum or computation result.
#[derive(Debug, Clone, PartialEq)]
pub struct TrustAssessment {
    /// The assigned trust level after audit.
    pub level: TrustLevel,
    /// Confidence in the assessment. Range: [0.0, 1.0].
    pub confidence: f64,
    /// Human-readable rationale for the assessment.
    pub rationale: String,
    /// Number of axioms that were checked.
    pub axioms_checked: usize,
    /// Number of axioms that passed.
    pub axioms_passed: usize,
}

impl TrustAssessment {
    /// Create a new assessment. Confidence is clamped to [0.0, 1.0].
    pub fn new(
        level: TrustLevel,
        confidence: f64,
        rationale: String,
        axioms_checked: usize,
        axioms_passed: usize,
    ) -> Self {
        let clamped = confidence.clamp(0.0, 1.0);
        debuglog!(
            "TrustAssessment::new: level={:?}, confidence={:.4}, checked={}, passed={}",
            level, clamped, axioms_checked, axioms_passed
        );
        Self {
            level,
            confidence: clamped,
            rationale,
            axioms_checked,
            axioms_passed,
        }
    }

    /// Pass ratio: axioms_passed / axioms_checked. Returns 0.0 if no axioms.
    pub fn pass_ratio(&self) -> f64 {
        if self.axioms_checked == 0 {
            debuglog!("TrustAssessment::pass_ratio: no axioms checked");
            return 0.0;
        }
        let ratio = self.axioms_passed as f64 / self.axioms_checked as f64;
        debuglog!("TrustAssessment::pass_ratio: {:.4}", ratio);
        ratio
    }
}
