# Juan Syntax
This is the Juan language specification. (Mainly made to implement Juan properly)  
This does NOT help the average programmer or whatever, this is just for me to write everything about Juan and how it works inside to again, implement it well.

## 1. Naming Conventions

Juan diagnoses declarations that do not follow these case styles. Projects may promote the warnings to errors.

* **Modules**: `snake_case`  
* **Types**: `PascalCase`  
* **Functions & Variables**: `snake_case`  
* **Constants**: `SCREAMING_SNAKE_CASE`  
* **Primitives**: `lowercase`

---

## 2. Comments
* `//` — Single-line comment
* `/* ... */` — Multi-line block comment. These may be nested.
* `///` — Outer doc comment. Documents the declaration directly after it.
* `//!` — Inner doc comment. Documents the current module.
* `/** ... */` — Multi-line outer documentation block. Documents the declaration directly after it.

---

## 3. Modules, Packages & Visibility

Every `.juan` file declares exactly one module namespace. Module paths are independent of filenames and directories. A concrete module path may be declared by only one file in a package; its parent namespaces exist virtually and are importable without their own files.

### 3.1 Module Declaration
```juan
module my_module
```

The module declaration must be the first non-comment declaration in the file. Imports must come directly after it, before ordinary declarations.

### 3.2 Virtual Submodules

A dotted module path declares a submodule without requiring a concrete root module.

```juan
// File A
module foo.bar

// File B
module foo.idk
```

### 3.3 Importing Modules

Importing a module or virtual parent imports its namespace tree. Submodules remain qualified and are NOT injected as unqualified local names.

```juan
import foo

foo.bar.do_something()
foo.idk.do_something_else()
```

You can still import one exact submodule directly. The final part of that path becomes the local module name:

```juan
import foo.bar

bar.do_something()
```

You can rename the local module if needed:

```juan
import foo.bar as foo_bar

foo_bar.do_something()
```

#### Collision handling
If two imported modules would get the same local name, it is a compile error. At least one of them must use an explicit alias.

```juan
import mod_a.sub as sub_a
import mod_b.sub as sub_b

sub_a.do_something()
sub_b.do_something_else()
```

Import order never decides which declaration wins.

#### Selective Imports
A selective import directly imports a type, value, operator or macro:

```juan
import { Player } from game.player
import game.player

let player = player.new("Davey", 100.0)
```

Module imports remain qualified while selective imports create local names. Imports are private to the current module and never automatically re-export anything.

Types, values and macros use separate namespaces. An ambiguous use is a compile error that lists every candidate.

### 3.4 Packages & Dependencies

A package manifest lists source roots, binary entry points and dependency aliases.

The standard library owns the `std` root. A dependency's manifest alias becomes its import root:

```juan
import awesome_math.vector
```

Local roots, dependency aliases and `std` may not collide. The package-manifest format is outside Juan's source grammar.

### 3.5 Visibility

Declarations and fields are module-private by default.

* `pub` — Visible from other packages.
* `pub(package)` — Visible to every module in the same package, but not outside it.
* No modifier — Visible only inside the declaring module.

An import never increases the visibility of the declaration it names.

---

## 4. Control Flow

Control-flow constructs are expressions. Block forms use explicit `end` terminators; braces are reserved for records.

### 4.1 Block Values & Newlines

Juan does not use semicolons. A completed newline separates items in a block, and the final bare expression before `end` becomes the value of that block. `return` is only needed to return early.

```juan
fn calculate(): i32
    let base = 10
    let bonus = 5
    base + bonus
end
```

Earlier bare expressions are evaluated and discarded.

A block with no final expression evaluates to `Unit`. A function with no return type must produce `Unit`. To explicitly discard a non-`Unit` result in the final position, use `let _ = expression`.

Newlines inside `()`, `[]` or `{}` do not finish an expression. A newline also continues after an incomplete token such as an operator, comma, `=`, or `=>`. A line beginning with `|>` continues the previous pipeline, and a line beginning with `.` continues a postfix member or method chain.

```juan
let total = base +
    bonus

let result = input
    |> validate()
    |> transform()
```

Other than the special leading `|>` form, operators must end the line when continuing an expression.

### 4.2 Conditional Expressions

`if` expressions can be written as single-line expressions or multi-line blocks. Conditions MUST be `bool`; integers, strings and handles are not implicitly truthy.

#### Single-Line Expression
```juan
if true => "True" else => "False"
```

