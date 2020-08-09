//! String interning library
//!
//! # Examples
//!
//! ```
//! use string_interner::Symbol;
//!
//! let s = Symbol::intern("Hello, world!");
//! assert_eq!(s.as_str(), "Hello, world!");
//! let s2 = Symbol::intern("Hello, world!");
//! assert_eq!(s2.as_str(), "Hello, world!");
//! assert_eq!(s, s2);
//! ```
mod arena;

use arena::Arena;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(u32);

impl Symbol {
    /// Get the internal `u32` representation.
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// Intern a [`String`] and receive a Symbol that points to it.
    pub fn intern<S: AsRef<str>>(s: S) -> Self {
        with_interner(move |interner| interner.intern(s))
    }

    /// Get the string representation that this token points to.
    pub fn as_str(self) -> &'static str {
        with_interner(|interner| interner.symbol_to_str(self))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Symbol::intern(s)
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Symbol::intern(s)
    }
}

#[derive(Debug)]
struct Interner {
    map: HashMap<&'static str, u32>,
    strings: Vec<&'static str>,
    arena: Arena,
}

impl Interner {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            strings: Vec::new(),
            arena: Arena::new(),
        }
    }

    fn intern<S: AsRef<str>>(&mut self, s: S) -> Symbol {
        if let Some(idx) = self.map.get(s.as_ref()) {
            return Symbol(*idx);
        }

        let idx = self.strings.len();
        debug_assert!(
            idx <= (u32::MAX as usize),
            "Cannot intern more than {} strings",
            u32::MAX
        );
        let idx = idx as u32;
        let allocated_str: &'static str =
            unsafe { mem::transmute(self.arena.allocate_string(s.as_ref())) };
        self.strings.push(allocated_str);
        self.map.insert(allocated_str, idx);
        Symbol(idx)
    }

    fn symbol_to_str(&self, symbol: Symbol) -> &'static str {
        self.strings[symbol.as_u32() as usize]
    }
}

fn with_interner<F, T>(f: F) -> T
where
    F: FnOnce(&mut Interner) -> T,
{
    INTERNER.with(|interner| f(&mut interner.borrow_mut()))
}

thread_local! {
    static INTERNER: RefCell<Interner> = RefCell::new(Interner::new());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input1 = " ".repeat(4096);
        let s1 = Symbol::intern(&input1);
        assert_eq!(s1.as_str(), &input1);
        let input2 = "+".repeat(1);
        let s2 = Symbol::intern(&input2);
        assert_eq!(s2.as_str(), &input2);
        let input3 = "-".repeat(4097);
        let s3 = Symbol::intern(&input3);
        assert_eq!(s3.as_str(), &input3);

        assert_eq!(s1.as_str(), &input1);
        assert_eq!(s2.as_str(), &input2);
        assert_eq!(s3.as_str(), &input3);
    }

    #[test]
    fn test_simple_interning() {
        let s = Symbol::intern("Hello");
        assert_eq!(s.as_str(), "Hello");
    }

    #[test]
    fn test_interning_same_string_multiple_times() {
        let s1 = Symbol::intern("Hello, world");
        let s2 = Symbol::intern("Hello, world");
        assert_eq!(s1.as_str(), "Hello, world");
        assert_eq!(s1, s2);
        assert_eq!(s2.as_str(), "Hello, world");
    }

    #[test]
    fn test_interning_different_strings() {
        let s1: Symbol = "Hello, world".into();
        let s2: Symbol = "Hello, world".into();
        let s3: Symbol = "Hello, world!".into();
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_ne!(s2, s3);
        assert_eq!(s3.as_str(), "Hello, world!");
    }
}
