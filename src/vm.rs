//! Virtual Machine for C∀O (Kao) execution
//!
//! This module implements the stack-based virtual machine that executes C∀O code.
//! It follows the concatenative programming paradigm with postfix notation.

use chronos_core::{
    MatchArm, OrdinalValue, Pattern, Token, Type, TypeSignature, Value, WordDefinition,
};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during VM execution
#[derive(Debug, Clone)]
pub enum VmError {
    /// Stack underflow - not enough values on stack
    StackUnderflow(String),
    /// Type mismatch
    TypeMismatch { expected: String, found: String },
    /// Unknown word
    UnknownWord(String),
    /// Division by zero
    DivisionByZero,
    /// Invalid operation
    InvalidOperation(String),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow(op) => write!(f, "Stack underflow in operation: {}", op),
            VmError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            VmError::UnknownWord(word) => write!(f, "Unknown word: {}", word),
            VmError::DivisionByZero => write!(f, "Division by zero"),
            VmError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for VmError {}

/// The virtual machine state
pub struct VirtualMachine {
    /// The data stack
    stack: Vec<Value>,
    /// Dictionary of defined words
    dictionary: HashMap<String, WordDefinition>,
    /// Type signatures for words
    type_signatures: HashMap<String, TypeSignature>,
    /// Call stack for tracking recursion depth
    call_stack: Vec<String>,
    /// Maximum recursion depth
    max_recursion_depth: usize,
}

impl VirtualMachine {
    /// Create a new virtual machine with core library
    pub fn new() -> Self {
        let mut vm = VirtualMachine {
            stack: Vec::new(),
            dictionary: HashMap::new(),
            type_signatures: HashMap::new(),
            call_stack: Vec::new(),
            max_recursion_depth: 1000,
        };

        vm.load_core_library();
        vm
    }