An inline `if` may not directly contain another unparenthesized inline `if`:

```juan
if a => (if b => 1 else => 2) else => 3
```

#### Multi-Line Block
```juan
let text = if true
    "True"
else
    "False"
end
```

When an `if` is used as a value it must have an `else`, and all reachable branches must produce compatible types. An `if` used only for its effects may omit the `else` and evaluates to `Unit`.

`else if` forms one conditional chain:

```juan
let text = if score >= 100
    "Amazing"
else if score >= 50
    "Good"
else
    "Try again"
end
```

### 4.3 Loops & Iteration

#### Infinite Loop (`loop`)
Executes a block indefinitely until broken using `break`. A `loop` may also produce a value using `break <expr>`.

```juan
let result = loop
    if ready()
        break calculate_result()
    end
end
```

Every reachable value-carrying `break` in the same loop must produce a compatible type. The repeating loop body itself must produce `Unit`; the value of the entire `loop` expression comes from `break <expr>`.

#### Conditional Loop (`while`)
Repeats a block as long as the boolean condition is `true`.

```juan
while true
    io.println("Still running")
end
```

`while` and its body must produce `Unit`, and `break` may not carry a value inside it.

#### Iteration (`for`)
Iterates over a range or a collection.

```juan
// 0, 1, 2, 3, 4
for i in 0..5
    print(i)
end

// 0, 1, 2, 3, 4, 5
for i in 0..=5
    print(i)
end

for item in inventory
    item.do_something()
end
```

`a..b` is a half-open range and excludes `b`. `a..=b` is inclusive. A `for` loop and its body must produce `Unit`, and `break` may not carry a value inside it.

#### Jump Statements
* `break`: Immediately breaks the current loop and may return a value only when used with `loop`.
* `continue`: Stops the current iteration of the loop and proceeds to the next cycle.
* `return`: Immediately exits the enclosing function and optionally returns a value.

```juan
for i in 0..10
    if i == 3 => continue
    if i == 8 => break
    io.println(i)
end
```

`break`, `continue` and `return` have the uninhabited type `Never`. `Never` may also be written as an ordinary return type for a function that never returns.

### 4.4 Match

`match` checks patterns from top to bottom and must be exhaustive unless the final reachable arm is `_`.

```juan
let description = match damage_type
    Normal => "Normal"
    Poison => "Poison"
    Fire => "Fire"
end
```

A multi-statement arm MUST use `do ... end`.

```juan
match state
    Flying => io.println("Flying")
    Fading { remaining } => do
        log(remaining)
        io.println(remaining)
    end
end
```

There is no arrow after the match value and arms do not use commas. Patterns may contain variants, record fields, literals, bindings and `_`. When a match is used as a value, all reachable arms must produce compatible types.

---

## 5. Functions

Each binary target has one entry function:

```juan
module main

import std.io

fn main()
    io.println("Hello, World!")
end
```

The package manifest chooses the entry-point module for each binary target. That module must contain exactly one compatible `fn main()`.

Functions are free functions organized into modules. Juan does not have classes, inheritance or hidden instance methods.

Functions have expression bodies or block bodies. A block body's value follows Section 4.1.

```juan
// Expression function
fn foo(): i32 => 8

// Block function
fn foo(): i32
    8
end
```

### 5.1 Parameter Access & Ownership

Function signatures explicitly publish how each parameter may be accessed. The compiler still infers the operations performed by the body and verifies that they do not exceed the published contract.

* No modifier (default) — Temporary read access.
* `mut` — Temporary exclusive read/write access that may update the caller's place.
* `take` — Ownership transfers into the function.

```juan
fn inspect(player: Player)
    io.println(player.name)
end

fn reset(mut player: Player)
    player.health = 100.0
end

fn upload(take pixels: Buffer<u8>): Texture
    create_texture(pixels)
end
```

A `mut` argument must be a mutable place. Passing an existing value for read or `mut` access does not consume it. A `take` argument becomes moved at the call site and cannot be used again unless reassigned.

Two overlapping places may not receive incompatible access during the same call. Multiple reads are allowed. A write may not overlap another live read or write.

### 5.2 Function Types

Functions, union constructors and closures may be values.

```juan
fn(i32): i32
fn(mut Player, f32)
fn(take Buffer<u8>): Texture
```

Parameter modes are part of the function type. For otherwise identical types:

* A read-only `fn(T)` may be used where `fn(mut T)` is accepted.
* A mutating `fn(mut T)` may NOT be used where `fn(T)` is required.
* `take` is invariant and only matches another `take` parameter.

