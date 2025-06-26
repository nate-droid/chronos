//! Type Inference Engine for C∀O
//!
//! Implements Hindley-Milner style type inference to automatically deduce types
//! for word definitions, reducing boilerplate while maintaining type safety.

use crate::types::{Token, Type, TypeSignature, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during type inference
#[derive(Debug, Clone, PartialEq)]
pub enum InferenceError {
    /// Cannot unify two types
    UnificationError(Type, Type),
    /// Type variable occurs in the type it's being unified with (infinite type)
    OccursCheck(String, Type),
    /// Unknown word referenced in definition
    UnknownWord(String),
    /// Cannot infer type without more context
    InsufficientContext(String),
    /// Type inference failed for specific reason
    InferenceFailed(String),
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferenceError::UnificationError(t1, t2) => {
                write!(f, "Cannot unify types {} and {}", t1, t2)
            }
            InferenceError::OccursCheck(var, ty) => {
                write!(f, "Occurs check failed: {} occurs in {}", var, ty)
            }
            InferenceError::UnknownWord(word) => {
                write!(f, "Unknown word: {}", word)
            }
            InferenceError::InsufficientContext(msg) => {
                write!(f, "Insufficient context: {}", msg)
            }
            InferenceError::InferenceFailed(msg) => {
                write!(f, "Type inference failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for InferenceError {}

/// Type constraint used during inference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeConstraint {
    /// Two types must be equal
    Equal(Type, Type),
    /// A type must be unified with another
    Unify(Type, Type),
}

/// Type substitution mapping type variables to concrete types
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    mappings: HashMap<String, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, var: String, ty: Type) {
        self.mappings.insert(var, ty);
    }

    pub fn get(&self, var: &str) -> Option<&Type> {
        self.mappings.get(var)
    }

    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Variable(var) => {
                if let Some(concrete_type) = self.mappings.get(var) {
                    self.apply(concrete_type) // Apply substitution recursively
                } else {
                    ty.clone()
                }
            }
            Type::Composite { name, fields } => {
                let mut new_fields = HashMap::new();
                for (key, field_type) in fields {
                    new_fields.insert(key.clone(), self.apply(field_type));
                }
                Type::Composite {
                    name: name.clone(),
                    fields: new_fields,
                }
            }
            _ => ty.clone(), // Concrete types don't need substitution
        }
    }

    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = Substitution::new();

        // Apply this substitution to other's mappings
        for (var, ty) in &other.mappings {
            result.insert(var.clone(), self.apply(ty));
        }

        // Add this substitution's mappings (if not already present)
        for (var, ty) in &self.mappings {
            if !result.mappings.contains_key(var) {
                result.insert(var.clone(), ty.clone());
            }
        }

        result
    }
}

/// Main type inference engine
pub struct TypeInferer {
    /// Current type variable counter for generating fresh variables
    var_counter: u32,
    /// Active type constraints
    constraints: Vec<TypeConstraint>,
    /// Known word signatures from the environment
    word_signatures: HashMap<String, TypeSignature>,
    /// Debug mode flag
    debug: bool,
}

