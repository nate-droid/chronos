# C∀O Implementation Guide

**Purpose**: Step-by-step instructions for common development tasks in C∀O  
**Audience**: Developers implementing new features  
**Prerequisites**: Read PROJECT_STATUS.md, DEVELOPMENT_ROADMAP.md, and ARCHITECTURE_GUIDE.md  

## 🚀 Quick Start for New Features

### Before You Begin
1. **Environment Setup**
   ```bash
   cd chronos
   cargo build
   cargo run  # Test current functionality
   ```

2. **Verify Current State**
   ```cao
   C∀O> .help                    # Check available commands
   C∀O> .performance             # Baseline metrics
   C∀O> .save baseline           # Save clean state
   C∀O> quit
   ```

3. **Create Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🎯 Task 1: Implementing Type Inference (PRIORITY 1)

### Overview
Implement basic Hindley-Milner type inference to reduce boilerplate in word definitions.

### Step 1: Create Type Inference Module
Create `src/type_inference.rs`:

```rust
//! Type inference engine for C∀O
//! Implements Hindley-Milner algorithm for automatic type deduction

use crate::types::{Type, TypeSignature, Token, Value};
use std::collections::HashMap;
use std::fmt;

/// Type inference errors
#[derive(Debug, Clone)]
pub enum InferenceError {
    UnificationFailure(Type, Type),
    OccursCheck(String, Type),
    UnboundVariable(String),
    InsufficientInformation,
}

impl fmt::Display for InferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferenceError::UnificationFailure(t1, t2) => {
                write!(f, "Cannot unify types {:?} and {:?}", t1, t2)
            }
            InferenceError::OccursCheck(var, ty) => {
                write!(f, "Occurs check failed: {} occurs in {:?}", var, ty)
            }
            InferenceError::UnboundVariable(var) => {
                write!(f, "Unbound type variable: {}", var)
            }
            InferenceError::InsufficientInformation => {
                write!(f, "Insufficient information for type inference")
            }
        }
    }
}

/// Type constraint for unification
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub left: Type,
    pub right: Type,
}

/// Type inference engine
pub struct TypeInferer {
    /// Type variable substitutions
    substitutions: HashMap<String, Type>,
    /// Generated constraints
    constraints: Vec<TypeConstraint>,
    /// Next fresh type variable number
    next_var: usize,
}

impl TypeInferer {
    /// Create new type inference engine
    pub fn new() -> Self {
        TypeInferer {
            substitutions: HashMap::new(),
            constraints: Vec::new(),
            next_var: 0,
        }
    }

    /// Generate fresh type variable
    pub fn fresh_type_var(&mut self) -> Type {
        let var_name = format!("T{}", self.next_var);
        self.next_var += 1;
        Type::Variable(var_name)
    }

    /// Infer type signature for a sequence of tokens
    pub fn infer_word_type(&mut self, tokens: &[Token]) -> Result<TypeSignature, InferenceError> {
        // Reset state
        self.constraints.clear();
        self.substitutions.clear();

        // Start with fresh type variables for input/output
        let input_type = self.fresh_type_var();
        let output_type = self.fresh_type_var();

        // Analyze token sequence
        let mut current_stack_type = vec![input_type.clone()];
        
        for token in tokens {
            current_stack_type = self.infer_token_effect(token, current_stack_type)?;
        }

        // The final stack should match our output type
        if current_stack_type.len() == 1 {
            self.add_constraint(current_stack_type[0].clone(), output_type.clone());
        } else if current_stack_type.is_empty() {
            self.add_constraint(Type::Unit, output_type.clone());
        } else {
            return Err(InferenceError::InsufficientInformation);
        }

        // Solve constraints
        self.unify_constraints()?;

        // Apply substitutions
        let final_input = self.apply_substitution(&input_type);
        let final_output = self.apply_substitution(&output_type);

        Ok(TypeSignature {
            inputs: vec![final_input],
            outputs: vec![final_output],
        })
    }

    /// Infer the stack effect of a single token
    fn infer_token_effect(&mut self, token: &Token, stack: Vec<Type>) -> Result<Vec<Type>, InferenceError> {
        match token {
            Token::Literal(Value::Nat(_)) => {
                let mut new_stack = stack;
                new_stack.push(Type::Nat);
                Ok(new_stack)
            }
            Token::Literal(Value::Bool(_)) => {
                let mut new_stack = stack;
                new_stack.push(Type::Bool);
                Ok(new_stack)
            }
            Token::Word(word) => {
                self.infer_word_effect(word, stack)
            }
            _ => Err(InferenceError::InsufficientInformation),
        }
    }

    /// Infer the effect of a built-in word
    fn infer_word_effect(&mut self, word: &str, mut stack: Vec<Type>) -> Result<Vec<Type>, InferenceError> {
        match word {
            "dup" => {
                if let Some(top) = stack.last() {
                    stack.push(top.clone());
                    Ok(stack)
                } else {
                    Err(InferenceError::InsufficientInformation)
                }
            }
            "drop" => {
                if !stack.is_empty() {
                    stack.pop();
                    Ok(stack)
                } else {
                    Err(InferenceError::InsufficientInformation)
                }
            }
            "+" | "-" | "*" | "/" => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    self.add_constraint(a, Type::Nat);
                    self.add_constraint(b, Type::Nat);
                    stack.push(Type::Nat);
                    Ok(stack)
                } else {
                    Err(InferenceError::InsufficientInformation)
                }
            }
            _ => Err(InferenceError::InsufficientInformation),
        }
    }

    /// Add a constraint between two types
    fn add_constraint(&mut self, left: Type, right: Type) {
        self.constraints.push(TypeConstraint { left, right });
    }

    /// Unify all constraints using Robinson's algorithm
    fn unify_constraints(&mut self) -> Result<(), InferenceError> {
        for constraint in self.constraints.clone() {
            self.unify(&constraint.left, &constraint.right)?;
        }
        Ok(())
    }

    /// Unify two types
    fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), InferenceError> {
        let t1 = self.apply_substitution(t1);
        let t2 = self.apply_substitution(t2);

        match (&t1, &t2) {
            // Same types unify
            (Type::Nat, Type::Nat) | (Type::Bool, Type::Bool) | (Type::Unit, Type::Unit) => Ok(()),
            
            // Variable unification
            (Type::Variable(var), ty) | (ty, Type::Variable(var)) => {
                if let Type::Variable(other_var) = ty {
                    if var == other_var {
                        return Ok(());
                    }
                }
                
                // Occurs check
                if self.occurs_check(var, ty) {
                    return Err(InferenceError::OccursCheck(var.clone(), ty.clone()));
                }
                
                self.substitutions.insert(var.clone(), ty.clone());
                Ok(())
            }
            
            // Mismatch
            _ => Err(InferenceError::UnificationFailure(t1, t2)),
        }
    }

    /// Check if variable occurs in type (prevents infinite types)
    fn occurs_check(&self, var: &str, ty: &Type) -> bool {
        match ty {
            Type::Variable(other_var) => var == other_var,
            Type::Composite { fields, .. } => {
                fields.values().any(|field_type| self.occurs_check(var, field_type))
            }
            _ => false,
        }
    }

    /// Apply current substitutions to a type
    fn apply_substitution(&self, ty: &Type) -> Type {
        match ty {
            Type::Variable(var) => {
                if let Some(substitution) = self.substitutions.get(var) {
                    self.apply_substitution(substitution)
                } else {
                    ty.clone()
                }
            }
            _ => ty.clone(),
        }
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
    use crate::types::Token;

    #[test]
    fn test_simple_arithmetic() {
        let mut inferer = TypeInferer::new();
        let tokens = vec![
            Token::Word("dup".to_string()),
            Token::Word("*".to_string()),
        ];
        
        let result = inferer.infer_word_type(&tokens);
        assert!(result.is_ok());
        // Should infer Nat -> Nat for square function
    }
}
```

