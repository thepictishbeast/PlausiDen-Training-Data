// ============================================================
// Tier 1 — Forensic AST Generator
// Section 1.III: "Generates mathematically perfect ASTs."
//
// This module defines the trait for Tier 1 generation: taking
// a symbolic or vector-space input and producing a verified AST.
// The AST is the immutable logical backbone; Tier 2 decorates.
// ============================================================

use crate::hdlm::ast::{Ast, NodeKind, NodeId};
use crate::hdlm::error::HdlmError;
use crate::hdc::vector::BipolarVector;
use crate::debuglog;

/// Trait for Tier 1 forensic generators.
/// Implementations produce ASTs from various input modalities.
pub trait ForensicGenerator {
    /// Generate an AST from a sequence of symbolic tokens.
    fn generate_from_tokens(&self, tokens: &[&str]) -> Result<Ast, HdlmError>;

    /// Generate an AST from a hypervector representation.
    /// This bridges the HDC and HDLM subsystems.
    fn generate_from_vector(&self, hv: &BipolarVector) -> Result<Ast, HdlmError>;
}

/// Minimal forensic generator for arithmetic expressions.
/// Demonstrates the Tier 1 pipeline: tokens -> verified AST.
///
/// Supported grammar: number, (expr op expr)
/// where op is one of: +, -, *, /
pub struct ArithmeticGenerator;

impl ArithmeticGenerator {
    /// Parse a flat token sequence into an expression AST.
    /// Tokens are expected in prefix notation: [op, left, right] or [literal].
    pub fn parse_prefix(ast: &mut Ast, tokens: &[&str], pos: &mut usize) -> Result<NodeId, HdlmError> {
        if *pos >= tokens.len() {
            return Err(HdlmError::Tier1GenerationFailed {
                reason: "Unexpected end of token stream".to_string(),
            });
        }

        let token = tokens[*pos];
        *pos += 1;

        debuglog!("ArithmeticGenerator::parse_prefix: token='{}', pos={}", token, *pos);

        match token {
            "+" | "-" | "*" | "/" => {
                let op_node = ast.add_node(NodeKind::BinaryOp { operator: token.to_string() });
                let left = Self::parse_prefix(ast, tokens, pos)?;
                let right = Self::parse_prefix(ast, tokens, pos)?;
                ast.add_child(op_node, left)?;
                ast.add_child(op_node, right)?;
                Ok(op_node)
            }
            _ => {
                // Attempt to parse as literal
                let _: f64 = token.parse().map_err(|_| HdlmError::Tier1GenerationFailed {
                    reason: format!("Invalid literal: '{}'", token),
                })?;
                let lit_node = ast.add_node(NodeKind::Literal { value: token.to_string() });
                Ok(lit_node)
            }
        }
    }
}

impl ForensicGenerator for ArithmeticGenerator {
    fn generate_from_tokens(&self, tokens: &[&str]) -> Result<Ast, HdlmError> {
        if tokens.is_empty() {
            return Err(HdlmError::Tier1GenerationFailed {
                reason: "Empty token stream".to_string(),
            });
        }

        let mut ast = Ast::new();
        let root = ast.add_node(NodeKind::Root);
        let mut pos = 0;
        let expr = Self::parse_prefix(&mut ast, tokens, &mut pos)?;
        ast.add_child(root, expr)?;

        if pos != tokens.len() {
            return Err(HdlmError::Tier1GenerationFailed {
                reason: format!("Unconsumed tokens: consumed {}, total {}", pos, tokens.len()),
            });
        }

        debuglog!(
            "ArithmeticGenerator::generate_from_tokens: {} nodes, {} tokens consumed",
            ast.node_count(), pos
        );
        Ok(ast)
    }

    fn generate_from_vector(&self, _hv: &BipolarVector) -> Result<Ast, HdlmError> {
        // Vector-to-AST decoding requires a trained codebook (item memory).
        // Structural placeholder — Beta defines the semantic mapping axioms.
        debuglog!("ArithmeticGenerator::generate_from_vector: NOT IMPLEMENTED (awaiting codebook)");
        Err(HdlmError::Tier1GenerationFailed {
            reason: "Vector-to-AST decoding requires trained HDC codebook (pending Phase 3)".to_string(),
        })
    }
}

// ============================================================
// Tier 1 Tests
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_literal() -> Result<(), HdlmError> {
        let gen = ArithmeticGenerator;
        let ast = gen.generate_from_tokens(&["42"])?;
        assert_eq!(ast.node_count(), 2); // Root + Literal
        assert_eq!(ast.leaf_count(), 1);
        Ok(())
    }

    #[test]
    fn test_simple_addition() -> Result<(), HdlmError> {
        // Prefix: + 3 4
        let gen = ArithmeticGenerator;
        let ast = gen.generate_from_tokens(&["+", "3", "4"])?;
        // Root -> BinaryOp(+) -> [Literal(3), Literal(4)]
        assert_eq!(ast.node_count(), 4); // Root, +, 3, 4
        assert_eq!(ast.leaf_count(), 2);
        Ok(())
    }

    #[test]
    fn test_nested_expression() -> Result<(), HdlmError> {
        // Prefix: * + 1 2 - 5 3  =>  (1 + 2) * (5 - 3)
        let gen = ArithmeticGenerator;
        let ast = gen.generate_from_tokens(&["*", "+", "1", "2", "-", "5", "3"])?;
        // Root, *, +, 1, 2, -, 5, 3 = 8 nodes
        assert_eq!(ast.node_count(), 8);
        assert_eq!(ast.leaf_count(), 4); // 1, 2, 5, 3
        Ok(())
    }

    #[test]
    fn test_dfs_of_generated_ast() -> Result<(), HdlmError> {
        let gen = ArithmeticGenerator;
        let ast = gen.generate_from_tokens(&["+", "1", "2"])?;
        let order = ast.dfs()?;
        assert_eq!(order.len(), 4);
        // Root(0), BinaryOp(1), Literal(2), Literal(3)
        assert_eq!(order[0], 0); // Root first
        Ok(())
    }

    #[test]
    fn test_empty_tokens_fails() {
        let gen = ArithmeticGenerator;
        assert!(gen.generate_from_tokens(&[]).is_err());
    }

    #[test]
    fn test_invalid_literal_fails() {
        let gen = ArithmeticGenerator;
        let result = gen.generate_from_tokens(&["not_a_number"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_unconsumed_tokens_fails() {
        let gen = ArithmeticGenerator;
        // "42 99" — 42 is consumed, 99 is extra
        let result = gen.generate_from_tokens(&["42", "99"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_truncated_expression_fails() {
        let gen = ArithmeticGenerator;
        // "+" expects two operands, only gets one
        let result = gen.generate_from_tokens(&["+", "1"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_four_operators() -> Result<(), HdlmError> {
        let gen = ArithmeticGenerator;
        for op in &["+", "-", "*", "/"] {
            let ast = gen.generate_from_tokens(&[op, "10", "5"])?;
            assert_eq!(ast.node_count(), 4);
        }
        Ok(())
    }
}