impl TypeInferer {
    pub fn new() -> Self {
        Self {
            var_counter: 0,
            constraints: Vec::new(),
            word_signatures: HashMap::new(),
            debug: false,
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn add_word_signature(&mut self, name: String, signature: TypeSignature) {
        self.word_signatures.insert(name, signature);
    }

    /// Generate a fresh type variable
    fn fresh_type_var(&mut self) -> Type {
        let var_name = format!("T{}", self.var_counter);
        self.var_counter += 1;
        Type::Variable(var_name)
    }

    /// Check if a type variable occurs in a type (occurs check)
    fn occurs_check(&self, var: &str, ty: &Type) -> bool {
        match ty {
            Type::Variable(v) => v == var,
            Type::Composite { fields, .. } => fields
                .values()
                .any(|field_ty| self.occurs_check(var, field_ty)),
            _ => false,
        }
    }

    /// Unify two types, returning a substitution
    fn unify(&mut self, t1: &Type, t2: &Type) -> Result<Substitution, InferenceError> {
        if self.debug {
            println!("Unifying {} with {}", t1, t2);
        }

        match (t1, t2) {
            // Same concrete types unify trivially
            (Type::Unit, Type::Unit)
            | (Type::Bool, Type::Bool)
            | (Type::Nat, Type::Nat)
            | (Type::Ordinal, Type::Ordinal)
            | (Type::Quote, Type::Quote) => Ok(Substitution::new()),

            // Variable unification
            (Type::Variable(var), ty) | (ty, Type::Variable(var)) => {
                if let Type::Variable(other_var) = ty {
                    if var == other_var {
                        return Ok(Substitution::new()); // Same variable
                    }
                }

                // Occurs check
                if self.occurs_check(var, ty) {
                    return Err(InferenceError::OccursCheck(var.clone(), ty.clone()));
                }

                let mut subst = Substitution::new();
                subst.insert(var.clone(), ty.clone());
                Ok(subst)
            }

            // Composite type unification
            (
                Type::Composite {
                    name: n1,
                    fields: f1,
                },
                Type::Composite {
                    name: n2,
                    fields: f2,
                },
            ) => {
                if n1 != n2 {
                    return Err(InferenceError::UnificationError(t1.clone(), t2.clone()));
                }

                if f1.len() != f2.len() {
                    return Err(InferenceError::UnificationError(t1.clone(), t2.clone()));
                }

                let mut combined_subst = Substitution::new();
                for (key, field1) in f1 {
                    if let Some(field2) = f2.get(key) {
                        let subst = self.unify(field1, field2)?;
                        combined_subst = combined_subst.compose(&subst);
                    } else {
                        return Err(InferenceError::UnificationError(t1.clone(), t2.clone()));
                    }
                }
                Ok(combined_subst)
            }

            // Cannot unify different concrete types
            _ => Err(InferenceError::UnificationError(t1.clone(), t2.clone())),
        }
    }

    /// Infer the type signature for a word definition
    pub fn infer_word_type(&mut self, tokens: &[Token]) -> Result<TypeSignature, InferenceError> {
        if self.debug {
            println!("Inferring type for tokens: {:?}", tokens);
        }

        // For simple cases, we'll implement stack-based type inference
        // This is a simplified version - a full implementation would be more complex

        if tokens.is_empty() {
            // Empty definition is identity function
            let t = self.fresh_type_var();
            return Ok(TypeSignature {
                inputs: vec![t.clone()],
                outputs: vec![t],
            });
        }

        // Simple pattern matching for common cases
        let (input_types, output_types) = match tokens {
            // Arithmetic operations: consume two Nat, produce one Nat
            [Token::Word(op)] if matches!(op.as_str(), "+" | "-" | "*" | "/" | "%") => {
                (vec![Type::Nat, Type::Nat], vec![Type::Nat])
            }

            // Comparison operations: consume two values of same type, produce Bool
            [Token::Word(op)] if matches!(op.as_str(), "=" | "<" | ">" | "<=" | ">=") => {
                let t = self.fresh_type_var();
                (vec![t.clone(), t], vec![Type::Bool])
            }

            // Stack operations
            [Token::Word(op)] if op == "dup" => {
                let t = self.fresh_type_var();
                (vec![t.clone()], vec![t.clone(), t])
            }

            [Token::Word(op)] if op == "drop" => {
                let t = self.fresh_type_var();
                (vec![t], vec![])
            }

            [Token::Word(op)] if op == "swap" => {
                let t1 = self.fresh_type_var();
                let t2 = self.fresh_type_var();
                (vec![t1.clone(), t2.clone()], vec![t2, t1])
            }

            // Literal followed by operation
            [Token::Literal(Value::Nat(_)), Token::Word(op)]
                if matches!(op.as_str(), "+" | "-" | "*" | "/" | "%") =>
            {
                (vec![Type::Nat], vec![Type::Nat])
            }

            // Just a literal
            [Token::Literal(Value::Nat(_))] => (vec![], vec![Type::Nat]),

            [Token::Literal(Value::Bool(_))] => (vec![], vec![Type::Bool]),

            [Token::Literal(Value::Unit)] => (vec![], vec![Type::Unit]),

            // Known word reference
            [Token::Word(word_name)] => {
                if let Some(signature) = self.word_signatures.get(word_name) {
                    return Ok(signature.clone());
                } else {
                    return Err(InferenceError::UnknownWord(word_name.clone()));
                }
            }

            // More complex cases - for now, return a generic signature
            _ => {
                if self.debug {
                    println!("Complex pattern detected, using generic inference");
                }

                // Try to infer based on sequence analysis
                self.infer_sequence_type(tokens)?
            }
        };

        Ok(TypeSignature {
            inputs: input_types,
            outputs: output_types,
        })
    }

    /// Infer types for a sequence of tokens (more complex analysis)
    fn infer_sequence_type(
        &mut self,
        tokens: &[Token],
    ) -> Result<(Vec<Type>, Vec<Type>), InferenceError> {
        // This is a simplified sequential analysis
        // A full implementation would use proper stack effect composition

        let mut stack_effect_inputs = Vec::new();
        let mut stack_effect_outputs = Vec::new();
        let mut current_stack_depth = 0i32;

        for token in tokens {
            match token {
                Token::Literal(Value::Nat(_)) => {
                    current_stack_depth += 1;
                    stack_effect_outputs.push(Type::Nat);
                }
                Token::Literal(Value::Bool(_)) => {
                    current_stack_depth += 1;
                    stack_effect_outputs.push(Type::Bool);
                }
                Token::Word(word) => {
                    match word.as_str() {
                        "+" | "-" | "*" | "/" | "%" => {
                            if current_stack_depth < 2 {
                                // Need to consume from input stack
                                let needed = 2 - current_stack_depth;
                                for _ in 0..needed {
                                    stack_effect_inputs.push(Type::Nat);
                                }
                                current_stack_depth = 0;
                            } else {
                                current_stack_depth -= 2;
                            }
                            current_stack_depth += 1;
                            if stack_effect_outputs.is_empty() {
                                stack_effect_outputs.push(Type::Nat);
                            }
                        }
                        "dup" => {
                            if current_stack_depth < 1 {
                                let t = self.fresh_type_var();
                                stack_effect_inputs.push(t.clone());
                                stack_effect_outputs.extend([t.clone(), t]);
                                current_stack_depth = 1;
                            } else {
                                current_stack_depth += 1;
                            }
                        }
                        _ => {
                            // Unknown word - use generic polymorphic signature
                            return Err(InferenceError::InsufficientContext(format!(
                                "Cannot infer type for unknown word: {}",
                                word
                            )));
                        }
                    }
                }
                _ => {
                    return Err(InferenceError::InsufficientContext(
                        "Complex token pattern not yet supported".to_string(),
                    ));
                }
            }
        }

        // If we have excess values on stack, they become outputs
        if stack_effect_outputs.is_empty() && current_stack_depth > 0 {
            // Generic output type
            stack_effect_outputs.push(self.fresh_type_var());
        }

        Ok((stack_effect_inputs, stack_effect_outputs))
    }

    /// Solve all accumulated constraints
    pub fn solve_constraints(&mut self) -> Result<Substitution, InferenceError> {
        let mut solution = Substitution::new();

        for constraint in &self.constraints.clone() {
            match constraint {
                TypeConstraint::Equal(t1, t2) | TypeConstraint::Unify(t1, t2) => {
                    let applied_t1 = solution.apply(t1);
                    let applied_t2 = solution.apply(t2);
                    let unifier = self.unify(&applied_t1, &applied_t2)?;
                    solution = solution.compose(&unifier);
                }
            }
        }

        self.constraints.clear();
        Ok(solution)
    }
}

impl Default for TypeInferer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_type_vars() {
        let mut inferer = TypeInferer::new();
        let t1 = inferer.fresh_type_var();
        let t2 = inferer.fresh_type_var();

        assert_ne!(t1, t2);
        assert!(matches!(t1, Type::Variable(_)));
        assert!(matches!(t2, Type::Variable(_)));
    }

    #[test]
    fn test_unify_concrete_types() {
        let mut inferer = TypeInferer::new();
        let result = inferer.unify(&Type::Nat, &Type::Nat);
        assert!(result.is_ok());

        let result = inferer.unify(&Type::Nat, &Type::Bool);
        assert!(result.is_err());
    }

    #[test]
    fn test_unify_variables() {
        let mut inferer = TypeInferer::new();
        let var = Type::Variable("T0".to_string());
        let result = inferer.unify(&var, &Type::Nat).unwrap();

        assert_eq!(result.get("T0"), Some(&Type::Nat));
    }

    #[test]
    fn test_occurs_check() {
        let inferer = TypeInferer::new();
        let var = "T0";
        let recursive_type = Type::Variable("T0".to_string());

        assert!(inferer.occurs_check(var, &recursive_type));
        assert!(!inferer.occurs_check(var, &Type::Nat));
    }

    #[test]
    fn test_infer_arithmetic() {
        let mut inferer = TypeInferer::new();
        let tokens = vec![Token::Word("+".to_string())];
        let result = inferer.infer_word_type(&tokens).unwrap();

        assert_eq!(result.inputs, vec![Type::Nat, Type::Nat]);
        assert_eq!(result.outputs, vec![Type::Nat]);
    }

    #[test]
    fn test_infer_literal() {
        let mut inferer = TypeInferer::new();
        let tokens = vec![Token::Literal(Value::Nat(42))];
        let result = inferer.infer_word_type(&tokens).unwrap();

        assert_eq!(result.inputs, vec![]);
        assert_eq!(result.outputs, vec![Type::Nat]);
    }

    #[test]
    fn test_infer_stack_ops() {
        let mut inferer = TypeInferer::new();
        let tokens = vec![Token::Word("dup".to_string())];
        let result = inferer.infer_word_type(&tokens).unwrap();

        assert_eq!(result.inputs.len(), 1);
        assert_eq!(result.outputs.len(), 2);
        // Should be T -> T T for some type T
        if let (Type::Variable(v1), Type::Variable(v2), Type::Variable(v3)) =
            (&result.inputs[0], &result.outputs[0], &result.outputs[1])
        {
            assert_eq!(v1, v2);
            assert_eq!(v1, v3);
        }
    }
}