### Step 2: Integrate with Parser
Modify `src/parser.rs` to support optional type signatures:

```rust
// Add this method to Parser impl
pub fn parse_word_definition_with_inference(&mut self) -> Result<WordDefinition, ParseError> {
    // Check if we have a pending type signature
    if let Some(signature) = self.pending_signatures.get(&word_name) {
        // Use explicit signature
        Ok(WordDefinition {
            name: word_name,
            signature: signature.clone(),
            body: tokens,
            is_axiom: false,
            ordinal_cost: OrdinalValue::Zero,
        })
    } else {
        // Try type inference
        let mut inferer = TypeInferer::new();
        let inferred_signature = inferer.infer_word_type(&tokens)
            .map_err(|e| ParseError::TypeInferenceError(e.to_string()))?;
        
        Ok(WordDefinition {
            name: word_name,
            signature: inferred_signature,
            body: tokens,
            is_axiom: false,
            ordinal_cost: OrdinalValue::Zero,
        })
    }
}
```

### Step 3: Add REPL Commands
In `src/repl.rs`, add new commands:

```rust
// Add to handle_repl_command match
Some(&"infer") => {
    if let Some(word_name) = parts.get(1) {
        self.show_inferred_type(word_name)?;
    } else {
        println!("Usage: .infer <word>");
    }
}
Some(&"type-debug") => {
    self.type_debug = !self.type_debug;
    println!(
        "Type debugging: {}",
        if self.type_debug { "ON" } else { "OFF" }
    );
}

// Add new methods
fn show_inferred_type(&self, word_name: &str) -> Result<(), ReplError> {
    if let Some(word_def) = self.vm.get_word_definition(word_name) {
        let mut inferer = TypeInferer::new();
        match inferer.infer_word_type(&word_def.body) {
            Ok(signature) => {
                println!("Inferred type for '{}': {:?}", word_name, signature);
            }
            Err(e) => {
                println!("Type inference failed for '{}': {}", word_name, e);
            }
        }
    } else {
        println!("Word '{}' not found", word_name);
    }
    Ok(())
}
```

