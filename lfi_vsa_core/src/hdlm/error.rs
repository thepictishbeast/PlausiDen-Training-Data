// ============================================================
// HDLM Error Types — Multi-Level Semantic Mapping errors.
// ============================================================

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum HdlmError {
    /// AST node could not be constructed from the given input.
    MalformedAst {
        reason: String,
    },
    /// Tier 1 forensic generation failed.
    Tier1GenerationFailed {
        reason: String,
    },
    /// Tier 2 decorative expansion failed.
    Tier2ExpansionFailed {
        reason: String,
    },
    /// Attempted to traverse an empty AST.
    EmptyAst,
    /// Vector-to-AST decoding encountered an unmapped symbol.
    UnmappedSymbol {
        symbol_id: usize,
    },
}

impl fmt::Display for HdlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedAst { reason } => {
                write!(f, "HDLM MalformedAst: {}", reason)
            }
            Self::Tier1GenerationFailed { reason } => {
                write!(f, "HDLM Tier1GenerationFailed: {}", reason)
            }
            Self::Tier2ExpansionFailed { reason } => {
                write!(f, "HDLM Tier2ExpansionFailed: {}", reason)
            }
            Self::EmptyAst => {
                write!(f, "HDLM EmptyAst: cannot operate on empty tree")
            }
            Self::UnmappedSymbol { symbol_id } => {
                write!(f, "HDLM UnmappedSymbol: id={}", symbol_id)
            }
        }
    }
}

impl std::error::Error for HdlmError {}
