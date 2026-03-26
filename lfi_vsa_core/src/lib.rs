#![forbid(unsafe_code)]

// ============================================================
// LFI VSA Core — Crate Root
// Section 5: Absolute Memory Safety enforced via forbid(unsafe_code).
// All math operations return Result<T, E> or Option<T>.
// No .unwrap(), .expect(), or panic!() permitted.
// ============================================================

pub mod telemetry;
pub mod hdc;
pub mod psl;
pub mod hdlm;

// Re-export core public types for ergonomic access.
pub use hdc::error::HdcError;
pub use hdc::vector::{BipolarVector, HD_DIMENSIONS};
pub use hdc::compute::{ComputeBackend, LocalBackend};
pub use psl::supervisor::PslSupervisor;
pub use psl::trust::{TrustLevel, TrustAssessment};
pub use psl::axiom::{Axiom, AuditTarget, AxiomVerdict};
pub use hdlm::ast::{Ast, AstNode, NodeKind};
pub use hdlm::tier1_forensic::ForensicGenerator;
pub use hdlm::tier2_decorative::DecorativeExpander;
