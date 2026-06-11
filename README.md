# Kaban

Kaban is a compiled systems language bootstrapped in Rust designed with manual memory management and compile-time safety in  mind. Further language design specifications will be released once the design foundation is solidified.

## Design Goals

- **WYSIWYG**: what you write is what runs. No hidden allocations, no implicit copies, no runtime surprises
- **Manual memory management with compile-time safety**: Uses an invalidation-based ownership model
- **C's clarity without C's footguns**: explicit syntax, readable output
- **Target audience**: kernel devs, graphics devs, embedded engineers, people who need to know exactly what the machine is doing

## Current Status

| Phase | Status | Notes |
|---|---|---|
| Lexer | Complete | SoA layout, parallel kind/start/end arrays |
| Parser | Complete | DOD flat array AST, 190+ snapshot tests |
| HIR | In Progress | Zig ZIR-inspired, typeless, per-file flat instruction array |
| Type Checker | Planned | Produces typed AIR per function |
| Borrow Checker | Planned | Dataflow analysis over AIR, invalidation model |
| LLVM IR / Codegen | Planned | |

## Architecture

### Lexer - Structure of Arrays

The lexer outputs three parallel arrays instead of an array of `Token` structs:

```rust
pub struct TokenizedSource {
    pub kind: Vec<TokenKind>,
    pub start: Vec<UIndex>,
    pub end: Vec<UIndex>,
}
```

The parser's only reads `kind`. Start and end offsets live in separate allocations and are never loaded into cache during parsing. This is the same layout decision Zig's compiler uses with the only difference being that Kaban retrains `end` in a separate vec rather than dropping it. This eliminates the need for retokenization while preserving identical cache behavior.

### Parser - Data-Oriented Flat Array AST

The AST uses four parallel arrays instead of heap-allocated nodes connected by pointers:

```rust
node_tags:   Vec<NodeTag>        // u8 per node - the only array touched during most compiler passes
main_token:  Vec<TokenIndex>     // primary source token per node - drives error location and name recovery
node_data:   Vec<(u32, u32)>     // (lhs, rhs) operands - meaning determined by tag
extra:       Vec<u32>            // overflow storage for variable-length data
```

Extra parser info:

**No pointer chasing.** A tree traversal that only needs node types reads a single contiguous `u8` array. No cache misses from following heap pointers.

**No redundant name storage.** Declaration names (functions, structs, enums, impl blocks) are not stored in the AST. The grammar guarantees the name token always sits at `main_token + 1`, so it is recovered via index arithmetic. This was derived from studying Zig's compiler internals.

**Variable-length data without heap allocation.** Function arguments, struct fields, match arms, and generic parameters all live in `extra` as flat index ranges. No `Vec` per node.

**Tag-driven interpretation.** `lhs` and `rhs` are raw `u32` values. The tag determines whether they hold node indices, extra indices, token indices, or raw values. The same two-word slot serves all cases.

**Typed view layer**. Raw array access for complicated nodes (overflow data in extra) is encapsulated behind view functions that return typed structs (FuncCall, MethodCall, FuncDecl, etc.). Each view asserts the expected tag in debug builds and recovers all node fields without exposing the underlying layout to downstream compiler phases. Nodes that fit in lhs/rhs/main_token are accessed directly, no wrapper needed.

Expressions are parsed with a Pratt parser covering the full operator precedence hierarchy - arithmetic, logical, bitwise, comparison, casting, prefix and postfix operators, generic instantiation.

**Currently parsing:**

- Expressions: arithmetic, logical, bitwise, member access, method calls, generic instantiation, indexing, type casting, array literals, comptime blocks, anonymous functions, struct instantiation, if/match/while/for/do-while
- Types: primitive types, pointer/borrow/ownership modifiers, generics, union types, function types, optional and error propagation types
- Pattern matching: tuple, array, and struct destructuring (nested, with mutability), `is`/`to` binding patterns
- Declarations: functions, structs, enums, named impl blocks, interfaces with structural constraints, constants
- Statements: let bindings, assignments, compound assignments, return/pass/break/continue
- Error recovery with synchronization points

### Compiler Pipeline

```
Source -> Lexer -> AST -> HIR -> Sema (AIR) -> Borrow Checker -> LLVM IR -> Machine Code
```

- **HIR**: typeless, per-file, flat instruction array. Desugaring and local name resolution happen here
- **AIR**: typed, per-function, produced by Sema. Type inference, comptime evaluation, and generic instantiation happen heres
- **Borrow checker**: dataflow analysis pass over AIR. Tracks T* pointer liveness across control flow, flags use-after-free.

## Language Highlights

Note: These language design specifications are subject to change.

**Named impl blocks.** Every impl block has a unique name per type. There is no ambiguous method resolution.

```
impl Person::Core {
    func get_name(self&) -> String& { }
}
impl Person::Factory {
    func from_json(json: String&) -> Person* { }
}
```

**Explicit ownership at call sites.** Method mutability is encoded in the call operator, not just the receiver type.

```
person.method() // self& - immutable
person:method() // self&mut - mutable
```

**`pass` / `return` distinction.** `pass` exits the current block with a value. `return` exits the function. Blocks are expressions and their value is always explicit.

```
let x = {
    let y = compute();
    pass y + 20;
};
```

**Allocator-explicit memory.** No `new`/`delete` operators. Allocators are first-class values passed explicitly.

```
let buf = heap.alloc(type i32, 64)!;
heap.free(buf);
```

## Test Suite

The parser has 190+ snapshot tests using `cargo-insta`. Each snapshot captures the full AST output for a given input, making regressions immediately visible as structured diffs rather than failed assertions.

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

Any contribution submitted for inclusion in this project shall be dual-licensed as above, without any additional terms or conditions.