The static function type controls the call. Calling a value typed `fn(mut Player)` still requires a mutable `Player` even if its current function happens to be read-only.

Union constructors are ordinary noncapturing function values:

```juan
type LoadError = Io(IoError) | Parse(ParseError)

let convert: fn(IoError): LoadError = LoadError.Io
```

### 5.3 Scoped & Escaping Functions

A plain function value is scoped and may not be stored or returned beyond its allowed region. `escape fn` is an owned callable that may be stored, returned or placed inside a GC-managed value.

```juan
fn(EventData)
escape fn(EventData)
```

An escaping callable may be used where a scoped callable is expected. A scoped callable may NOT be used where an escaping callable is required.

Named functions and union constructors may convert to `escape fn`.

### 5.4 Closures & Capture Modes

An anonymous `fn` expression creates a closure. Parameter types may be inferred when an expected function type is known.

```juan
let double: fn(i32): i32 = fn(value) => value * 2
```

Captures are listed explicitly:

* `read name` — Temporarily reads an outer place. Scoped closures only.
* `mut name` — Temporarily updates an outer place. Scoped closures only.
* `copy name` — Copies a recursively copyable value into the closure.
* `take name` — Moves an owned value into the closure.

```juan
let mut total = 0

let add: fn(i32) = fn[mut total](value)
    total += value
end

let prefix = "Player"
let formatter: escape fn(str): str = escape fn[copy prefix](name)
    prefix + ": " + name
end
```

An escaping closure may capture only with `copy` or `take`. Capturing a move-only value requires `take`. A scoped closure may not be returned or stored in an escaping value.

### 5.5 Function Effects

Effects are part of the function type:

* `alloc` — Allocates managed runtime memory.
* `block` — May block the executing host thread.
* `io` — Reads from or writes to an external system.
* `nondeterminism` — Observes time, randomness or nondeterministic external state.
* `host` — Invokes a host operation not described by a narrower effect.

```juan
fn(i32): str !{alloc} // May allocate
fn(i32): i32 !{}      // Does not allocate
```

A function without an effect set is conservatively treated as `!{alloc, block, io, nondeterminism, host}`. `!{}` promises that the call performs no tracked effect. The compiler checks the body, every transitive call, invoked callback, generic operation and destructor on every exit path. Host imports publish their effects in their signatures.

```juan
fn add(a: i32, b: i32): i32 !{} => a + b
```

An effect set is usable where any superset is accepted; the reverse is forbidden. Adding an effect to a public function or destructor is a breaking interface change.

---

## 6. Variables, Constants & Expressions

Bindings may not shadow another binding in the same scope. A nested scope may shadow an outer binding.

```juan
let my_var = 8
let mut counter = 8
```

The inferred type of both bindings is `i32`. Numeric places may be incremented or decremented:

```juan
let mut my_var = 8
my_var--
```

`++` and `--` are standalone update statements only. They may NOT be placed inside another expression, so `array[i++]`, `use(i++)` and `let old = i++` are syntax errors. The updated place is evaluated exactly once.

Module constants use `const`:

```juan
module main

import std.io

const NAME: str = "Juan"

fn main()
    io.println(NAME)
end
```

You may NOT define a mutable variable in global module scope. Constant initializers must be pure compile-time expressions and may not perform I/O, runtime allocation, environment access, clock reads, randomness or mutable-state access.

`do ... end` creates a value-producing nested scope:

```juan
let hp = do
    let base = 100.0
    let shield = 25.0
    base + shield
end
```

```juan
let result = input_val
    |> step_one(arg1)
    |> step_two()

// Equivalent to: step_two(step_one(input_val, arg1))
```

`|>` is left-associative, evaluates each input exactly once and inserts it as the first argument of the next call. It has lower precedence than every arithmetic, comparison, range and boolean operator.

---

## 7. Types & Declarations

Juan is statically typed and has no undefined or null value.

### 7.1 Nominal Wrappers & Aliases

A nominal type may wrap an existing type:

```juan
type Health = i32

let health = Health(100)
let raw_health = health.inner
```

`Health` and `i32` are different types and never convert implicitly. A scalar wrapper has one synthesized read-only field named `inner`.

`alias` creates a transparent second name for the same type:

```juan
alias EntityIndex = i32
```

### 7.2 Records

Record types use braces:

```juan
type Foo = {
    bar: str
}

let basic_test = Foo { bar = "Test" }
```

