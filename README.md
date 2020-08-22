# String interner

String interner based on [Rust's interner](https://github.com/rust-lang/rust/blob/5528caf91442729b47b8f818be0a0ddfd0c17ffb/src/librustc_span/symbol.rs#L1368).

Interning a string returns a `Symbol`, which is just a `u32`. This allows a faster comparison between strings, as it is only a case of comparing two integers. Strings are interned in a [thread local](https://github.com/ErickHdez96/string-interner/blob/04afbaa84aa8af1962b8bd28caf71f0162eba01b/src/lib.rs#L116) variable, meaning that all the interned strings are deallocated until the end of the program (and it is not yet multi-thread friendly).

## Usage

```rust
use string_interner::Symbol;

fn main() {
    // Use `Symbol::intern` to intern a string ant get a Symbol.
    let symbol = Symbol::intern("Hello, world!");
    // From<&str> is also implemented for Symbol.
    let symbol_2: Symbol = "Hello, world!".into();
    // And From<String>.
    let symbol_3: Symbol = String::from("Bye, world!").into();

    // Comparing two Symbols is just a comparison between two `u32`s
    assert_eq!(symbol, symbol_2);
    assert_eq!(symbol.as_str(), symbol_2.as_str());
    assert_eq!(symbol.as_str(), "Hello, world!");

    assert_ne!(symbol_2, symbol_3);
    assert_ne!(symbol_2.as_str(), symbol_3.as_str());
    assert_eq!(symbol_3.as_str(), "Bye, world!");
}
```
