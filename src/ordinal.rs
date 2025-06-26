//! Ordinal Analysis for C∀O (Kao) termination verification
//!
//! This module implements ordinal analysis to ensure program termination and system consistency.
//! For Phase 1, this is a mock implementation that will be replaced with true ordinal analysis.

use crate::types::{OrdinalValue, Token, WordDefinition};
use std::fmt;

/// Errors that can occur during ordinal verification
#[derive(Debug, Clone)]
pub enum OrdinalError {
    /// Non-terminating recursion detected
    NonTerminating(String),
    /// Invalid ordinal structure
    InvalidOrdinal(String),
    /// Ordinal arithmetic overflow
    Overflow(String),
    /// Consistency check failed
    ConsistencyFailed(String),
}

impl fmt::Display for OrdinalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdinalError::NonTerminating(msg) => write!(f, "Non-terminating: {}", msg),
            OrdinalError::InvalidOrdinal(msg) => write!(f, "Invalid ordinal: {}", msg),
            OrdinalError::Overflow(msg) => write!(f, "Ordinal overflow: {}", msg),
            OrdinalError::ConsistencyFailed(msg) => write!(f, "Consistency failed: {}", msg),
        }
    }
}

impl std::error::Error for OrdinalError {}

/// Mock ordinal verifier for Phase 1 implementation
pub struct OrdinalVerifier {
    /// Whether to use strict checking (disabled for mock)
    strict_mode: bool,
}

impl OrdinalVerifier {
    /// Create a new ordinal verifier
    pub fn new() -> Self {
        OrdinalVerifier {
            strict_mode: false, // Mock mode
        }
    }

    /// Verify that a word definition terminates and compute its ordinal cost
    pub fn verify_termination(
        &self,
        word_def: &WordDefinition,
    ) -> Result<OrdinalValue, OrdinalError> {
        if self.strict_mode {
            self.analyze_termination_strict(word_def)
        } else {
            self.analyze_termination_mock(word_def)
        }
    }

    /// Mock termination analysis (Phase 1)
    fn analyze_termination_mock(
        &self,
        word_def: &WordDefinition,
    ) -> Result<OrdinalValue, OrdinalError> {
        // Simple heuristics for mock verification:
        // 1. If the word contains recursive calls to itself, assign higher cost
        // 2. If it's clearly non-recursive, assign cost 1
        // 3. For complex cases, assign a reasonable finite ordinal

        let recursive_calls = self.count_recursive_calls(&word_def.body, &word_def.name);

        if recursive_calls == 0 {
            // Non-recursive word
            Ok(OrdinalValue::Finite(1))
        } else if recursive_calls <= 2 && self.has_decreasing_pattern(&word_def.body) {
            // Likely terminating recursion with decreasing pattern
            Ok(OrdinalValue::Finite((recursive_calls + 1) as u64))
        } else if recursive_calls > 10 {
            // Suspicious - might be non-terminating
            Err(OrdinalError::NonTerminating(format!(
                "Word '{}' has {} recursive calls without clear termination",
                word_def.name, recursive_calls
            )))
        } else {
            // Assume terminating but assign higher ordinal cost
            Ok(OrdinalValue::Finite(recursive_calls as u64 * 2))
        }
    }

    /// Strict termination analysis (future implementation)
    fn analyze_termination_strict(
        &self,
        _word_def: &WordDefinition,
    ) -> Result<OrdinalValue, OrdinalError> {
        // This would implement true ordinal analysis:
        // 1. Build call graph
        // 2. Identify recursive cycles
        // 3. Analyze decreasing measures
        // 4. Construct ordinal proof
        // 5. Verify well-foundedness

        Err(OrdinalError::ConsistencyFailed(
            "Strict ordinal analysis not yet implemented".to_string(),
        ))
    }

    /// Count recursive calls in a token sequence
    fn count_recursive_calls(&self, tokens: &[Token], word_name: &str) -> usize {
        tokens
            .iter()
            .filter(|token| {
                if let Token::Word(name) = token {
                    name == word_name
                } else {
                    false
                }
            })
            .count()
    }