### Step 4: Update Help System
Add to `.help` output:
```
Type System:
  .infer <word>    Show inferred type for word
  .type-debug      Toggle type inference debugging
```

### Step 5: Testing
Test the implementation:

```cao
C∀O> : double 2 * ;                    # Should work without explicit signature
C∀O> .infer double                     # Should show: Nat -> Nat
C∀O> : identity dup drop ;             # Should infer: T -> T
C∀O> .type-debug                       # Enable debugging
C∀O> : add3 3 + ;                      # Should show inference steps
```

### Step 6: Error Handling
Add comprehensive error handling:

```rust
// Add to ParseError enum
ParseError::TypeInferenceError(String),

// Update error display
ParseError::TypeInferenceError(msg) => {
    write!(f, "Type inference error: {}", msg)
}
```

## 🔧 Task 2: Adding New REPL Commands

### Template for New Commands
1. **Add to command handler**:
```rust
Some(&"newcommand") => {
    if let Some(arg) = parts.get(1) {
        self.handle_new_command(arg)?;
    } else {
        println!("Usage: .newcommand <argument>");
    }
}
```

2. **Implement handler method**:
```rust
fn handle_new_command(&mut self, arg: &str) -> Result<(), ReplError> {
    // Implementation here
    println!("New command executed with: {}", arg);
    Ok(())
}
```

3. **Update help**:
```rust
println!("  .newcommand <arg>    Description of new command");
```

4. **Test thoroughly**:
```cao
C∀O> .newcommand test
C∀O> .help                             # Verify help shows new command
```

## 🎨 Task 3: Adding New Built-in Words