If a variable has the same name as the field, you may use shorthand:

```juan
let bar = "Test"
let basic_test = Foo { bar }
```

An anonymous record expression such as `{ bar }` is allowed only when exactly one complete expected record type is known from the surrounding function return, variable type or argument. Otherwise the record type name is required.

A field can be modified only through a mutable place.

```juan
let basic_test = Foo { bar }
basic_test.bar = "Test2" // Error

let mut basic_test = Foo { bar }
basic_test.bar = "Test2" // Works
```

A public record may contain private or package-visible fields. If any required field is inaccessible, outside code cannot construct the record directly and must use a constructor function.

```juan
module game.player

pub type Player = {
    pub name: str,
    health: f32
}

pub fn new(name: str, health: f32): Player => Player { name, health }

pub fn health(player: Player): f32 => player.health
```

### 7.3 Copy & Move Classification

Primitive scalars, `str` handles, named functions and records/unions made entirely from copyable fields are copyable.

`Buffer<T>`, resources and records/unions containing a move-only field are move-only. Assignment, storage, capture or argument passing consumes a move-only value only when ownership is transferred through `take` or another owning position.

Partial moves out of record fields are rejected. A moved binding cannot be used until it is reassigned.

### 7.4 UFCS

UFCS provides dot-call syntax for free functions:

```juan
import game.player

let health = plr.health()

// Exactly equivalent to:
let health = player.health(plr)
```

`value.function(args)` resolves to a matching free function in the value type's home module or one explicitly imported into the current module. The value becomes the first argument. Exactly one candidate must match.

Field access and UFCS calls are separated syntactically:

```juan
player.health       // Always field access
player.health()     // Always UFCS
(player.callback)() // Calls a function stored in a field
```

Parameter modes and effects apply through UFCS as they do through a qualified call.

### 7.5 Operator Overloads

Operators are free functions.

```juan
module vec2

pub type Vec2 = {
    pub x: f32,
    pub y: f32
}

pub fn new(x: f32, y: f32): Vec2 => { x, y }
pub fn +(a: Vec2, b: Vec2): Vec2 !{} => { x = a.x + b.x, y = a.y + b.y }
pub fn *(v: Vec2, s: f32): Vec2 !{} => { x = v.x * s, y = v.y * s }

let mut a = vec2.new(3.0, 4.0)
let b = vec2.new(3.0, 4.0)
let c = a + b

a += b + c * 2.0
a *= 0.5
```

Compound assignment always uses the base operator. `a += b` evaluates the place `a` once, calculates `a + b`, and stores the result back. Compound forms cannot be overloaded separately.

An operator overload may only be declared in the module that defines at least one nominal operand type. Import order never resolves overloads; if one best candidate cannot be found, compilation fails and lists the candidates.

Operator precedence cannot be changed. From strongest to weakest:

1. Calls, indexing, field access, UFCS and postfix `?`
2. Unary `!`, `~` and `-`
3. `*`, `/` and `%`
4. `+` and `-`
5. `<<` and `>>`
6. Bitwise `&`
7. Bitwise `^`
8. Bitwise `|`
9. `<`, `<=`, `>`, `>=`, `==` and `!=`
10. Boolean `&&`
11. Boolean `||`
12. Ranges `..` and `..=`
13. Pipeline `|>`

Comparison operators do not chain. Assignment is a statement and is not part of the expression precedence table. Boolean short-circuiting, assignment, field access and function calls are not overloadable. `a != b` means `!(a == b)`.

Indexing may be overloadable for nominal containers, but it must produce an ephemeral place governed by the scoped-access rules in Section 8. It never returns a storable reference.

### 7.6 Tagged Unions

Tagged unions define variants with or without fields:

```juan
type DamageType = Normal | Poison | Fire
type State = Flying | Fading { remaining: f32 }
```

Variants are qualified when constructing them outside a context that already knows the union type:

```juan
let state = State.Fading { remaining = 2.0 }
```

Variants may be unqualified inside a `match` over their union type.

### 7.7 Generics & Structural Requirements

Types and functions may be generic.

```juan
pub type Result<T, E> = Ok(T) | Err(E)
pub fn identity<T>(item: T): T => item
```

Public generic requirements are written explicitly as free-function signatures. A generic with requirements uses `where ... do` before its block body:

```juan
fn keys_equal<K, S>(strategy: S, a: K, b: K): bool !{}
where
    fn equals(strategy: S, a: K, b: K): bool !{}
do
    equals(strategy, a, b)
end
```

