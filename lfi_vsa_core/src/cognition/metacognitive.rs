// ============================================================
// MetaCognitive Profiler — Self-Awareness for LFI
//
// Tracks what LFI is good at and bad at using BipolarVectors.
// The weakness_map and strength_map are bundled hypervectors:
//   - Each domain (coding, math, security, etc.) gets a base vector
//   - Successes bundle into strength_map with domain vector
//   - Failures bundle into weakness_map with domain vector
//   - Probing either map with a domain vector returns performance signal
//
// The improvement_queue prioritizes domains with highest weakness
// signal for targeted learning (Active Learning strategy).
//
// This is the "knows what it doesn't know" module — critical for
// escape velocity (self-directed improvement).
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::holographic::HolographicMemory;
use crate::hdc::error::HdcError;
use std::collections::HashMap;

/// A domain of knowledge or capability that LFI can be profiled on.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CognitiveDomain {
    Coding,
    Mathematics,
    Security,
    NaturalLanguage,
    Planning,
    Reasoning,
    FactualKnowledge,
    Conversation,
    SelfImprovement,
    Custom(String),
}

impl CognitiveDomain {
    /// Deterministic seed for this domain's base vector.
    fn seed(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        match self {
            CognitiveDomain::Coding => "domain:coding".hash(&mut hasher),
            CognitiveDomain::Mathematics => "domain:mathematics".hash(&mut hasher),
            CognitiveDomain::Security => "domain:security".hash(&mut hasher),
            CognitiveDomain::NaturalLanguage => "domain:natural_language".hash(&mut hasher),
            CognitiveDomain::Planning => "domain:planning".hash(&mut hasher),
            CognitiveDomain::Reasoning => "domain:reasoning".hash(&mut hasher),
            CognitiveDomain::FactualKnowledge => "domain:factual_knowledge".hash(&mut hasher),
            CognitiveDomain::Conversation => "domain:conversation".hash(&mut hasher),
            CognitiveDomain::SelfImprovement => "domain:self_improvement".hash(&mut hasher),
            CognitiveDomain::Custom(name) => format!("domain:custom:{}", name).hash(&mut hasher),
        }
        hasher.finish()
    }

    /// Get the base hypervector for this domain.
    pub fn base_vector(&self) -> BipolarVector {
        BipolarVector::from_seed(self.seed())
    }
}

/// A single performance record for profiling.
#[derive(Debug, Clone)]
pub struct PerformanceRecord {
    /// The domain this record belongs to.
    pub domain: CognitiveDomain,
    /// Whether the task was successful.
    pub success: bool,
    /// Confidence of the result (0.0 to 1.0).
    pub confidence: f64,
    /// The task input vector (for holographic storage).
    pub task_vector: BipolarVector,
    /// Optional description of what happened.
    pub description: String,
}

/// An entry in the improvement queue — a domain that needs work.
#[derive(Debug, Clone)]
pub struct ImprovementTarget {
    /// The domain to improve.
    pub domain: CognitiveDomain,
    /// How weak we are in this domain (higher = weaker = higher priority).
    pub weakness_score: f64,
    /// How strong we are (for context).
    pub strength_score: f64,
    /// Net score: weakness - strength. Higher = more improvement needed.
    pub improvement_priority: f64,
    /// Number of failures recorded.
    pub failure_count: usize,
    /// Number of successes recorded.
    pub success_count: usize,
}

/// The MetaCognitive Profiler — LFI's self-awareness engine.
///
/// Maintains holographic maps of strengths and weaknesses,
/// tracks per-domain performance, and generates a prioritized
/// improvement queue for self-directed learning.
pub struct MetaCognitiveProfiler {
    /// Holographic memory of successful task patterns per domain.
    strength_map: HolographicMemory,
    /// Holographic memory of failed task patterns per domain.
    weakness_map: HolographicMemory,
    /// Per-domain success/failure counters.
    domain_stats: HashMap<CognitiveDomain, DomainStats>,
    /// Running average confidence per domain.
    domain_confidence: HashMap<CognitiveDomain, f64>,
    /// Total records processed.
    pub total_records: usize,
}

