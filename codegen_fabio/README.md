# About

The original Ruby parser uses hard-coded markers to identify the C header
components and handles those appropriately. I tried to create a parser which
makes less assumptions and tries to be as generic as possible by identifying the C
syntax to parse the data into the appropriate types. For example, it can handle
multiline function declarations, etc (see `src/tests`). Of course, hardcoded
markers can still be implemented for precise code generation (such as
`_Nonnull`, etc).

The parser is **not done yet**. Currently, it primarily handles some function
decelerations and generates some Rust interfaces. However, not all C types are
handled yet (neither are consts/pointers), but it can already partially generate
output by simply ignoring unhandled data (including typedefs, includes, etc).
The required structure for processing all of this is there.

Try it:

```bash
cargo run --bin parser -- ../include/TrustWalletCore/TWString.h
cargo run --bin parser -- ../include/TrustWalletCore/TWBitcoinScript.h
cargo run --bin parser -- ../include/TrustWalletCore/TWHDWallet.h
...
```

Generally, there are quite a bit of `TODO`-s and polishing work to do,
especially regarding configuration, CLI interface, error handling, some
auxiliary data which might be useful for debugging, etc. But the current work
shows the general architecture.