An expression-bodied generic uses `=>` instead of `do` after the requirement list.

Requirements are satisfied in this order:

1. An exact built-in operation.
2. A matching public operation in the home module of a participating nominal type.
3. Otherwise the requirement is unsatisfied.

Imported extension functions may be called directly or through UFCS but do NOT satisfy public structural requirements.

The generic body is checked against its published requirements. Public generic functions must list every requirement. Private generics may infer requirements, which become part of their internal compiled interface.

Juan has no associated types; use an explicit type parameter:

```juan
fn next<I, T>(mut iterator: I): Option<T>
```

Type parameters may have defaults. Defaulted parameters must follow required parameters, and callers may omit only a trailing run of them.

### 7.8 Explicit Conversions

Juan does not implicitly convert between numeric or nominal types. Conversions use named functions.

```juan
let wide = i64.from(small)             // Guaranteed lossless
let value = i32.try_from(large)?       // Checked
let value = i32.truncating(decimal)    // Explicit truncation
let bits = u32.wrapping_from(signed)   // Explicit wrapping
```

There is no general-purpose `as` cast in ordinary Juan code. Unsafe host/layout conversions belong behind checked interoperability functions.

### 7.9 Strings

`str` is immutable, garbage-collected UTF-8 text. Copying a `str` shares its immutable contents instead of copying all of its bytes. String literals use static immutable storage and do not allocate each time they are evaluated.

`str` does not support integer indexing. The standard library provides byte, Unicode-scalar and text iterators.

Repeated string construction uses `StringBuilder`.

---

## 8. Arrays, Buffers, Maps & Scoped Access

### 8.1 Scoped Views

`Slice<T>` and internal `View<T>` values may NOT be returned, stored globally, placed in a GC-managed object, captured by an escaping closure or retained after their owning storage may change. APIs expose them through non-escaping callbacks.

A generic type parameter used in a returned or otherwise escaping position may not be instantiated with a scoped type.

Indexing may create an ephemeral place for the enclosing expression:

```juan
positions[i].x += velocity.x
```

That place may be read, updated or passed to one immediate call, but it cannot be stored or returned.

### 8.2 Maps

`Map<K, V, S>` is a typed dictionary with deterministic insertion-order iteration. `S` is its key strategy type.

The standard canonical form uses its default strategy and is written as `Map<K, V>`. A custom form keeps the strategy explicit:

```juan
type CaseInsensitive = {}

fn equals(strategy: CaseInsensitive, a: str, b: str): bool !{}
fn hash(strategy: CaseInsensitive, value: str): u64 !{}

let users: Map<str, User, CaseInsensitive> = map.new(CaseInsensitive {})
```

A strategy may contain state such as a randomized hash seed, but all values of one strategy type must define the same key-equivalence relation. Equal keys must produce equal hashes for the same strategy value.

Map access uses:

* `get_copy(key)` — Returns `Option<V>` and is available only when `V` is copyable.
* `remove(key)` — Returns `Option<V>` and transfers the stored value out.
* `with(key, callback)` — Gives a scoped read access to the value.
* `with_mut(key, callback)` — Gives scoped exclusive access to the value.

```juan
let score = scores.get_copy("Davey")

let description = scores.with("Davey", fn(score)
    "Score: " + score.to_string()
end)

scores.with_mut("Davey", fn(mut score)
    score += 100
end)
```

The callbacks are non-escaping. Their result type must be independently escaping and may not contain access derived from the map entry. While `with_mut` holds exclusive access, its callback may not capture or re-enter the same map incompatibly.

Updating an existing key keeps its iteration position. Removing and reinserting it places it at the end.

### 8.3 Buffers & Arrays
* `Array<T, N>` — Fixed-size inline storage. It may live in a local, another type or heap allocation; it is not guaranteed to be on the stack.
* `Buffer<T>` — Move-only, dynamically growing owned heap storage.
* `Slice<T>` — Scoped bounds-checked access to contiguous elements. It does not own them.

A buffer exposes scoped slices using callbacks:

```juan
positions.with_slice(fn(slice: Slice<Vec3>)
    process(slice)
end)

positions.with_slice_mut(fn(mut slice: Slice<Vec3>)
    update(slice)
end)
```

Buffer bounds are checked unless proven valid. Ownership follows the parameter and return rules in Section 5.1.

---

## 9. Error Handling

