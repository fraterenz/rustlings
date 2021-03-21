Rust is a strongly-type language: 
- the language takes advantages of the behaviours embedded into the variables' types: choose the type of your variable based on the tasks these variables need to perform. Similar to [C++ operator overloading](https://youtu.be/DnT-LUQgc7s?t=774). For instance, the null pointer example encapsulated into a `Option` annum, see [here](#no-hidden-states)
- types must have a fixed sized at compile type, see [recursive types with `Box`](https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes), all these types implement the `Sized` trait

Rust provides memory safety when resources matter (speed and CPU usage in the system programming field) that is low-level language, memory safe with zero cost abstraction.

# Learning rust
Read the book and at the same time do rustlings, have a look at [half-hour to rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust), do exercism rust track, the [crust of rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) and have a look at the following crates:

1. [ndarray](https://github.com/rust-ndarray/ndarray)
2. [autograd](https://github.com/raskr/rust-autograd)
3. dbug

## Talks

- [Jon Gjengset](https://www.youtube.com/watch?v=DnT-LUQgc7s)
- [Niko Matsakis](https://www.youtube.com/watch?v=jQOZX0xkrWA)

# Algebraic types and pattern matching
In a `struct` or `enum`, the data in the `struct` fields and the behavior in `impl` blocks are separated, which is different from other programming languages.

Remember that `Option` is a `enum` (similar to a type) and `Some` and `None` are the values that a variable `Option` can take, so the data is `Some` or `None` but the type is `Option`.
```
if let Some(f) = my_vec.find(|t| t >= 42) {
        // found or None; brackets because if?
}
```
means if `find` finds the `t_i` it returns `Some(t_i)` and then assign `t_i` to `f`.

Another example, from [here](https://fasterthanli.me/articles/a-half-hour-to-learn-rust):

```
if let Number { odd: true, value } = n {
	println!("Odd number: {}", value);
}
```

In a match block, the compiler ensures that you have exhausted and thought about all the possibilities that could match. Else, error (you can use `_` to match all the things you haven't listed).

# Pointer types
There are mainly three pointer types in rust:

1. (im)mutable reference `&` and `&mut`
2. (im)mutable raw pointers `*cont T` and `*mut T`
3. Smart pointers (like in C++)

So when you create a variable `let a = &mut b` you are creating a mutable reference on `b`, that is `a` can modify `b`. On the other hand, if you write `let mut a = &b` means that `a` cannot modify `b`, but `a` can be modified later in the program (of course without affecting `b`):

```
fn main() {
    let mut a: u32 = 3;
    let mut b = &a;
    println!("Hello, world! {}", b);
    println!("Hello, world! {}", a);
    b=&2;
    println!("Hello, world! {}", a);
    println!("Hello, world! {}", b);
}
```
this will print `3 3 3 2`.

# Vec, array and slices
Each of these are different types:
- **vec:** a `struct` with a pointer (pointing to the data), capacity (amount of space allocated for any future elements that will be added onto the vector) and length (the number of actual elements in the vector), see [here for more](https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation)
- **array:** fixed size array, size known at compile time
- **mutable slices:** its size is known only at run-time (dynamically sized type), view into block of memory, `&[T]`
- **immutable slices:** its size is known only at run-time (dynamically sized type), view into block of memory, `&mut [T]`

## To move or to borrow
- **vec:** with mutable or immutable indexing both with `v[2]` but depends on `v` [whether it is mutable or not](https://doc.rust-lang.org/std/ops/trait.IndexMut.html), move with several methods such as `pop`, `take` ??
- **array:** borrow by coercing into slice with `&` and `&mut`, move with [`slice patterns`](https://doc.rust-lang.org/reference/patterns.html#slice-patterns)
- **slices (both immutable and mutable):** you can borrow ?, but you cannot move since slice does not own its data

## Copying
Shallow copy for all except arrays which means move for `vec` and `Copy` for slices. To avoid moving `vec`, use `clone` to perform a deep copy. For array, deep copy since arrays are on the stack and not on the heap.

## From one type to the other
- **T2slice:** for both `vec` and array you can transform it into a slice with `&`, you can also use `leak` for `vec` when you want to consume it
- **T2vec:** for both slice and array you can transform it into a `vec` with `to_vec`, for slice you also have `extend_from_slice`
- **slice2array:** `copy_from_slice` or `try_into` from Trait `TryInto` see [here](https://stackoverflow.com/questions/25428920/how-to-get-a-slice-as-an-array-in-rust)
- **vec2array:** I think you cannot do that, since ??
TODO: have a look at [this](https://doc.rust-lang.org/stable/rust-by-example/conversion/try_from_try_into.html) and rustlings from into.


# Strings
When you declare a binding `let a = "Yo";` you are creating a binding to a string slice which is not stored on the heap, which is not owned type. So use `String` only when you need to modify the string. Some methods to convert string slices (or in general any variable whose type implements the `Display` trait):

```
let s = "hello".to_string();  // ToString
let s = String::from("hello"); // From
let s: String = "hello".into(); // Into
let s = "hello".to_owned();  // ToOwned
```
The slice data structure `&str` stores the starting position and the length of the slice.

## The different types of string in rust
They could roughly represented [as follows](https://youtu.be/rAl-9HwD858?t=4012): 

- `str -> [char]`: but not quite the same as, no size, a sequence of char
- `&str -> &[char]`: pointer that stores the start of the string/slice and the length of the slice
- `String -> Vec<char>`: 1. heap allocated 2. dynamically expandable

From a `String` to `&str` is cheap and easy, performed with `AsRef` in `String`. It's easy because `String` knows where it starts and the length. The opposite is not cheap, because you need to copy all the chars into heap and uses `memcpy` (deep copy). Note that you can have a `&` to a `&str` as in `crust/strsplit/src/lib.rs`.

# Functions and Methods
There are [3 possible ways to create bindings](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1) between function parameters and arguments:

1. by-value (meaning in Rust that it either takes ownership of the bound value, or makes a copy of it, depending on whether the type of that value implements the `Copy` trait), consumes the original variable (invalidate the original binding) if the data is on the heap (because it is a shallow copy of smart unique pointers, allowing one owner at the tine)
2. by-reference (can have many immutable references, like in C++ with unique ptr)
3. by-mutable-reference (only one mut reference, cannot have a mutable and an immutable reference at the same time)

`Copy` is a trait indicating a type is “trivially copyable,” meaning it can be copied with only a call to `memcpy`, so all the data contained in the structure is contiguous; there are no pointers to chase. `Copy` tells us that copying a piece of data is fast.

Methods are different from functions, since method are defined on an object (an instance of a `Struct` for instance). However, methods can take ownership (as functions) of `self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably, just as they can any other parameter.

Methods used the keyword `self` as argument since Rust knows the type of `self` is the same of the object due to this method being inside the `impl` context. See also the [automatic dereferencing process](#modern-language).

## Closures
Closures have zero cost overhead and are used when a function requires access to the context. Functions that require no context have the `fn` type, which is called a function pointer and implements all the traits used to define a closure `Fn`, `FnMut` and `FnOnce`.

`let k = my_vec.iter().filter(|&&n| **n > 0).count();` since `iter` returns a reference over an item, that is `&T`(let's say `i32`, see [here](https://doc.rust-lang.org/std/iter/index.html#the-three-forms-of-iteration)) and `filter` takes a closure that in turn takes a mutable reference on the `Item` that is on `Self`, as explained by the trait bound in the doc `where, P: FnMut(&Self::Item) -> bool`, see [here](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter). The trait `Fn` is used by closures to capture the values from their environment, see [here](https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html).

**Calling a closure is executing a method on a struct**: A closure is syntactic sugar to create a struct capturing the whole environment variables by reference, unless so variables are explicitly consumed in the closure as in the example [here](https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html), `move |x| m*x + c` as in threads.

**Storing closures** even if two closures implement `Fn` traits, since they are structures, they need to be stored in a heterogeneous collection `Box<Fn(x: f64)->f64 + 'a>`, see [here](https://stevedonovan.github.io/rust-gentle-intro/pain-points.html#references-and-lifetimes) and also [here](#OOP).

# Memory management
By default, variable bindings have 'move semantics', however, if a type implements `Copy`, it instead has ['copy semantics'](https://doc.rust-lang.org/std/marker/trait.Copy.html).
Managing memory at compile time (pointers checks) is the key point of rust. When that is not possible (e.g. user input non defined at compile time but at runtime), rust stored data on the heap (e.g. `Vec`, `Box` or `String`). I think that most of the things that are stored on the heap binds to the variable using smart pointers, more specifically `unique_ptr` in C++, see [here](https://youtu.be/CaZP-1ETL-o?t=377). 

The rules are there to [avoid the following errors](https://youtu.be/DnT-LUQgc7s?t=1211):

1. only one owner: the owner is responsible to free the memory allocated to an object. SO NO DOUBLE FREES! (moving)
2. no pointers outlive the owner: if the owner was dropped/moved, cannot be reference to it. SO NO USE AFTER FREE / dangling pointers (borrow checker)
3. safely give immutable reference to some code you don't know, and they cannot modify it
4. only 1 writer or multiple readers. NO DATA RACES

With data on the heap (that is that do not have the `Copy` trait) you have three options, two of them involve (borrowing) creating a new pointer:

1. **moving:** the new created pointer takes the ownership. Rust copies the pointer (shallow copy) + invalidates the 1st pointer BUT NOT THE DATA POINTED! If you try to use the invalidated pointer, you'll get the value borrowed after move, see [Figure4-2](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html) and see rustling/scratch/moving1.rs

2. **borrowing:** the new created pointer does not takes the ownership. It cannot modify the object (aka cannot borrow as mutable), more than one pointers of this type can exist.

3. **borrowing mutably:** there can be only one pointer accessing the data in the same scope (but there can be two pointers sequentially), see [video Gary](https://youtu.be/79phqVpE7cU?t=50). Only one mutable pointer at the same time. You can achieve this by either creating a mutable ref `&mut` or in the prototype of a function, as in `rustling/move_semantics3.rs`. See [Figure4-5](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html). Note that you can have a immutable object borrowed as `mut`, as long as the variable immutable is not used later on in the code, see [here](https://github.com/rust-lang/rustlings/issues/631).

Scope means `{}` but also the last time a binding is mentioned (again as in [Gary's video](https://youtu.be/79phqVpE7cU?t=506)).

So, with a string s, depending on what you want to do, you can: 
1. reading (&s): just need to read the data (borrowing)
2. mutating (&mut s): read to write the data (borrowing mutable, only 1 owner at the time)
3. consuming (s): the variable wont be needed later on (moving)

**Ownership rules (moving):** value is a name bound to an object,
1. Each value in Rust has a variable that is called its owner.
2. There can only be one owner at a time for each value.
3. When the owner goes out of scope, the value will be dropped.

**Borrowing rules (references):**
1. Can't have a mutable and an immutable ref (borrowing) at the same time
2. Max 1 mut reference (counting also the owner! that is if the owner has `mut` the ref cannot have `mut` too) or several immutable refs

When data is on the heap, the value `s1` bound to the data is only a pointer! Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance. To create deep copies `clone` trait.

## Examples
### Messing with ownership (moves)
Immutable variable moved to `mut` variable: this particular example is not allowed because `val` has been moved to `var` and the data associated to it has been dropped?:

```
let val = Vec::<u32>::new();
let mut var = val; // create a mutable object
var.push(5);
println!("{:?}!", var);
// if we remove this line here, all fine since
// val has completed this immutable job and has
// been dropped
println!("{:?}!", val);  //  error!
```
note that if you remove the last line, the code will work even though `val` is immutable and `var` is `mut`. See [issue](https://github.com/rust-lang/rustlings/issues/631#issuecomment-770170180). Indeed, we think about `let val = Vec::new()` has a pointer pointing to a immutable allocated memory; the next line just create another pointer, `mut` this time, with the same memory address of `val`. All fine only if `val` is not used after the initiation of `var` pointer.

### Messing with references
Note that these observations are valid also when using data on the stack:

1. Mut references and `mut` variables: this is allowed since `var` is a reference, dropped since not used anymore (but the data associated to it is not dropped since no owernship), and `val` is valid again:
```
let mut val = Vec::<u32>::new();
let var = &mut val; 
var.push(5);
println!("{:?}!", var);  // drop mut ref var, revalidates val?
println!("{:?}!", val);
```
2. Immutable references and `mut` variables: this is fine since the references will never access bad data (see [gist](https://gist.github.com/rust-play/b77eee2dbcd6ec4072fd3d9eb5a8a875), see [Jamila](https://youtu.be/nesyOcj8swk?t=346) because in C++ you can have data races, see the [video also](https://youtu.be/lQ7XF-6HYGc?t=1582)):
```
let mut val = Vec::<u32>::new();
val.push(1);
let val1 = &val;
let val2 = &val;
println!("{} {} {}", val, val1, val2);
val.push(12);
// this would result in an error because val1 ref
// is an immutable ref so at first it pointed to 
// a vector [1] but now it points to a vec [1, 12]
// which is not possible, since it is immutable.
// If you remove this line here, no error because 
// the job of val1 was completed with val = [1],
// eventhough the objected has changed later on, val1
// is not used anymore and it's dropped.
//println!("{:?}, val1);
```
3. Mut references and immutable variables: error.

4. Immutable references and immutable variables: fine.

# Modules
Great flexibility in rust module system since rust's module paths are not tied directly to the filesystem: hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

# Bring paths into scope with `use`
The convention is to always bring to scope the parent module to avoid confusion (`use std::env` and then `env::args` instead of directly `std::env::args`) except for structs, enums, and other items where you should use the second convention, that is `std::collections::HashMap`. 

# Traits
Are used to capture common behaviors among different types.

# To panic! or not to panic!
To panic! is more appropriate for a programming problem than a usage problem. For the latter is better to return a `Result` from the function, and in the caller block (like the `main` for instance), you can `unwrap_or_else` the `Result`. Doing so, the value will be `unwrap`ped if the function returned an instance of an `Ok`, else it will print an string that you want, and then you can exit the program from [here](https://doc.rust-lang.org/book/ch13-03-improving-error-handling-and-modularity.html#calling-confignew-and-handling-errors):

```
let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
});
```

With errors you can either panic or not. In order to panic you can:

1. `.unwrap()` a `Result`
2. `.expect("my message")` a `Result` you can add a message
3. `match` with `Err(e) => panic!(e)`
4. use the `?` operator after a `Result`

To not panic you can:

1. `if let` see rustlings `exercices/option2.rs` and [here](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#handling-errors-returned-from-run-in-main); `if let` used also when you call a function that does not return anything except an error in case of failure
2. `match` with `Err(e) => println!("Warning")`

# Lifetimes and references
Lifetimes are there to avoid dangling references, only used when references! There are 3 rules that the borrow checker applies in order to assign a lifetime to each reference:

1. Each reference has one lifetime
2. If a function has only one input reference, then the output reference will have the same lifetime of the input reference
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters

See also the [crust of rust folder](https://github.com/fraterenz/crust/tree/master/strsplit).

# OOP
State pattern OOP encapsulates both the state (attributes of the class) and the behavior (with methods of that class). This is achieved using classes.

There is no concept of `class` in rust, because the idea of grouping data as well as methods is somewhat controversial, see [here](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html#animals) and [here](https://www.infoworld.com/article/2073649/why-extends-is-evil.html). According to [rust book](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html#inheritance-as-a-type-system-and-as-code-sharing), inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary. Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance.In this way the code is more flexible and less prone to errors. **Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism.**

In rust, classes are split between data and traits. Therefore, you cannot have inheritance of classes but of traits (implementation inheritance vs interface inheritance, see [here](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html#animals)), the data will not be inherited. To have a concept similar to classes you need to have a collection (such as an Enum or Struct) and some traits, see [here](https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html#animals). Remember that traits are used to share common behaviors among different types.

The idea in rust is to use the type check system to enforce the state and to encode the behaviors. In the [rust book example](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types) the user works with the `Post` (using `Post::new()`, `post.request_review()` and `post.approve()` but these methods returns different objects that behave differently since they represent different states of the `Post`. **Give the user the parent class, after every method calls, the object returned is specialized (consumes the previous object taking the ownership of `self`) such that it represents both the behavior and the state, but the behavior is in the implemented trait (which are used to capture common behaviors among different types), whereas the state is in the type**.

You can have dynamic dispatch allowing polymorphism on traits only, the virtual methods (trait objects) will be resolved at runtime, causing a little runtime overhead. As in C++, you need both a pointer (`Box`) and a virtual method redefined in another structure or enum (trait): `Box<dyn Draw>`.

# Macros
Remember to export macros with `#[macro_export]`, it is similar to `pub` for functions. There are 4 types of macros:

1. declarative: allow you to write something similar to a Rust match expression, very useful for [repetitions](https://youtu.be/q6paRBbLgNw?t=2799) but also for non-fixed number of parameters in function
2. procedural custom derive
3. procedural attribute-like 
4. procedural function-like

Procedural macros accept some code as an input, operate on that code, and produce some code as an output.

Differences with functions:

- you must define macros or bring them into scope before you call them in a file `#[macro_export]`
- can take a variable number of parameters
- macros are expanded before the compiler interprets the meaning of the code
- more complex than function: more difficult to read, understand, and maintain

## More on declarative macros: crust
The goal is to substitute a grammar-valid rust sentence into a rust valid code.

- macros input: must be syntactically grammar valid
- macros output: rust valid code, you can think of it such that the return type must be an expression
- the double curly brackets: `{ { ` the first one is required by the macro syntax and means "this stuff here", whereas the second one is to tell the compiler that we want to expand the macro into a block

**macro input:** macros input must be parsable rust code (syntactically correct, but not valid rust) but the output must be [valid rust](https://youtu.be/q6paRBbLgNw?t=587). More precisely, the input must be [single non-leaf token tree](https://danielkeep.github.io/tlborm/book/mbe-syn-macros-in-the-ast.html), which are parsed during the parser step while constructing the abstract syntax tree (AST). Note that identifiers in macro world are completely distinct from variables [outside the macro world](https://youtu.be/q6paRBbLgNw?t=873), but if you use the syntax `$x:ident` in the macro world, and you call it using `avec!(x)`, then the two identifiers match: your are creating the link between the two `x`, see [hygiene](https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html).

#### Macro expansion
The macro call gets completely replaced with the rust code found in the `{ }` based on the match arm of the macro. This steps is done after the construction of the AST, but [before the compiler starts linking and doing statical analysis](https://danielkeep.github.io/tlborm/book/mbe-syn-expansion.html), see also overleaf `CS`. Since the macro expansions happens **after** the construction of the AST, the syntax used to invoke a macro must be a proper part of the language's syntax, there are [four valid syntax expansions](https://danielkeep.github.io/tlborm/book/mbe-syn-macros-in-the-ast.html). This involves traversing the AST, locating macro invocations and replacing them with their expansion. 

To see what your macros expands to, use the [crate](https://github.com/dtolnay/cargo-expand) which takes as input the source of the crate you are currently in, and expands all the macros with their definitions. To use it, you need nightly version of rust that you can set for the current project as indicated [here](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly).

After the macro expansion, the compiler will parse the result into an AST node that represents an item, based on the position of the macro call. In other words, where you can invoke a macro determines what its result will be interpreted as. 

As a consequence:
- **limited number of invocation positions:** since macros are parsed by the parser as part of the AST (and expanded immediately after), they can be used only in positions where there are [supported](https://danielkeep.github.io/tlborm/book/mbe-syn-macros-in-the-ast.html): 1. Patterns 2. Statements 3. Expressions 4. Items 5. `impl` Items.
- **expansion must match the expected AST node:** macros can only expand to the kind of AST node the parser expects at that position.
- **macros must be complete and syntactically valid constructs**
- **recursion limit in macro expansion:** until all the macros are recursively expanded, the AST construction is not ended, unless there are more than 32 recursive expansions. This limit can be raised using the `#![recursion_limit="…"]`

#### `macro_rules!`
It is a syntax extension, meaning it is technically not part of the Rust syntax. Takes this form:
```
macro_rules! $name {
    $rule0 ;
    $rule1 ;
    // …
    $ruleN ;
}
```
and there must be at least one rule and each rule looks like this `($pattern) => {$expansion}`. Interestingly, `macro_rules!` does not expand to anything, instead it manipulates compiler-internal structures to register the macro.

**matching:** if the input matches the pattern, the invocation is replaced by the expansion; otherwise, the next rule is tried. If all rules fail to match, macro expansion fails with an error. Patterns can also contain captures. These allow input to be matched based on some general grammar category, with the result captured to a variable which can then be substituted into the output

**captures:** are written as a dollar followed by an identifier, a colon, and the kind of capture, that must be [one of the following](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html#captures). It is not very clear what each type really represents, also because there is [hierarchy among types](https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html), but `expr` is anything that you can add a `;` afterwards.

**repetitions:** create for loops, see [here](https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html#repetitions).

# Project organization
See [here](https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/gpdti5j?utm_source=share&utm_medium=web2x&context=3) and [here](https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/gpel5rg?utm_source=share&utm_medium=web2x&context=3). 

# Modern language
The compiler knowns about:

- tests: `#[test]`, and can automatically access to private fields! The compiler knows also about the doc, also doc tests `///` and `/// assert_eq!(one_more(42), 43)``` and it will compile and run the doc tests too. **Very interesting:** `compile_fail` for doc tests that are supposed to fail, see [here](https://youtu.be/q6paRBbLgNw?t=3383).

- buit-in dependencies and dependecy graph: each time you compile your program, the dependencies in the `.toml` file will be fetched with `cargo build`. Up to date version of the dependency without breaking (since if you specify `1.3.2` it will fecth everything greather than that, but smaller than `2.0.0`, since semantically that version should cause breaking changes in the code).

But, no pre-built libraries: need to build everything from source. This is mainly due to generics `<T>`: if the library I want to use has generics, I need to compile the version of the method for the type I'm using, which is defined in my library.

- automatic referencing and dereferencing: when you have pointers such as `Vec` or `String`, calling `method` on the pointer, `my_vec.method`, will automatically dereference the pointer `my_vec` and call `method` on the pointed object (not like C++ where you have to derefence explicitly doing `(&p1).distance(&p2);`, see [here](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?highlight=impl#wheres-the---operator).

- **clippy:** `cargo clippy` or in CLI

- **rustfmt:** `cargo fmt`

# No hidden states

No null pointer, you need to check the `Option` and the `Result` enums (the latter checked with `?` the try operator: `"42".parse()?` returns error or unwrap). The data type `Option` tells you whether the object could be `None`.

# Data modelling

- *primitive obsession:* when a complex pattern would suite better the problem compared to primitive types, e.g. when two pieces of data are related to each other and instead of being grouped into a Struct, they are stored as tuple or array, see [here](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#grouping-configuration-values).
- if you need to create an object that can be shared by either two different kind of resources (such as shared with one or more recipients **or** received from a source), see [here](https://www.reddit.com/r/rust/comments/l594zl/everywhere_i_go_i_miss_rusts_enums/gkteafc?utm_source=share&utm_medium=web2x&context=3)

# Reading the doc 
The `std::iter::Iterator` trait:

1. Is associated to the type `Item`, associated types are just placeholders used by the method signature implemented by the trait, see [here](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
2. `next` is the only required method to use this trait
3. Provides other methods
4. Has several functions

The function `chain` for instance, takes as input an iterator `self` and generic type that implements the trait `IntoIterator`, and all the types implementing that trait can be found clicking on the trait, linked to the [doc of the trait](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#implementors). It returns a new `Iterator`, a strict `Chain`. Note that it takes the ownership of `self` meaning that it create a new iterator and invalidate the previous one (by consuming it?), that is why in many examples you see `myarray.iter().sum()`, that is they do not create a variable `let myiter = myarray.iter()`, since `myiter` will not be available after the `sum()` call.

`pub fn sum<S>(self) -> S where  S: Sum<Self::Item>,` means that the method returns an object of generic type `S` that implements the trait `Sum`, it does not mean that the method returns the `Sum` trait.

**The `Vec` entry:** `Methods from Deref<Target = [T]>` tells you that some methods come from array slices, since `Vec` can be `Deref` into slice arrays, see the documentation at `trait.Deref` and look for `Vec`, [sac code](https://doc.rust-lang.org/src/alloc/vec.rs.html#2096-2102). So, `Deref` means that we are talking about references, since they implement this trait with an [associated array type](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types) `Target = [T]`: this works because when we call `sort` on `Vec`, rust performs the [`deref` coercion](https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods) that is it automatically convert a reference of a type into a reference of another type.

# Key points to remember

- binary application vs library
- primitive obsession
- Test-driven development (TDD) process
- methods ([traits](https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method), struts and [enemas](https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html)) vs functions
- automatic dereferencing
- stack vs heap
- to panic or not
- integration vs unites
- redirect error to std err `cargo run > output.txt` should not contain any errors: `if let Err(e) = erronous_function { eprintln!("ERR"); process:exit(1) }` when you want only to print and exit
- `let A = if mybool { 1 } else { 2 };` is cool, remember the `;` to complete the `let` statement
- shadowing, borrowing, heap and mutability
- smart pointers (`Box` is `unique_ptr` in C++, see [here](https://stevedonovan.github.io/rust-gentle-intro/pain-points.html#shared-references)) vs strict, indirection
- interior mutability
- rust has **not** inheritance: use traits and generics (bounded parametric polymorphism)
- to implement Object-Oriented Design Pattern in rust you transformation of types [see here](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#implementing-transitions-as-transformations-into-different-types)
- blanket implementation
- `collect` with iterators is powerful because you can create several different data objects with the same code, just changing the type of the expected result (see rustling ex `iterators3.rs`).
- RABI
- Compiler Driven Development with `cargo check`