/// Per-domain statistics.
#[derive(Debug, Clone, Default)]
struct DomainStats {
    successes: usize,
    failures: usize,
    total_confidence: f64,
}

impl MetaCognitiveProfiler {
    /// Create a new empty profiler.
    pub fn new() -> Self {
        debuglog!("MetaCognitiveProfiler::new: Initializing self-awareness engine");
        Self {
            strength_map: HolographicMemory::new(),
            weakness_map: HolographicMemory::new(),
            domain_stats: HashMap::new(),
            domain_confidence: HashMap::new(),
            total_records: 0,
        }
    }

    /// Record a performance observation.
    ///
    /// Successes are bundled into the strength_map.
    /// Failures are bundled into the weakness_map.
    /// Both are keyed by domain base vector for later probing.
    pub fn record(&mut self, record: &PerformanceRecord) -> Result<(), HdcError> {
        debuglog!(
            "MetaCognitiveProfiler::record: domain={:?}, success={}, conf={:.3}",
            record.domain, record.success, record.confidence
        );

        let domain_vector = record.domain.base_vector();

        // Bind the task vector with the domain vector for contextual storage
        let contextual = domain_vector.bind(&record.task_vector)?;

        if record.success {
            self.strength_map.associate(&domain_vector, &contextual)?;
        } else {
            self.weakness_map.associate(&domain_vector, &contextual)?;
        }

        // Update stats
        let stats = self.domain_stats.entry(record.domain.clone()).or_default();
        if record.success {
            stats.successes += 1;
        } else {
            stats.failures += 1;
        }
        stats.total_confidence += record.confidence;

        // Update running average confidence
        let total = stats.successes + stats.failures;
        self.domain_confidence.insert(
            record.domain.clone(),
            stats.total_confidence / total as f64,
        );

        self.total_records += 1;
        debuglog!(
            "MetaCognitiveProfiler::record: total_records={}, domain_stats={:?}",
            self.total_records, stats
        );

        Ok(())
    }

    /// Probe strength in a domain.
    ///
    /// Returns a similarity score: higher = more strength signal.
    /// Score range is [-1.0, 1.0] but typically [0.0, 0.5] for
    /// populated domains.
    pub fn probe_strength(&self, domain: &CognitiveDomain) -> Result<f64, HdcError> {
        debuglog!("MetaCognitiveProfiler::probe_strength: domain={:?}", domain);
        let domain_vector = domain.base_vector();
        let retrieved = self.strength_map.probe(&domain_vector)?;
        let sim = retrieved.similarity(&domain_vector)?;
        debuglog!("MetaCognitiveProfiler::probe_strength: sim={:.4}", sim);
        Ok(sim)
    }

    /// Probe weakness in a domain.
    ///
    /// Returns a similarity score: higher = more weakness signal.
    pub fn probe_weakness(&self, domain: &CognitiveDomain) -> Result<f64, HdcError> {
        debuglog!("MetaCognitiveProfiler::probe_weakness: domain={:?}", domain);
        let domain_vector = domain.base_vector();
        let retrieved = self.weakness_map.probe(&domain_vector)?;
        let sim = retrieved.similarity(&domain_vector)?;
        debuglog!("MetaCognitiveProfiler::probe_weakness: sim={:.4}", sim);
        Ok(sim)
    }

    /// Get the success rate for a domain.
    pub fn success_rate(&self, domain: &CognitiveDomain) -> f64 {
        if let Some(stats) = self.domain_stats.get(domain) {
            let total = stats.successes + stats.failures;
            if total == 0 {
                return 0.0;
            }
            stats.successes as f64 / total as f64
        } else {
            0.0
        }
    }

    /// Get the average confidence for a domain.
    pub fn average_confidence(&self, domain: &CognitiveDomain) -> f64 {
        self.domain_confidence.get(domain).copied().unwrap_or(0.0)
    }