Optional values use `Option<T>` and recoverable failures use `Result<T, E>`.

### 9.1 Option<T>

An `Option<T>` is `Some(T)` or `None`.

### 9.2 Result<T, E>

Postfix `?` extracts `Ok(value)`. If it receives `Err(error)`, it immediately returns that exact error type from the current function.

```juan
fn load_player_config(path: str): Result<Config, LoadError>
    let content = read_file(path)
        .map_error(LoadError.Io)?

    let config = parse_json(content)
        .map_error(LoadError.Parse)?

    Result.Ok(config)
end
```

Juan does not perform hidden error conversions. `map_error` accepts a function value, and tagged-union constructors such as `LoadError.Io` work directly.

Bounds failures, failed runtime safety checks and violated runtime invariants trap. Integer overflow and division by zero trap in every build profile; explicit `checked_*`, `wrapping_*` and `saturating_*` operations provide alternate behavior.

---

## 10. Resources & Deterministic Destruction

Files, GPU resources, sockets, subscriptions and other external resources use deterministic destruction.

A move-only resource type may designate exactly one destructor using the built-in `@drop` attribute:

```juan
@drop
fn close(take file: File) !{}
    host.close_file(file.handle)
end
```

`take` transfers cleanup responsibility. An owning value must eventually be returned, moved into another owner, passed to another `take` parameter, explicitly dropped or automatically destroyed at scope exit.

Destruction occurs exactly once in reverse ownership order on normal return, early return and `?` propagation. Partial initialization and failed construction destroy only the fields that became initialized.

Destructor effects are published in the owning type's compiled interface. An allocation-free function may own a local only when every destructor that can run on its exit paths is also allocation-free, or when ownership is transferred before the exit.

An `@drop` function is not a GC finalizer.

Resource-owning values may not be stored inside ordinary GC-managed objects or escaping closures.

---

## 11. SIMD Operations

Juan provides fixed-width SIMD vector and mask types as compiler-known primitives. A target without matching hardware instructions must preserve the same behavior using narrower vectors or scalar operations.

### 11.1 Built-in SIMD Types
* `f32x4`, `f32x8`
* `i32x4`, `i32x8`
* `u8x16`, `u8x32`
* `mask32x4`, `mask32x8`, `mask8x16`, `mask8x32`

### 11.2 Construction & Operators

Vector types support element-wise operations with standard arithmetic operators (`+`, `-`, `*`, `/`). Their lane counts are part of their types and never change based on the target machine.

```juan
import std.simd

fn process_quad(a: f32x4, b: f32x4): f32x4 !{}
    a * b + f32x4.splat(1.0)
end
```

### 11.3 Lane Masking & Selection

Conditional vector logic uses lane masks and selection.

```juan
import std.simd

fn clamp_lower(v: f32x4, min_val: f32): f32x4 !{}
    let limit = f32x4.splat(min_val)
    let mask: mask32x4 = v.greater_than(limit)
    simd.select(mask, v, limit)
end
```

---

## 12. Attributes & Contracts

Attributes are compiler metadata or obligations attached to an existing declaration. Attributes are NOT automatically macros.

```juan
@attribute_name
@attribute_name(argument)
@attribute_name(key = value)
```

Unknown attributes are compile errors.

### 12.1 Built-in Attributes

* `@requires(condition)` — Every caller must establish the precondition.
* `@ensures(condition)` — The condition must hold on every applicable return.
* `@invariant(condition)` — Defines a type or loop invariant.
* `@decreases(expression)` — Supplies a termination measure for recursive or verified code.
* `@trusted(reason)` — Creates an explicitly reviewed verification boundary.
* `@layout(c)` — Gives a narrowly allowed record a stable C-compatible ABI layout.
* `@drop` — Marks the one deterministic destructor for a resource type.
* `@deprecated(message)` — Produces use-site warnings.
* `@const` — Marks a pure function as valid in constant evaluation.
* `@syntax(kind)` — Declares a syntax macro of the specified kind.

Built-in attribute names are reserved and do not require imports. Each built-in defines its legal targets and argument grammar. A package may not shadow one.

Function effects use the type syntax defined in Section 5.5, not attributes.

### 12.2 Contract Semantics

Contract expressions are pure. They may not mutate, allocate runtime objects, perform I/O, read clocks or randomness, or call functions whose effects are incompatible with specification evaluation.

`old(expression)` denotes the logical value of an expression in the function's pre-state.

