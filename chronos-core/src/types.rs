//! Core type system for C∀O (Kao)
//!
//! This module defines the fundamental types and values in the C∀O language,
//! following categorical foundations where types are objects and functions are morphisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// The core value types in C∀O
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// Terminal object - represents a single value
    Unit,
    /// Boolean values
    Bool(bool),
    /// Natural numbers (non-negative integers)
    Nat(u64),
    /// String values
    String(String),
    /// Ordinal values for proof-theoretic analysis
    Ordinal(OrdinalValue),
    /// Quoted code blocks (code as data)
    Quote(Vec<Token>),
    /// User-defined composite types
    Composite {
        type_name: String,
        fields: HashMap<String, Value>,
    },
    /// Option type - Some(value) or None
    Option(Option<Box<Value>>),
    /// Result type - Ok(value) or Err(error)
    Result(Result<Box<Value>, Box<Value>>),
    /// List type - ordered collection of values
    List(Vec<Value>),
}

/// Ordinal values for termination analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrdinalValue {
    /// Zero ordinal
    Zero,
    /// Finite ordinal (natural number)
    Finite(u64),
    /// Omega (first transfinite ordinal)
    Omega,
    /// Omega raised to a power
    OmegaPower(Box<OrdinalValue>),
    /// Sum of ordinals
    Sum(Vec<OrdinalValue>),
    /// Product of ordinals
    Product(Box<OrdinalValue>, Box<OrdinalValue>),
}

/// Tokens representing parsed C∀O code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    /// A literal value
    Literal(Value),
    /// A word (function name)
    Word(String),
    /// Start of a quotation
    QuoteStart,
    /// End of a quotation
    QuoteEnd,
    /// A comment
    Comment(String),
    /// Pattern matching expression
    MatchExpression {
        value: Box<Token>,
        arms: Vec<MatchArm>,
    },
}

/// A single arm of a match expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Token>,
}

/// Pattern matching patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard,
    /// Variable pattern (binds to any value)
    Variable(String),
    /// Literal pattern (matches exact value)
    Literal(Value),
    /// Constructor pattern (Some(x), Ok(y), etc.)
    Constructor { name: String, args: Vec<Pattern> },
    /// List pattern [a, b, c]
    List(Vec<Pattern>),
}

/// Type signatures for categorical type checking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeSignature {
    /// Input types (stack effect - what's consumed)
    pub inputs: Vec<Type>,
    /// Output types (stack effect - what's produced)
    pub outputs: Vec<Type>,
}

/// The type system representing categorical objects
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Terminal object type
    Unit,
    /// Boolean type
    Bool,
    /// Natural number type
    Nat,
    /// String type
    String,
    /// Ordinal type for proof analysis
    Ordinal,
    /// Quotation type (code blocks)
    Quote,
    /// User-defined composite type
    Composite {
        name: String,
        fields: HashMap<String, Type>,
    },
    /// Type variable for polymorphism
    Variable(String),
    /// Option type - Option<T>
    Option(Box<Type>),
    /// Result type - Result<T, E>
    Result(Box<Type>, Box<Type>),
    /// List type - List<T>
    List(Box<Type>),
}

/// Word (function) definition in C∀O
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDefinition {
    /// The name of the word
    pub name: String,
    /// Type signature
    pub signature: TypeSignature,
    /// The implementation (sequence of tokens)
    pub body: Vec<Token>,
    /// Whether this is an axiom (no implementation)
    pub is_axiom: bool,
    /// Ordinal cost for termination analysis
    pub ordinal_cost: OrdinalValue,
}

/// User-defined type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// Name of the type
    pub name: String,
    /// Field definitions
    pub fields: HashMap<String, Type>,
    /// Constructor signature
    pub constructor_signature: TypeSignature,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nat(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Ordinal(ord) => write!(f, "{}", ord),
            Value::Quote(tokens) => {
                write!(f, "[ ")?;
                for token in tokens {
                    write!(f, "{} ", token)?;
                }
                write!(f, "]")
            }
            Value::Composite { type_name, fields } => {
                write!(f, "{}{{ ", type_name)?;
                for (key, value) in fields {
                    write!(f, "{}:{} ", key, value)?;
                }
                write!(f, "}}")
            }
            Value::Option(opt) => match opt {
                Some(value) => write!(f, "Some({})", value),
                None => write!(f, "None"),
            },
            Value::Result(res) => match res {
                Ok(value) => write!(f, "Ok({})", value),
                Err(error) => write!(f, "Err({})", error),
            },
            Value::List(values) => {
                write!(f, "[")?;
                for (i, value) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for OrdinalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrdinalValue::Zero => write!(f, "0"),
            OrdinalValue::Finite(n) => write!(f, "{}", n),
            OrdinalValue::Omega => write!(f, "ω"),
            OrdinalValue::OmegaPower(exp) => write!(f, "ω^{}", exp),
            OrdinalValue::Sum(ordinals) => {
                for (i, ord) in ordinals.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}", ord)?;
                }
                Ok(())
            }
            OrdinalValue::Product(a, b) => write!(f, "{} * {}", a, b),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Literal(value) => write!(f, "{}", value),
            Token::Word(name) => write!(f, "{}", name),
            Token::QuoteStart => write!(f, "["),
            Token::QuoteEnd => write!(f, "]"),
            Token::Comment(text) => write!(f, "( {} )", text),
            Token::MatchExpression { value, arms } => {
                write!(f, "match {} ", value)?;
                for arm in arms {
                    write!(f, "| {:?} -> ", arm.pattern)?;
                    for token in &arm.body {
                        write!(f, "{} ", token)?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => write!(f, "Unit"),
            Type::Bool => write!(f, "Bool"),
            Type::Nat => write!(f, "Nat"),
            Type::String => write!(f, "String"),
            Type::Ordinal => write!(f, "Ordinal"),
            Type::Quote => write!(f, "Quote"),
            Type::Composite { name, fields } => {
                write!(f, "{}{{ ", name)?;
                for (key, ty) in fields {
                    write!(f, "{}:{} ", key, ty)?;
                }
                write!(f, "}}")
            }
            Type::Variable(name) => write!(f, "{}", name),
            Type::Option(inner) => write!(f, "Option<{}>", inner),
            Type::Result(ok_type, err_type) => write!(f, "Result<{}, {}>", ok_type, err_type),
            Type::List(inner) => write!(f, "List<{}>", inner),
        }
    }
}
