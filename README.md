## Kaban
Kaban is a compiled systems language bootstrapped in Rust designed with manual memory management and compile-time safety in  mind. Further specifications will be released once the design foundation is solidified.

## Design Goals
- WYSIWYG - honest about what code does, no hidden costs
- C's clarity without C's footguns
- Manual memory management with an invalidation-based ownership model (heavy inspiration from Rust's borrow checker)
- Target audience: kernel devs, graphics programmers, systems engineers

## Current Status
- Lexer (Complete ✅) - DOD span-based, zero allocation
- Parser (In Progress 🏗️) - Pratt parser, DOD SoA AST. Active development on the [parser branch](../../tree/parser)
- Type Checker (Pending)
- IR / Codegen (Pending)

## Parser Architecture
The parser uses a Data-Oriented Design (DOD) flat array AST inspired heavily by Zig's compiler architecture. Rather than allocating individual heap nodes connected by pointers (the traditional recursive tree approach), all AST data is stored in three flat arrays:

- **Tag array** - a tightly packed enum (`u8`) array of node types, one per node
- **Data array** - synced with tags, each entry holds two `u32` indices (`left`, `right`) whose meaning is defined by the corresponding tag
- **Extra array** - overflow storage for nodes that require more than two children, referenced by index from the data array

This layout keeps hot data (node tags) contiguous in memory, improving cache performance during compiler passes that only need to inspect node types. Variable length data (function arguments, struct fields, match arms) is stored in the extra array and referenced by range indices rather than heap-allocated `Vec`s.

Expressions are parsed using a Pratt parser, which handles the full precedence hierarchy including arithmetic, logical, bitwise, comparison, 
casting, prefix and postfix operators.

**Currently parsing:**
- Expressions: arithmetic, logical, bitwise, member access, method calls, indexing, type casting, array literals, if/match/while/for/do-while
- Pattern matching: tuple, array, and struct destructuring (nested, with mutability)
- Statements: let bindings, assignments, compound assignments, return/pass/break/continue
- Error recovery with recovery points (Bug found see [issue #3](../../issues/3))


## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

Any contribution submitted for inclusion in this project shall be dual-licensed as above, without any additional terms or conditions.