Contracts are checked at runtime unless the compiler proves them. A proven check may be omitted without changing the contract's meaning. Failed checks identify the contract, source location and relevant values.

Runtime quantifiers must have finite bounded domains. Floating-point contracts use Juan's ordinary IEEE behavior, including explicit handling of NaN when relevant.

Declared parameter modes provide the initial frame condition: a function may only modify state reachable through `mut` parameters or explicitly authorized host effects.

---

## 13. Macros

Juan macros are hygienic declarative syntax transformations. They match structured syntax and emit Juan syntax trees. Constant evaluation computes values without transforming syntax. Procedural macros and arbitrary compile-time programs are not part of Juan.

### 13.1 Declaration Macros

A declaration macro uses `fn` with the built-in `@syntax(declaration)` attribute:

```juan
@syntax(declaration)
pub fn event($name: identifier, $fields: field*)
    => type $name = {
        $fields
    }
end
```

Inside an `@syntax` declaration only:

* `$name` declares or splices a metavariable.
* `identifier`, `type`, `expression`, `pattern`, `field`, `declaration` and `token_tree` are compiler-known syntax categories.
* `*`, `+` and `?` mean repeated, one-or-more and optional captures.
* `=>` separates the matcher from the syntax-tree template.

This is not an executable function and has no runtime function type.

After explicitly importing the macro:

```juan
import { event } from awesome_events.syntax

event Damage
    source: Entity
    target: Entity
    amount: f32
end
```

It expands into an ordinary `Damage` record declaration. Macros extend declaration boundaries; core expression, statement, pattern and type grammar remain closed.

### 13.2 Attribute & Derive Macros

An imported attribute macro receives the parsed declaration it decorates and may validate it or emit additional declarations.

```juan
import { serialize, replicate } from awesome_net.syntax

@serialize
@replicate(rate = 20)
type PlayerSnapshot = {
    position: Vec3,
    health: u16
}
```

`@derive` is a built-in dispatcher for imported declarative derives:

```juan
import { equality, serialization } from std.derive

@derive(equality, serialization)
type Player = {
    name: str,
    health: Health
}
```

Derives emit free functions and declarations.

### 13.3 Parsing Without Executing Macros

Tooling must parse a macro invocation without running user code. At a declaration boundary, an explicitly imported syntax name followed by tokens becomes a generic `SyntaxInvocation` node.

The delimiter scanner gathers tokens through the matching `end`, respecting nested core blocks and balanced `()`, `[]` and `{}`. A macro's compiled interface exposes a declarative parsing schema that the editor may inspect without executing expansion code.

If a tool lacks the macro interface, it preserves the invocation as an opaque token tree. Macro-defined `end` envelopes may not contain another macro-defined `end` envelope; nested core constructs remain valid.

### 13.4 Hygiene, Resolution & Expansion

* Macro names are lexically scoped and explicitly imported.
* Literal names created by a template receive definition-site identity.
* Captured names retain call-site identity and source spans.
* Templates cannot request implicit call-site capture.
* Macros may emit declarations and attributes.
* Macros may not emit imports, module declarations or new macro definitions.
* Expansion happens after parsing and macro-import discovery, but before ordinary name resolution and type checking.
* Expansion order is source order with bounded rounds and a finite recursion limit.
* Diagnostics show the invocation, generated failing node and expansion trace.

Attribute macros expand top to bottom. Built-in non-expanding attributes remain attached to the declaration and cannot have their meaning removed by a macro. Generated attributes expand in a later bounded round.

### 13.5 Capability Boundary

Macro expansion has no filesystem, network, environment-variable, clock, randomness, process or arbitrary host access. It may inspect captured syntax and immutable compiler facts such as the target profile.

Equal macro input, imported interfaces and compiler configuration must produce equal syntax trees. Expansion has instruction, memory, output-size and recursion budgets.

---

## 14. Garbage Collection, Concurrency & Hot Reload

### 14.1 Garbage Collection

Juan uses one isolated managed heap per runtime. The collector is precise, incremental, host-steppable and non-moving. Hosts control work budgets and may request a full collection for diagnostics or retired-code reclamation.

Managed values include strings, escaping closure environments and graph-shaped scripting objects.

Rust and other hosts keep managed values alive through opaque rooting handles. Raw managed-object pointers never become stable public API.

### 14.2 Concurrency

Juan values do not cross threads or runtimes by default. A runtime is scheduled by one host thread at a time. Communication uses copied immutable data, explicitly transferred owned values supported by the host, serialized messages or thread-safe opaque host handles.

