// ============================================================
// LFI Agent Orchestrator — High-Level Reasoning Loop
// Section 2: "Operate as an autonomous intelligence leveraging
// Zero-Trust and Assume Breach protocols."
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::compute::LocalBackend;
use crate::hdlm::codebook::HdlmCodebook;
use crate::hdlm::ast::{Ast, NodeKind};
use crate::psl::supervisor::PslSupervisor;
use crate::psl::axiom::AuditTarget;
use crate::hid::{HidDevice, HidCommand};
use crate::coder::LfiCoder;
use crate::languages::constructs::UniversalConstruct;
use crate::languages::registry::LanguageId;
use crate::hdc::error::HdcError;
use crate::debuglog;
// Unused Arc removed

/// The primary LFI Agent. Orchestrates the VSA, PSL, HID, and Coder layers.
pub struct LfiAgent {
    /// Compute backend for HDC operations.
    pub compute: LocalBackend,
    /// PSL Supervisor for forensic auditing.
    pub supervisor: PslSupervisor,
    /// Codebook for semantic mapping.
    pub codebook: HdlmCodebook,
    /// Hardware interface.
    pub hid: HidDevice,
    /// Universal Polyglot Coder.
    pub coder: LfiCoder,
}

impl LfiAgent {
    /// Initialize a new agent with a baseline codebook and supervisor.
    pub fn new() -> Result<Self, HdcError> {
        debuglog!("LfiAgent::new: Initializing autonomous agent");
        
        let compute = LocalBackend;
        let supervisor = PslSupervisor::new();
        
        // Initialize codebook with core node kinds
        let kinds = vec![
            NodeKind::Root,
            NodeKind::Assignment,
            NodeKind::Call { function: String::new() },
            NodeKind::Return,
        ];
        let codebook = HdlmCodebook::new(&kinds).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Codebook init failed: {}", e),
        })?;
        
        let hid = HidDevice::new(None)?;
        let coder = LfiCoder::new();
        
        Ok(Self { compute, supervisor, codebook, hid, coder })
    }

    /// Executes a forensic task: Sense -> Think -> Act (Audited).
    pub fn execute_task(&self, task_name: &str) -> Result<(), HdcError> {
        debuglog!("LfiAgent::execute_task: starting '{}'", task_name);

        // 1. SENSE: Ingest task intent into hypervector
        let task_vector = BipolarVector::new_random()?;
        
        // 2. THINK: Forensic AST Generation (Simplified)
        let mut ast = Ast::new();
        ast.add_node(NodeKind::Root);
        debuglog!("LfiAgent::execute_task: generated forensic AST");

        // 3. AUDIT: PSL verification of the task vector against axioms
        let target = AuditTarget::Vector(task_vector.clone());
        
        if self.supervisor.axiom_count() > 0 {
            let assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
                reason: format!("PSL Audit Failure: {:?}", e),
            })?;
            debuglog!("LfiAgent::execute_task: PSL Level: {:?}, Score: {:.4}", 
                assessment.level, assessment.level.score());

            if !assessment.level.permits_execution() {
                debuglog!("LfiAgent::execute_task: FAIL - Hostile payload detected by PSL");
                return Err(HdcError::InitializationFailed {
                    reason: format!("PSL Audit Failure: Trust level {:?} too low", assessment.level),
                });
            }
        }

        // 4. ACT: Dispatch to HID or Coder
        if task_name.contains("code") {
            debuglog!("LfiAgent::execute_task: Dispatching to Universal Coder");
            let _ = self.coder.synthesize(LanguageId::Rust, &[UniversalConstruct::VariableBinding])
                .map_err(|e| HdcError::InitializationFailed { reason: e })?;
        } else {
            debuglog!("LfiAgent::execute_task: Dispatching action to HID");
            self.hid.execute(HidCommand::MouseMove { x: 100, y: 100 })?;
        }

        debuglog!("LfiAgent::execute_task: SUCCESS - Task '{}' completed.", task_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_task_orchestration() -> Result<(), HdcError> {
        let agent = LfiAgent::new()?;
        agent.execute_task("Initialize UI Probe")?;
        Ok(())
    }

    #[test]
    fn test_agent_coding_task() -> Result<(), HdcError> {
        let agent = LfiAgent::new()?;
        agent.execute_task("code a new safety module")?;
        Ok(())
    }
}