    /// Load the core library (Genesis Axioms)
    fn load_core_library(&mut self) {
        // Stack manipulation words
        self.define_builtin(
            "dup",
            vec![Type::Variable("a".to_string())],
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("a".to_string()),
            ],
        );
        self.define_builtin("drop", vec![Type::Variable("a".to_string())], vec![]);
        self.define_builtin(
            "swap",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![
                Type::Variable("b".to_string()),
                Type::Variable("a".to_string()),
            ],
        );
        self.define_builtin(
            "over",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
            ],
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
                Type::Variable("a".to_string()),
            ],
        );
        self.define_builtin(
            "rot",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("b".to_string()),
                Type::Variable("c".to_string()),
            ],
            vec![
                Type::Variable("b".to_string()),
                Type::Variable("c".to_string()),
                Type::Variable("a".to_string()),
            ],
        );

        // Arithmetic words
        self.define_builtin("+", vec![Type::Nat, Type::Nat], vec![Type::Nat]);
        self.define_builtin("-", vec![Type::Nat, Type::Nat], vec![Type::Nat]);
        self.define_builtin("*", vec![Type::Nat, Type::Nat], vec![Type::Nat]);
        self.define_builtin("/", vec![Type::Nat, Type::Nat], vec![Type::Nat]);

        // Comparison words
        self.define_builtin(
            "=",
            vec![
                Type::Variable("a".to_string()),
                Type::Variable("a".to_string()),
            ],
            vec![Type::Bool],
        );
        self.define_builtin("<", vec![Type::Nat, Type::Nat], vec![Type::Bool]);
        self.define_builtin(">", vec![Type::Nat, Type::Nat], vec![Type::Bool]);

        // Control flow
        self.define_builtin("if", vec![Type::Bool, Type::Quote, Type::Quote], vec![]);

        // System words
        self.define_builtin(".", vec![Type::Variable("a".to_string())], vec![]);
        self.define_builtin(".s", vec![], vec![]);
        self.define_builtin("--ordinal", vec![Type::Quote], vec![Type::Ordinal]);
    }

    /// Define a builtin word with its type signature
    fn define_builtin(&mut self, name: &str, inputs: Vec<Type>, outputs: Vec<Type>) {
        let signature = TypeSignature { inputs, outputs };
        self.type_signatures.insert(name.to_string(), signature);
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Value, VmError> {
        self.stack
            .pop()
            .ok_or_else(|| VmError::StackUnderflow("pop".to_string()))
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<&Value> {
        self.stack.last()
    }

    /// Get the current stack contents
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    /// Get the current stack (alternative name for compatibility)
    pub fn get_stack(&self) -> &[Value] {
        &self.stack
    }

    /// Get access to the dictionary
    pub fn get_dictionary(&self) -> &HashMap<String, WordDefinition> {
        &self.dictionary
    }

    /// Clear the stack
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    /// Define a new word
    pub fn define_word(&mut self, word_def: WordDefinition) {
        self.type_signatures
            .insert(word_def.name.clone(), word_def.signature.clone());
        self.dictionary.insert(word_def.name.clone(), word_def);
    }

    /// Execute a single token
    pub fn execute_token(&mut self, token: &Token) -> Result<(), VmError> {
        match token {
            Token::Literal(value) => {
                self.push(value.clone());
                Ok(())
            }
            Token::Word(word) => self.execute_word(word),
            Token::MatchExpression { value, arms } => self.execute_match_expression(value, arms),
            _ => Err(VmError::InvalidOperation(format!(
                "Cannot execute token: {:?}",
                token
            ))),
        }
    }

    /// Execute a word (builtin or user-defined)
    pub fn execute_word(&mut self, word: &str) -> Result<(), VmError> {
        // Check recursion depth
        if self.call_stack.len() >= self.max_recursion_depth {
            return Err(VmError::InvalidOperation(format!(
                "Maximum recursion depth exceeded in word: {}",
                word
            )));
        }

        self.call_stack.push(word.to_string());
        let result = self.execute_word_impl(word);
        self.call_stack.pop();
        result
    }

    /// Internal implementation of word execution
    fn execute_word_impl(&mut self, word: &str) -> Result<(), VmError> {
        match word {
            // Stack manipulation
            "dup" => self.builtin_dup(),
            "drop" => self.builtin_drop(),
            "swap" => self.builtin_swap(),
            "over" => self.builtin_over(),
            "rot" => self.builtin_rot(),

            // Arithmetic
            "+" => self.builtin_add(),
            "-" => self.builtin_sub(),
            "*" => self.builtin_mul(),
            "/" => self.builtin_div(),

            // Comparison
            "=" => self.builtin_eq(),
            "<" => self.builtin_lt(),
            ">" => self.builtin_gt(),

            // Control flow
            "if" => self.builtin_if(),

            // System
            "." => self.builtin_dot(),
            ".s" => self.builtin_dot_s(),
            "help" => self.builtin_help(),
            "words" => self.builtin_words(),
            "--ordinal" => self.builtin_ordinal(),

            // Polymorphic type constructors
            "Some" => self.builtin_some(),
            "None" => self.builtin_none(),
            "Ok" => self.builtin_ok(),
            "Err" => self.builtin_err(),
            "list" => self.builtin_list(),
            "test-pattern" => self.test_pattern_matching(),

            // User-defined words
            _ => {
                if let Some(word_def) = self.dictionary.get(word).cloned() {
                    if word_def.is_axiom {
                        return Err(VmError::InvalidOperation(format!(
                            "Cannot execute axiom: {}",
                            word
                        )));
                    }
                    self.execute_tokens(&word_def.body)
                } else {
                    Err(VmError::UnknownWord(word.to_string()))
                }
            }
        }
    }

    /// Execute a sequence of tokens
    pub fn execute_tokens(&mut self, tokens: &[Token]) -> Result<(), VmError> {
        for token in tokens {
            self.execute_token(token)?;
        }
        Ok(())
    }

    // Builtin word implementations

    fn builtin_dup(&mut self) -> Result<(), VmError> {
        let value = self
            .peek()
            .ok_or_else(|| VmError::StackUnderflow("dup".to_string()))?;
        let value = value.clone();
        self.push(value);
        Ok(())
    }

    fn builtin_drop(&mut self) -> Result<(), VmError> {
        self.pop()?;
        Ok(())
    }

    fn builtin_swap(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(b);
        self.push(a);
        Ok(())
    }

    fn builtin_over(&mut self) -> Result<(), VmError> {
        if self.stack.len() < 2 {
            return Err(VmError::StackUnderflow("over".to_string()));
        }
        let second = self.stack[self.stack.len() - 2].clone();
        self.push(second);
        Ok(())
    }

    fn builtin_rot(&mut self) -> Result<(), VmError> {
        if self.stack.len() < 3 {
            return Err(VmError::StackUnderflow("rot".to_string()));
        }
        let c = self.pop()?;
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(b);
        self.push(c);
        self.push(a);
        Ok(())
    }

    fn builtin_add(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                self.push(Value::Nat(x + y));
                Ok(())
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_sub(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                if x >= y {
                    self.push(Value::Nat(x - y));
                    Ok(())
                } else {
                    Err(VmError::InvalidOperation(
                        "Cannot subtract to negative result".to_string(),
                    ))
                }
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_mul(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                self.push(Value::Nat(x * y));
                Ok(())
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_div(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                if y == 0 {
                    Err(VmError::DivisionByZero)
                } else {
                    self.push(Value::Nat(x / y));
                    Ok(())
                }
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_eq(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        let result = a == b;
        self.push(Value::Bool(result));
        Ok(())
    }

    fn builtin_lt(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                self.push(Value::Bool(x < y));
                Ok(())
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_gt(&mut self) -> Result<(), VmError> {
        let b = self.pop()?;
        let a = self.pop()?;
        match (a, b) {
            (Value::Nat(x), Value::Nat(y)) => {
                self.push(Value::Bool(x > y));
                Ok(())
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Nat Nat".to_string(),
                found: "other types".to_string(),
            }),
        }
    }

    fn builtin_if(&mut self) -> Result<(), VmError> {
        let else_quote = self.pop()?;
        let then_quote = self.pop()?;
        let condition = self.pop()?;

        match condition {
            Value::Bool(true) => {
                if let Value::Quote(tokens) = then_quote {
                    self.execute_tokens(&tokens)
                } else {
                    Err(VmError::TypeMismatch {
                        expected: "Quote".to_string(),
                        found: "other type".to_string(),
                    })
                }
            }
            Value::Bool(false) => {
                if let Value::Quote(tokens) = else_quote {
                    self.execute_tokens(&tokens)
                } else {
                    Err(VmError::TypeMismatch {
                        expected: "Quote".to_string(),
                        found: "other type".to_string(),
                    })
                }
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Bool".to_string(),
                found: "other type".to_string(),
            }),
        }
    }

    fn builtin_dot(&mut self) -> Result<(), VmError> {
        let value = self.pop()?;
        println!("{}", value);
        Ok(())
    }

    fn builtin_dot_s(&mut self) -> Result<(), VmError> {
        print!("<{}> ", self.stack.len());
        for value in &self.stack {
            print!("{} ", value);
        }
        println!();
        Ok(())
    }

    fn builtin_ordinal(&mut self) -> Result<(), VmError> {
        let quote = self.pop()?;
        match quote {
            Value::Quote(_tokens) => {
                // For now, just return a simple ordinal
                // In a full implementation, this would analyze the quote for termination
                self.push(Value::Ordinal(OrdinalValue::Finite(1)));
                Ok(())
            }
            _ => Err(VmError::TypeMismatch {
                expected: "Quote".to_string(),
                found: "other type".to_string(),
            }),
        }
    }

    /// Get a word definition from the dictionary
    pub fn get_word_definition(&self, name: &str) -> Option<&WordDefinition> {
        self.dictionary.get(name)
    }

    /// Get all word definitions
    pub fn get_all_word_definitions(&self) -> &HashMap<String, WordDefinition> {
        &self.dictionary
    }

    /// Create Some(value) from top stack value
    fn builtin_some(&mut self) -> Result<(), VmError> {
        let value = self.pop()?;
        self.push(Value::Option(Some(Box::new(value))));
        Ok(())
    }

    /// Create None value
    fn builtin_none(&mut self) -> Result<(), VmError> {
        self.push(Value::Option(None));
        Ok(())
    }

    /// Create Ok(value) from top stack value
    fn builtin_ok(&mut self) -> Result<(), VmError> {
        let value = self.pop()?;
        self.push(Value::Result(Ok(Box::new(value))));
        Ok(())
    }

    /// Create Err(value) from top stack value
    fn builtin_err(&mut self) -> Result<(), VmError> {
        let value = self.pop()?;
        self.push(Value::Result(Err(Box::new(value))));
        Ok(())
    }

    /// Create List from top n stack values (where n is on top of stack)
    fn builtin_list(&mut self) -> Result<(), VmError> {
        let count = self.pop()?;
        if let Value::Nat(n) = count {
            let mut values = Vec::new();
            for _ in 0..n {
                values.push(self.pop()?);
            }
            values.reverse(); // Since we popped in reverse order
            self.push(Value::List(values));
            Ok(())
        } else {
            Err(VmError::TypeMismatch {
                expected: "Nat".to_string(),
                found: "other type".to_string(),
            })
        }
    }

    /// Execute a match expression
    pub fn execute_match_expression(
        &mut self,
        value_token: &Token,
        arms: &[MatchArm],
    ) -> Result<(), VmError> {
        // First execute the value expression to get the value to match
        self.execute_token(value_token)?;
        let value = self.pop()?;

        // Try each arm in order
        for arm in arms {
            if let Some(bindings) = self.match_pattern(&arm.pattern, &value)? {
                // Pattern matched - push bindings to stack and execute body
                for (_var_name, bound_value) in bindings {
                    self.push(bound_value);
                }
                return self.execute_tokens(&arm.body);
            }
        }

        Err(VmError::InvalidOperation("No pattern matched".to_string()))
    }

    /// Check if a pattern matches a value, returning bindings if successful
    pub fn match_pattern(
        &mut self,
        pattern: &Pattern,
        value: &Value,
    ) -> Result<Option<Vec<(String, Value)>>, VmError> {
        match pattern {
            Pattern::Wildcard => {
                // Wildcard matches anything
                Ok(Some(vec![]))
            }
            Pattern::Variable(name) => {
                // Variable matches anything and binds the value
                Ok(Some(vec![(name.clone(), value.clone())]))
            }
            Pattern::Literal(lit_value) => {
                // Literal matches if values are equal
                if value == lit_value {
                    Ok(Some(vec![]))
                } else {
                    Ok(None)
                }
            }
            Pattern::Constructor { name, args } => {
                // Match constructor patterns like Some(x), Ok(y), etc.
                match (name.as_str(), value) {
                    ("Some", Value::Option(Some(inner_value))) => {
                        if args.len() == 1 {
                            self.match_pattern(&args[0], inner_value)
                        } else {
                            Ok(None)
                        }
                    }
                    ("None", Value::Option(None)) => {
                        if args.is_empty() {
                            Ok(Some(vec![]))
                        } else {
                            Ok(None)
                        }
                    }
                    ("Ok", Value::Result(Ok(inner_value))) => {
                        if args.len() == 1 {
                            self.match_pattern(&args[0], inner_value)
                        } else {
                            Ok(None)
                        }
                    }
                    ("Err", Value::Result(Err(error_value))) => {
                        if args.len() == 1 {
                            self.match_pattern(&args[0], error_value)
                        } else {
                            Ok(None)
                        }
                    }
                    _ => Ok(None),
                }
            }
            Pattern::List(patterns) => {
                // Match list patterns
                if let Value::List(values) = value {
                    if patterns.len() != values.len() {
                        return Ok(None);
                    }

                    let mut all_bindings = vec![];
                    for (pattern, value) in patterns.iter().zip(values.iter()) {
                        if let Some(bindings) = self.match_pattern(pattern, value)? {
                            all_bindings.extend(bindings);
                        } else {
                            return Ok(None);
                        }
                    }
                    Ok(Some(all_bindings))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Create a simple test pattern matching expression for demonstration
    pub fn test_pattern_matching(&mut self) -> Result<(), VmError> {
        // Test 1: Create Some(42) and match it
        self.push(Value::Nat(42));
        self.builtin_some()?;

        // Create a simple match expression manually
        let some_pattern = Pattern::Constructor {
            name: "Some".to_string(),
            args: vec![Pattern::Variable("x".to_string())],
        };
        let none_pattern = Pattern::Constructor {
            name: "None".to_string(),
            args: vec![],
        };

        let arms = vec![
            MatchArm {
                pattern: some_pattern,
                body: vec![Token::Word("dup".to_string()), Token::Word("+".to_string())],
            },
            MatchArm {
                pattern: none_pattern,
                body: vec![Token::Literal(Value::Nat(0))],
            },
        ];

        let value_token = Token::Literal(self.pop()?);
        self.execute_match_expression(&value_token, &arms)?;

        println!("Pattern matching test completed");
        Ok(())
    }

    fn builtin_help(&mut self) -> Result<(), VmError> {
        println!("C∀O Core Library (Genesis Axioms)");
        println!("==================================\n");

        println!("Stack Manipulation:");
        println!("  dup     ( a -> a a )        Duplicate top element");
        println!("  drop    ( a -> )            Remove top element");
        println!("  swap    ( a b -> b a )      Exchange top two");
        println!("  over    ( a b -> a b a )    Copy second to top");
        println!("  rot     ( a b c -> b c a )  Rotate three elements\n");

        println!("Arithmetic:");
        println!("  +       ( Nat Nat -> Nat )  Addition");
        println!("  -       ( Nat Nat -> Nat )  Subtraction");
        println!("  *       ( Nat Nat -> Nat )  Multiplication");
        println!("  /       ( Nat Nat -> Nat )  Division\n");

        println!("Comparison:");
        println!("  =       ( a a -> Bool )     Equality test");
        println!("  <       ( Nat Nat -> Bool ) Less than");
        println!("  >       ( Nat Nat -> Bool ) Greater than\n");

        println!("Control Flow:");
        println!("  if      ( Bool Quote Quote -> ) Conditional");

        println!("System:");
        println!("  .       ( a -> )            Print value");
        println!("  .s      ( -> )              Print stack");
        println!("  words   ( -> )              List words");
        println!("  help    ( -> )              Show this help");
        println!("  --ordinal ( Quote -> Ordinal ) Ordinal cost\n");

        println!("Polymorphic Constructors:");
        println!("  Some    ( a -> Option<a> )  Create Some value");
        println!("  None    ( -> Option<a> )    Create None value");
        println!("  Ok      ( a -> Result<a,b> ) Create Ok value");
        println!("  Err     ( b -> Result<a,b> ) Create Err value\n");

        println!("Type '.help' for REPL commands");
        Ok(())
    }

    fn builtin_words(&mut self) -> Result<(), VmError> {
        println!("Available words:");
        println!("===============\n");

        println!("Core words (built-in):");
        let mut core_words = vec![
            "dup",
            "drop",
            "swap",
            "over",
            "rot",
            "+",
            "-",
            "*",
            "/",
            "=",
            "<",
            ">",
            "if",
            ".",
            ".s",
            "help",
            "words",
            "--ordinal",
            "Some",
            "None",
            "Ok",
            "Err",
            "list",
            "test-pattern",
        ];
        core_words.sort();

        for (i, word) in core_words.iter().enumerate() {
            if i % 6 == 0 && i > 0 {
                println!();
            }
            print!("{:<12}", word);
        }
        println!("\n");

        if !self.dictionary.is_empty() {
            println!("User-defined words:");
            let mut user_words: Vec<_> = self.dictionary.keys().collect();
            user_words.sort();

            for (i, word) in user_words.iter().enumerate() {
                if i % 6 == 0 && i > 0 {
                    println!();
                }
                print!("{:<12}", word);
            }
            println!();
        }

        println!(
            "\nTotal: {} core words, {} user-defined words",
            core_words.len(),
            self.dictionary.len()
        );
        Ok(())
    }
}