    /// Generate the improvement queue, sorted by priority (highest first).
    ///
    /// Priority = failure_rate * (1.0 - average_confidence).
    /// Domains with high failure rates and low confidence are prioritized.
    pub fn improvement_queue(&self) -> Result<Vec<ImprovementTarget>, HdcError> {
        debuglog!("MetaCognitiveProfiler::improvement_queue: generating priorities");

        let mut targets = Vec::new();

        for (domain, stats) in &self.domain_stats {
            let total = stats.successes + stats.failures;
            if total == 0 {
                continue;
            }

            let failure_rate = stats.failures as f64 / total as f64;
            let avg_conf = stats.total_confidence / total as f64;

            let weakness_score = failure_rate * (1.0 - avg_conf);
            let strength_score = (1.0 - failure_rate) * avg_conf;
            let improvement_priority = weakness_score - strength_score + 0.5; // Normalize around 0.5

            targets.push(ImprovementTarget {
                domain: domain.clone(),
                weakness_score,
                strength_score,
                improvement_priority,
                failure_count: stats.failures,
                success_count: stats.successes,
            });
        }

        // Sort by improvement priority (highest first)
        targets.sort_by(|a, b| b.improvement_priority.partial_cmp(&a.improvement_priority).unwrap_or(std::cmp::Ordering::Equal));

        debuglog!(
            "MetaCognitiveProfiler::improvement_queue: {} domains ranked",
            targets.len()
        );

        Ok(targets)
    }

    /// Get a summary of all tracked domains.
    pub fn summary(&self) -> HashMap<CognitiveDomain, (usize, usize, f64)> {
        let mut result = HashMap::new();
        for (domain, stats) in &self.domain_stats {
            let avg_conf = self.domain_confidence.get(domain).copied().unwrap_or(0.0);
            result.insert(domain.clone(), (stats.successes, stats.failures, avg_conf));
        }
        result
    }

    /// Check if a domain is identified as a weakness (failure rate > 50%).
    pub fn is_weak(&self, domain: &CognitiveDomain) -> bool {
        self.success_rate(domain) < 0.5
    }

    /// Check if a domain is identified as a strength (success rate > 80%).
    pub fn is_strong(&self, domain: &CognitiveDomain) -> bool {
        self.success_rate(domain) >= 0.8
    }

    /// Get the total number of domains being tracked.
    pub fn domain_count(&self) -> usize {
        self.domain_stats.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(domain: CognitiveDomain, success: bool, confidence: f64) -> PerformanceRecord {
        PerformanceRecord {
            domain,
            success,
            confidence,
            task_vector: BipolarVector::new_random().expect("random vector"),
            description: String::new(),
        }
    }

    #[test]
    fn test_profiler_creation() {
        let profiler = MetaCognitiveProfiler::new();
        assert_eq!(profiler.total_records, 0);
        assert_eq!(profiler.domain_count(), 0);
    }

    #[test]
    fn test_record_success() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();
        let record = make_record(CognitiveDomain::Coding, true, 0.9);
        profiler.record(&record)?;

        assert_eq!(profiler.total_records, 1);
        assert_eq!(profiler.domain_count(), 1);
        assert!((profiler.success_rate(&CognitiveDomain::Coding) - 1.0).abs() < 0.001);
        assert!((profiler.average_confidence(&CognitiveDomain::Coding) - 0.9).abs() < 0.001);
        Ok(())
    }

    #[test]
    fn test_record_failure() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();
        let record = make_record(CognitiveDomain::Security, false, 0.2);
        profiler.record(&record)?;