### Step-by-Step Process
1. **Add to CoreLibrary** (`src/core_lib.rs`):
```rust
self.define_core_word(
    "new-word",
    vec![Type::Nat],                    # Input types
    vec![Type::Nat],                    # Output types
    "Description of new word"
);
```

2. **Implement in VM** (`src/vm.rs`):
```rust
fn builtin_new_word(&mut self) -> Result<(), VmError> {
    let value = self.pop()?;
    // Process value
    let result = process_value(value);
    self.push(result);
    Ok(())
}

// Add to execute_word match
"new-word" => self.builtin_new_word(),
```

3. **Test implementation**:
```cao
C∀O> 5 new-word
C∀O> .s                                # Verify result on stack
```

## 🧪 Testing Strategy

### Manual Testing Checklist
- [ ] Feature works as expected
- [ ] Error cases handled gracefully
- [ ] Help documentation updated
- [ ] Backward compatibility maintained
- [ ] Performance acceptable

### Automated Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        let mut repl = Repl::new();
        let result = repl.eval("test input");
        assert!(result.is_ok());
        // Add specific assertions
    }
}
```

### Performance Testing
```cao
C∀O> .benchmark "new-feature-code" 1000
# Verify performance meets expectations
```

### Session Testing
```cao
C∀O> # Test new feature
C∀O> .save test-session
C∀O> .reset
C∀O> .load test-session
C∀O> # Verify feature state restored
```

## 🚀 Deployment Checklist

### Before Committing
- [ ] Code compiles without errors
- [ ] All new features tested manually
- [ ] Help documentation updated
- [ ] Performance impact measured
- [ ] Backward compatibility verified
- [ ] Session save/load works with changes

### Documentation Updates
- [ ] Update PROJECT_STATUS.md
- [ ] Add feature to LANGUAGE_IMPROVEMENTS_IMPLEMENTED.md
- [ ] Update demo scripts if needed
- [ ] Add examples to guides

### Git Workflow
```bash
git add .
git commit -m "feat: implement [feature name]

- Add [specific changes]
- Update [affected components]
- Test [verification performed]"

git push origin feature/your-feature-name
# Create pull request
```

## 🔍 Debugging Common Issues

### Compilation Errors
1. **Missing imports**: Add required use statements
2. **Type mismatches**: Check type signatures and conversions
3. **Lifetime issues**: Review borrowing and ownership

### Runtime Errors
1. **Stack underflow**: Verify word consumes correct number of values
2. **Type errors**: Check type signatures match implementation
3. **Session errors**: Ensure new types have serde support

### Performance Issues
1. **Use `.benchmark`** to measure impact
2. **Profile with `.performance`** command
3. **Check memory usage** patterns

## 📋 Code Style Guidelines

### Rust Conventions
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and enums
- Add comprehensive documentation
- Handle all Result types explicitly

### C∀O Specific
- Maintain categorical foundations
- Preserve mathematical rigor
- Prioritize developer experience
- Ensure backward compatibility

### Error Handling
```rust
// Preferred pattern
match operation() {
    Ok(result) => process_result(result),
    Err(e) => return Err(ReplError::from(e)),
}

// Avoid unwrap() in production code
```

## 🎯 Success Criteria

### Feature Implementation
- [ ] Functionality works as specified
- [ ] Error handling is comprehensive
- [ ] Performance is acceptable
- [ ] Integration is seamless

### Quality Assurance
- [ ] Code follows style guidelines
- [ ] Documentation is complete
- [ ] Tests are comprehensive
- [ ] Backward compatibility maintained

### User Experience
- [ ] Feature is discoverable
- [ ] Error messages are helpful
- [ ] Performance is responsive
- [ ] Integration feels natural

---

**Remember**: Every change should enhance C∀O while preserving its mathematical foundations and categorical nature. When in doubt, prioritize correctness and user experience.

*"Good implementation is invisible to users but obvious to maintainers."*