Shared mutable managed graphs are not supported.

### 14.3 Hot-Reload Generations

Hosts may omit hot reload.

Each loaded code module belongs to a generation:

* Existing frames finish using the generation they entered.
* Existing closures and function values stay bound to the generation that created them.
* New exported calls use the newest compatible generation.
* Type-layout or incompatible signature changes require explicit state migration or a clean runtime restart.

A retired generation remains pinned by active frames, Rust roots, registered callbacks, queued tasks, closures or function values in the managed heap. It may be unloaded only after a complete GC trace proves no managed reference reaches it, all external pins are gone and the runtime reaches an unload-safe point.

The runtime reports which roots prevent unloading and may warn or restart a development runtime when retired-generation memory exceeds a configured budget.

### 14.4 Initialization & Registration

General initialization and reloadable callback registration are separate operations.

* Initialization runs once when creating a runtime and may create persistent state.
* Registration declares generation-owned callbacks, systems, commands or UI extensions.
* Migration converts explicit persistent state between incompatible schemas.

Registration receives a capability-limited `Registrations` value and cannot spawn gameplay entities or perform unrelated startup work.

```juan
fn register(mut registrations: Registrations)
    registrations.on_update(UpdateRegistration {
        phase = UpdatePhase.Gameplay,
        priority = 0,
        callback = update
    })
end
```

Reload builds a new registration set transactionally. If registration fails, the old set stays active. On success, the host atomically installs the new set and revokes the old generation's registrations.

Registration execution order is deterministic:

1. Explicit phase.
2. Lower numeric priority first.
3. Fully qualified registration identity as the tie-breaker.

---

## 15. Rust Interoperability & Execution Modes

Juan is a scripting language for Rust hosts. Its compiler, runtime and embedding library are written in Rust, and generated bindings expose safe Rust APIs.

Separately compiled or hot-reloadable binaries use a stable internal calling convention and layout contract instead of Rust's native ABI and default layouts. No C source API is exposed.

Generated Rust adapters provide typed handles, `Result`, scoped rooting guards, panic containment, access validation and bulk data operations. The low-level boundary uses opaque handles and validated regions; it never creates overlapping Rust `&mut` references.

Host functions appear to Juan as typed free functions.

Juan supports one semantic language across multiple execution modes:

1. A portable bytecode VM for editor use and fast reload.
2. Ahead-of-time native object generation for production performance.
3. An optional JIT for supported desktop and editor platforms.

Every backend must pass the same conformance suite for results, traps, overflow, evaluation order, destruction, GC reachability, contracts, host calls and reload behavior.

---

## 16. Basic Parsing Rules

Identifiers use ASCII letters, numbers and `_`. They must start with a letter or `_`. Strings, character literals, comments and documentation support Unicode.

Integer literals may be decimal, binary (`0b`), octal (`0o`) or hexadecimal (`0x`). Their optional suffix is one of `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128` or `usize`; an unconstrained unsuffixed integer defaults to `i32`. Floating-point literals use a decimal fraction, an exponent or an `f32`/`f64` suffix; an unconstrained unsuffixed float defaults to `f64`. Underscores may separate digits but may not begin or end a digit sequence. A leading `-` is the unary operator, not part of a literal.

Strings use double quotes and support escapes such as `\n`, `\r`, `\t`, `\\`, `\"` and `\u{1F600}`. A single-quoted literal contains exactly one Unicode scalar and has type `char`. A `b'X'` literal contains one ASCII byte and has type `u8`; a `b"..."` literal contains only bytes and escapes in the range `0..=255` and has type `Array<u8, N>`.

Commas separate arguments, fields, generic arguments and items inside delimiters. A trailing comma is allowed in multiline `()`, `[]` and `{}` lists. Match arms and ordinary newline-separated block items do not use commas.

Keywords such as `as`, `from`, `where` and `do` are contextual where their grammar position is unambiguous. Built-in attribute and macro syntax names are not ordinary hard keywords.

The parser preserves comments, whitespace, attributes, macro invocations and invalid fragments in a lossless syntax tree. It creates error nodes and continues parsing when possible.

The compiler pipeline is:

```text
Lex
  -> Parse lossless CST
  -> Discover module and macro imports
  -> Expand macros
  -> Resolve ordinary names
  -> Check types, access, ownership, escape and effects
  -> Lower to Juan IR
  -> Execute or generate code
```