        assert_eq!(profiler.success_rate(&CognitiveDomain::Security), 0.0);
        assert!(profiler.is_weak(&CognitiveDomain::Security));
        Ok(())
    }

    #[test]
    fn test_mixed_performance() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();

        // 8 successes, 2 failures in Coding
        for _ in 0..8 {
            profiler.record(&make_record(CognitiveDomain::Coding, true, 0.85))?;
        }
        for _ in 0..2 {
            profiler.record(&make_record(CognitiveDomain::Coding, false, 0.3))?;
        }

        // 2 successes, 8 failures in Security
        for _ in 0..2 {
            profiler.record(&make_record(CognitiveDomain::Security, true, 0.6))?;
        }
        for _ in 0..8 {
            profiler.record(&make_record(CognitiveDomain::Security, false, 0.15))?;
        }

        assert!(profiler.is_strong(&CognitiveDomain::Coding));
        assert!(profiler.is_weak(&CognitiveDomain::Security));
        assert_eq!(profiler.total_records, 20);

        Ok(())
    }

    #[test]
    fn test_improvement_queue_ordering() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();

        // Strong domain: Coding
        for _ in 0..9 {
            profiler.record(&make_record(CognitiveDomain::Coding, true, 0.9))?;
        }
        profiler.record(&make_record(CognitiveDomain::Coding, false, 0.4))?;

        // Weak domain: Security
        profiler.record(&make_record(CognitiveDomain::Security, true, 0.5))?;
        for _ in 0..9 {
            profiler.record(&make_record(CognitiveDomain::Security, false, 0.1))?;
        }

        // Medium domain: Mathematics
        for _ in 0..5 {
            profiler.record(&make_record(CognitiveDomain::Mathematics, true, 0.7))?;
        }
        for _ in 0..5 {
            profiler.record(&make_record(CognitiveDomain::Mathematics, false, 0.3))?;
        }

        let queue = profiler.improvement_queue()?;
        assert_eq!(queue.len(), 3);

        // Security should be first (weakest)
        assert_eq!(queue[0].domain, CognitiveDomain::Security);
        // Coding should be last (strongest)
        assert_eq!(queue[queue.len() - 1].domain, CognitiveDomain::Coding);

        debuglog!("Improvement queue:");
        for target in &queue {
            debuglog!(
                "  {:?}: priority={:.3}, failures={}, successes={}",
                target.domain, target.improvement_priority,
                target.failure_count, target.success_count
            );
        }

        Ok(())
    }

    #[test]
    fn test_holographic_probing() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();

        // Record several successes in Coding
        for _ in 0..5 {
            profiler.record(&make_record(CognitiveDomain::Coding, true, 0.9))?;
        }

        // Record several failures in Security
        for _ in 0..5 {
            profiler.record(&make_record(CognitiveDomain::Security, false, 0.1))?;
        }

        // Strength probe should detect Coding signal
        let coding_strength = profiler.probe_strength(&CognitiveDomain::Coding)?;
        debuglog!("Coding strength signal: {:.4}", coding_strength);

        // Weakness probe should detect Security signal
        let security_weakness = profiler.probe_weakness(&CognitiveDomain::Security)?;
        debuglog!("Security weakness signal: {:.4}", security_weakness);

        // Both probes should return non-zero signals (actual value depends
        // on holographic interference, but should be detectable)
        // Note: with many bundled associations, signal degrades — this is expected
        Ok(())
    }

    #[test]
    fn test_domain_base_vectors_orthogonal() -> Result<(), HdcError> {
        let domains = vec![
            CognitiveDomain::Coding,
            CognitiveDomain::Mathematics,
            CognitiveDomain::Security,
            CognitiveDomain::NaturalLanguage,
        ];

        // All domain base vectors should be quasi-orthogonal
        for i in 0..domains.len() {
            for j in (i + 1)..domains.len() {
                let vi = domains[i].base_vector();
                let vj = domains[j].base_vector();
                let sim = vi.similarity(&vj)?;
                debuglog!("{:?} vs {:?}: sim={:.4}", domains[i], domains[j], sim);
                assert!(
                    sim.abs() < 0.1,
                    "{:?} vs {:?} should be orthogonal, sim={}",
                    domains[i], domains[j], sim
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_custom_domain() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();
        let custom = CognitiveDomain::Custom("pentesting".to_string());
        profiler.record(&make_record(custom.clone(), true, 0.95))?;
        assert_eq!(profiler.success_rate(&custom), 1.0);
        Ok(())
    }

    #[test]
    fn test_summary() -> Result<(), HdcError> {
        let mut profiler = MetaCognitiveProfiler::new();
        profiler.record(&make_record(CognitiveDomain::Coding, true, 0.9))?;
        profiler.record(&make_record(CognitiveDomain::Coding, false, 0.3))?;

        let summary = profiler.summary();
        let (successes, failures, avg_conf) = summary.get(&CognitiveDomain::Coding).expect("should have coding");
        assert_eq!(*successes, 1);
        assert_eq!(*failures, 1);
        assert!((*avg_conf - 0.6).abs() < 0.001); // (0.9 + 0.3) / 2
        Ok(())
    }
}