    /// Check for decreasing patterns that suggest termination
    fn has_decreasing_pattern(&self, tokens: &[Token]) -> bool {
        // Look for patterns like:
        // - Numerical decrements (n 1 -)
        // - Conditional termination (0 = [...] [...] if)
        // - List/structure decomposition patterns

        let mut i = 0;
        while i < tokens.len() {
            // Pattern: number 1 -
            if i + 2 < tokens.len() {
                if let (
                    Token::Literal(_),
                    Token::Literal(crate::types::Value::Nat(1)),
                    Token::Word(op),
                ) = (&tokens[i], &tokens[i + 1], &tokens[i + 2])
                {
                    if op == "-" {
                        return true;
                    }
                }
            }

            // Pattern: 0 = (equality check with zero)
            if i + 1 < tokens.len() {
                if let (Token::Literal(crate::types::Value::Nat(0)), Token::Word(op)) =
                    (&tokens[i], &tokens[i + 1])
                {
                    if op == "=" || op == "<" {
                        return true;
                    }
                }
            }

            i += 1;
        }

        false
    }

    /// Compare two ordinals (for ordering verification)
    pub fn compare_ordinals(&self, a: &OrdinalValue, b: &OrdinalValue) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (a, b) {
            (OrdinalValue::Zero, OrdinalValue::Zero) => Ordering::Equal,
            (OrdinalValue::Zero, _) => Ordering::Less,
            (_, OrdinalValue::Zero) => Ordering::Greater,

            (OrdinalValue::Finite(x), OrdinalValue::Finite(y)) => x.cmp(y),
            (OrdinalValue::Finite(_), _) => Ordering::Less,
            (_, OrdinalValue::Finite(_)) => Ordering::Greater,

            (OrdinalValue::Omega, OrdinalValue::Omega) => Ordering::Equal,
            (OrdinalValue::Omega, OrdinalValue::OmegaPower(_)) => Ordering::Less,
            (OrdinalValue::OmegaPower(_), OrdinalValue::Omega) => Ordering::Greater,

            // For more complex ordinals, use simplified comparison
            _ => Ordering::Equal, // Mock implementation
        }
    }

    /// Add two ordinals
    pub fn add_ordinals(
        &self,
        a: &OrdinalValue,
        b: &OrdinalValue,
    ) -> Result<OrdinalValue, OrdinalError> {
        match (a, b) {
            (OrdinalValue::Zero, x) | (x, OrdinalValue::Zero) => Ok(x.clone()),

            (OrdinalValue::Finite(x), OrdinalValue::Finite(y)) => x
                .checked_add(*y)
                .map(OrdinalValue::Finite)
                .ok_or_else(|| OrdinalError::Overflow("Finite ordinal addition".to_string())),

            (OrdinalValue::Finite(_), x) | (x, OrdinalValue::Finite(_)) => {
                // ω + n = ω, n + ω = ω
                Ok(x.clone())
            }

            // For complex ordinals, return the larger one (simplified)
            _ => {
                if self.compare_ordinals(a, b) == std::cmp::Ordering::Greater {
                    Ok(a.clone())
                } else {
                    Ok(b.clone())
                }
            }
        }
    }

    /// Multiply two ordinals
    pub fn multiply_ordinals(
        &self,
        a: &OrdinalValue,
        b: &OrdinalValue,
    ) -> Result<OrdinalValue, OrdinalError> {
        match (a, b) {
            (OrdinalValue::Zero, _) | (_, OrdinalValue::Zero) => Ok(OrdinalValue::Zero),
            (OrdinalValue::Finite(1), x) | (x, OrdinalValue::Finite(1)) => Ok(x.clone()),

            (OrdinalValue::Finite(x), OrdinalValue::Finite(y)) => x
                .checked_mul(*y)
                .map(OrdinalValue::Finite)
                .ok_or_else(|| OrdinalError::Overflow("Finite ordinal multiplication".to_string())),

            // Simplified: ω * n = ω (for n > 0)
            (OrdinalValue::Omega, OrdinalValue::Finite(n)) if *n > 0 => Ok(OrdinalValue::Omega),
            (OrdinalValue::Finite(n), OrdinalValue::Omega) if *n > 0 => Ok(OrdinalValue::Omega),

            // ω * ω = ω²
            (OrdinalValue::Omega, OrdinalValue::Omega) => {
                Ok(OrdinalValue::OmegaPower(Box::new(OrdinalValue::Finite(2))))
            }

            _ => Err(OrdinalError::InvalidOrdinal(
                "Complex ordinal multiplication not implemented".to_string(),
            )),
        }
    }

    /// Check if an ordinal sequence is strictly decreasing (well-founded)
    pub fn is_well_founded(&self, sequence: &[OrdinalValue]) -> bool {
        if sequence.len() <= 1 {
            return true;
        }

        for i in 1..sequence.len() {
            if self.compare_ordinals(&sequence[i - 1], &sequence[i]) != std::cmp::Ordering::Greater
            {
                return false;
            }
        }

        true
    }

    /// Verify global consistency of ordinal assignments
    pub fn verify_global_consistency(
        &self,
        word_definitions: &[WordDefinition],
    ) -> Result<(), OrdinalError> {
        // Check that ordinal assignments are consistent across all definitions
        // This would ensure no paradoxes in the type system

        if self.strict_mode {
            // In strict mode, we would:
            // 1. Build dependency graph
            // 2. Verify ordinal assignments respect dependencies
            // 3. Check for circular dependencies
            // 4. Ensure proof-theoretic consistency

            Err(OrdinalError::ConsistencyFailed(
                "Global consistency checking not implemented in strict mode".to_string(),
            ))
        } else {
            // Mock verification - just check for obviously problematic cases
            for word_def in word_definitions {
                if word_def.ordinal_cost == OrdinalValue::Zero && !word_def.is_axiom {
                    return Err(OrdinalError::ConsistencyFailed(format!(
                        "Non-axiom word '{}' has zero ordinal cost",
                        word_def.name
                    )));
                }
            }

            Ok(())
        }
    }
}

