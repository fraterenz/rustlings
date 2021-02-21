The language takes advantages of the behaviours embedded into the variables' types: choose the type of your variable based on the tasks these variables need to perform. Similar to [C++ operator overloading](https://youtu.be/DnT-LUQgc7s?t=774). For instance, the null pointer example encapsulated into a `Option` enum, see [here](#no-hidden-states).

Rust provides memory safety when resources matter (speed and cpu usage in the system programming field) that is low-level language, memory safe with zero cost abstraction.

# Learning rust
Read the book and at the same time do rustlings, have a look at [half-hour to rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust), do exercism rust track, the [crust of rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) and have a look at the following crates:

1. [ndarray](https://github.com/rust-ndarray/ndarray)
2. [autograd](https://github.com/raskr/rust-autograd)
3. dbug

## Talks

- [Jon Gjengset](https://www.youtube.com/watch?v=DnT-LUQgc7s)
- [Niko Matsakis](https://www.youtube.com/watch?v=jQOZX0xkrWA)

# Algebraic types and pattern matching

Remember that `Option` is a enum (similar to a type) and `Some` and `None` are the values that a variable `Option` can take, so the data is `Some` or `None` but the type is `Option`.
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

# Functions and Methods

There are [3 possible ways to create bindings](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1) between function parameters and arguments:

1. by-value (meaning in Rust that it either takes ownership of the bound value, or makes a copy of it, depending on whether the type of that value implements the `Copy` trait)
2. by-reference
3. by-mutable-reference

`Copy` is a trait indicating a type is “trivially copyable,” meaning it can be copied with only a call to `memcpy`, so all the data contained in the structure is contiguous; there are no pointers to chase. `Copy` tells us that copying a piece of data is fast.

Methods are different from functions, since method are defined on an object (an instance of a `Struct` for instance). However, methods can take ownership (as functions) of `self`, borrow `self` immutably as we’ve done here, or borrow `self` mutably, just as they can any other parameter.

Methods used the keyword `self` as argument since Rust knows the type of `self` is the same of the object due to this method being inside the `impl` context. See also the [automatic dereferencing process](#modern-language).

# Memory management
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
2. Max 1 mut reference or several immutable refs

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
# Modern language
The compiler knowns about:

- tests: `#[test]`, and can automatically access to private fields! The compiler knows also about the doc, also doc tests `///` and `/// assert_eq!(one_more(42), 43)``` and it will compile and run the doc tests too.

- buit-in dependencies and dependecy graph: each time you compile your program, the dependencies in the `.toml` file will be fetched with `cargo build`. Up to date version of the dependency without breaking (since if you specify `1.3.2` it will fecth everything greather than that, but smaller than `2.0.0`, since semantically that version should cause breaking changes in the code).

But, no pre-built libraries: need to build everything from source. This is mainly due to generics `<T>`: if the library I want to use has generics, I need to compile the version of the method for the type I'm using, which is defined in my library.

- automatic referencing and dereferencing: when you have pointers such as `Vec` or `String`, calling `method` on the pointer, `my_vec.method`, will automatically dereference the pointer `my_vec` and call `method` on the pointed object (not like C++ where you have to derefence explicitly doing `(&p1).distance(&p2);`, see [here](https://doc.rust-lang.org/book/ch05-03-method-syntax.html?highlight=impl#wheres-the---operator).

# No hidden states

No null pointer, you need to check the `Option` and the `Result` enums (the latter checked with `?` the try operator: `"42".parse()?` returns error or unwrap). The data type `Option` tells you whether the object could be `None`.

# Data modelling

- *primitive obsession:* when a complex pattern would suite better the problem compared to primitive types, e.g. when two pieces of data are related to each other and instead of being grouped into a Struct, they are stored as tuple or array, see [here](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#grouping-configuration-values).
- if you need to create an object that can be shared by either two different kind of resources (such as shared with one or more recipients **or** received from a source), see [here](https://www.reddit.com/r/rust/comments/l594zl/everywhere_i_go_i_miss_rusts_enums/gkteafc?utm_source=share&utm_medium=web2x&context=3)

# Reading the doc with an example
The `std::iter::Iterator` trait:

1. is associated to the type `Item`
2. `next` is the only required method to use this trait
3. provides other methods
4. has several functions

The function `chain` for instance, takes as input an iterator `self` and generic type that implements the trait `IntoIterator`, and all the types implementing that trait can be found clicking on the trait, linked to the [doc of the trait](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#implementors). It returns a new `Iterator`, a struct `Chain`. Note that it takes the ownership of `self` meaning that it create a new iterator and invalidate the previous one (by consuming it?), that is why in many examples you see `myarray.iter().sum()`, that is they do not create a variable `let myiter = myarray.iter()`, since `myiter` will not be available after the `sum()` call.

`pub fn sum<S>(self) -> S where  S: Sum<Self::Item>,` means that the method returns an object of generic type `S` that implements the trait `Sum`, it does not mean that the method returns the `Sum` trait.

# Key points to remember

- binary application vs library
- primitive obsession
- Test-driven development (TDD) process
- methods vs functions
- automatic dereferencing
- stack vs heap
- to panic or not
- integration vs unittest