impl Default for OrdinalVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Type, TypeSignature};

    #[test]
    fn test_mock_verifier_non_recursive() {
        let verifier = OrdinalVerifier::new();
        let word_def = WordDefinition {
            name: "double".to_string(),
            signature: TypeSignature {
                inputs: vec![Type::Nat],
                outputs: vec![Type::Nat],
            },
            body: vec![
                Token::Literal(crate::types::Value::Nat(2)),
                Token::Word("*".to_string()),
            ],
            is_axiom: false,
            ordinal_cost: OrdinalValue::Finite(1),
        };

        let result = verifier.verify_termination(&word_def);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), OrdinalValue::Finite(1));
    }

    #[test]
    fn test_ordinal_comparison() {
        let verifier = OrdinalVerifier::new();

        assert_eq!(
            verifier.compare_ordinals(&OrdinalValue::Finite(1), &OrdinalValue::Finite(2)),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            verifier.compare_ordinals(&OrdinalValue::Finite(5), &OrdinalValue::Omega),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_ordinal_addition() {
        let verifier = OrdinalVerifier::new();

        let result = verifier.add_ordinals(&OrdinalValue::Finite(3), &OrdinalValue::Finite(4));
        assert_eq!(result.unwrap(), OrdinalValue::Finite(7));

        let result = verifier.add_ordinals(&OrdinalValue::Finite(3), &OrdinalValue::Omega);
        assert_eq!(result.unwrap(), OrdinalValue::Omega);
    }

    #[test]
    fn test_well_founded_sequence() {
        let verifier = OrdinalVerifier::new();

        let decreasing = vec![
            OrdinalValue::Finite(5),
            OrdinalValue::Finite(3),
            OrdinalValue::Finite(1),
            OrdinalValue::Zero,
        ];

        assert!(verifier.is_well_founded(&decreasing));

        let increasing = vec![
            OrdinalValue::Finite(1),
            OrdinalValue::Finite(3),
            OrdinalValue::Finite(5),
        ];

        assert!(!verifier.is_well_founded(&increasing));
    }
}
