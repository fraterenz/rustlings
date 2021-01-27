Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. :)

Rust is a systems programming language.

It's easier because you can iterate very often while writing code the write-compile-correct cycle using `cargo run`: this commands will remove lots of bugs by indicating errors at compile time != python. In this way, it's easier to test also, since most of the errors are already caught. :)

Great package managing system `cargo` that allows a identitical and consistent respecting the requirements specified in the `Cargo.tolm` so Rustaceans are able to write smaller projects that are assembled from a number of packages. :)


https://blog.discord.com/why-discord-is-switching-from-go-to-rust-a190bbca2b1f :) :)

## 1.3 Cargo

The purpose of a Cargo.lock is to describe the state of the world at the time of a successful build. It is then used to provide deterministic builds across whatever machine is building the package by ensuring that the exact same dependencies are being compiled.

A library is not only used by the library developers, but also any downstream consumers of the library. Users dependent on the library will not inspect the library’s Cargo.lock (even if it exists). This is precisely because a library should not be deterministically recompiled for all users of the library.

In other words, libraries specify semver requirements for their dependencies but cannot see the full picture. Only end products like binaries have a full picture to decide what versions of dependencies should be used.

Another neat feature of Cargo is that you can run the cargo doc --open command, which will build documentation provided by all of your dependencies locally and open it in your browser :)

## 3.1. Variables and Mutability
There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

**shadowing:** use `let` keyword on the same name (e.g. `x`): the variable is immutable (so you can't reassign to a new value without `let`). Used to perform few transformations on a value, by keeping the variable immutable (later on). We’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. REMEMBER we’re not allowed to mutate a variable’s type unless using shadowing (see examples in https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html end of page)! This feature is often used in situations in which you want to convert a value from one type to another type. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as `guess_str` and guess for example. 

## 3.2 Data Types

Rust is a statically typed language. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, must annotate otherwise compiler error.

**Tuples** have fixed lenght. Can be unpacked as in python. python `__getitem__` is `.`

**Arrays** have fixed lenght. They have all elements of the same type. Array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array. You can init an arry like this `let a = [3;5]` that will create an array of 5 elements, all 3s.  python `__getitem__` is `[]` like in python. Invalid indexing (like out of bound) does not result in compilation error but in panicking (runtime error). This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing :). Iterating on arrays, see loops section.

**range:** `(1..4)`

**strings:** are mutable, != literals which are not. E.g. `let mut s = String::from("hello");` is string used when we dont know the value while coding. `let s = "hello";` is a litteral, immutable, like C++ if I remember correctly.

## 3.3 Functions

`fn` keyword to declare a function.

**Statements** are instructions that perform some action and do not return a value. Function definitions are also statements. You can't do `x = y = 6` since `y = 6` does not return anything != C.
 
**Expressions** evaluate to a resulting value. if is an expression. The block that we use to create new scopes, `{}`, e.g.: 
```
let y = {
	let x = 3;
	x + 1  // do not use ';' else if won't return anything, it will become a statement instead of an expression?
};
```
so `y` wil be equal to `4`, != python.

**ternary operator:** `let y = if condition { 5 } else { 3 };`. Remember this is wrong `let number = if condition { 5 } else { "six" };` because `if and else have incompatible types` :)

#### Functions with Return Values

Used `->` to define the type of the value returned. In `{}` block, **no semicolon because it’s an expression whose value we want to return!!**, see **expression** example

Rust will not automatically try to convert non-Boolean types to a Boolean, != python :(

#### Static methods

The `::` syntax in the `::new` line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String.

## 3.5 Looping

3 loops available: `loop`, `for` and `while`.

**looping over arrays:** `for element in a.iter()`

# 4 Memory management

## Summary
With data on the heap (that is that do not have the `Copy` trait) you have two options, both involves creating a new pointer:

1. **moving:** the new created pointer takes the ownership. Rust copies the pointer (shallow copy) + invalidates the 1st pointer BUT NOT THE DATA POINTED! If you try to use the invalidated pointer, you'll get the value borrowed after move, see [Figure4-2](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html) and see rustling/scratch/moving1.rs

2. **borrowing:** the new created pointer does not takes the ownership. It cannot modify the object (aka cannot borrow as mutable) unless the pointer has been defined as mut (like `&mut` or without any reference as in `rustling/move_semantics3.rs`). See [Figure4-5](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)

So, with a string s, depending on what you want to do, you can: 
1. reading (&s): just need to read the data (borrowing)
2. mutating (&mut s): read to write the data (borrowing mutable, only 1 owner at the time)
3. consuming (s): the variable wont be needed later on (moving)

**Ownership rules:** value is a name bound to an object,
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time for each value.
3. When the owner goes out of scope, the value will be dropped.

When data is on the heap, the value `s1` bound to the data is only a pointer! Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance. To create deep copies `clone` trait.

## Book

Memory is managed through a system of ownership with a set of rules that the compiler checks at **compile** time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

## Stack & heap: ownership
All data stored on the stack must have a known, fixed size, e.g. int, floats, arrays, tuples. Data with an unknown size at compile time or a size that might change must be stored on the heap instead, i.e. string. Types such as integers that have a known size at compile time are stored entirely on the stack.

Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; that location is always at the top of the stack.

Objects' memory is freed whenever the scope of the variable of the object is out of scope -> `drop` is called and the memory associated to the variable is freed. `drop` is similar to a destructor.

**Managing heap data is why ownership exists.**

**moving vs borrowing:** from [this good blog](https://hashrust.com/blog/memory-safety-in-rust-part-2/). when using obects stored on the heap (e.g. String `s1` and `s2`), assignment `s2 = s1` creates a shallow copy + invalidates `s1`, and thus `s1` does not exist anymore, since it has been "moved" to `s2`: you cant use `s1` but you can use `s2`. !! THIS IS NOT TRUE FOR PRIMITIVE LIKE INTEGERS THAT STAY ON THE STACK (AND NOT ON THE HEAP) AND THUS HAVE THE COPY TRAIT! [see here](https://hashrust.com/blog/moves-copies-and-clones-in-rust/). Avoid 1st memory safety bug *double free* error :) When you say reference you say borrowing, but the opposite is not true, for instance a slice is another type that borrows but is not a reference. But remember **moving** (move the owner) != **borrowing** (do not move the owner), eg: 
```
let v: Vec<i32> = Vec::new(); 
let v1 = v;//v1 is the new owner 
``` != 
```
let v: Vec<i32> = Vec::new();
let v1 = &v;//v1 has borrowed from v
v.len();//fine
v1.len();//also fine
```. That is you can only have 1 owner and the ownership of one object can be moved to another obect; in borrowing, multiple object can ref the same object. But:
1. a borrower cannot access the resource after the owner has destroyed it, avoid **use-after-free bug** :)
2. **mutable borrowing:** by default borrowing is immutable, to specify mutability you need `mut` everywhere: in the init of object you're borrowing, in the signature function and in the call -> although there can be multiple shared references, there can only be one mutable reference at one time, avoid **dangling pointers**
3. The scope in which the variable s is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership. When functions have references as parameters instead of the actual values, we won’t need to return 

**clone:** create a deep copy -> copy into the heap, and due to deep copying, both v and v1 are free to independently drop their heap buffers. So you can have the famous:

```
let v: Vec<i32> = Vec::new();
let v1 = v.clone();//ok since Vec implements Clone
println!("v's length is {}", v.len());//ok
```

**copy:** If a type has the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait. All simple scalar types have the `Copy` trait.

**functions:** Passing a variable to a function will move (for complex object stored on the heap?) or copy (for simple object stored on the stack?)

## Data types with no owernship: references

Values tied to the underlying data but with no owernship: eg string slice
```
    let s = String::from("Hello world")
    let myslice = &s[..5]
```
`myslice` is strongly tied to the data. The compiler will ensure the references into the String remain valid because of the rules of references (no dangling, no data race).

**references:** a data type that does not have owernship, when you dont want to take ownership, and we just want to read the data in the struct, not write to it. If you want to write with references you need `&mut`. Similar to C++, you can have reference to an object e.g. `string& s` is a point to a string object; doing this `s` does not take the ownership of the pointed object. BUT you can only read, not modify it. To modify it, you need a mutable reference but **an objected can be referenced only once in a reference scope**: `let r1 = &mut s; let r2 = &mut s;` wont compile since s is referenced twice. 2nd memory safty bug fix: avoid data race that is 2 pointers writing to the same object :) But We also **cannot have a mutable reference while we have an immutable one**. Reference’s scope starts from where it is introduced and continues through the last time that reference is used, see https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references last listing.

The rules of references are (so called **borrowing rules**):

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid: no dangling, no data race.

`s.clear()` needs and gets a mutable reference: borrowing!

**dangling pointer:** a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory THAT IS cannot reference to a dropped object (out of scope).

**data race:** these 3 conditions at the same time: 1. no mechanism being used to synchronize access to the data AND 2. Two or more pointers access the same data at the same time AND 3. > 1 pointer used to write the data. THAT IS: 2 writing on the same object.

**slice:** another data type without ownership: used to reference a **contiguous** sequence of elements in a collection rather than the whole collection. Internally in a string slice, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. string literals *are* string slices: `let s = "Hello world"` is an immutable pointer to the binary. **It's usually better to use a string slice instead of a string as param of a fn**, see [example in the book](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)

# 5 Structs

If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

**Using the Field Init Shorthand when Variables and Fields Have the Same name**

**tuple structs** like named tuple in Python. Each struct you define is its own type.

you can't have references in the attribute of a struct unless you use **lifetimes** chap10. All attributes must be owned by the struct, so for the String you should pass the string itself not a ref to a string (for which you need lifetime). We want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

**associated functions:** used as constructor of the struct that is return a new instance of the struct, do not have the `self` argument. The `::` syntax is used for both associated functions and namespaces created by modules, eg. `let sq = Rectangle::square(3);`

# 6 Enums and pattern Matching

You can have methods on Enums as on Struct with the `impl` keyword

`Option`: value could be something or it could be nothing. **Null** does not exist in Rust. It is encoded with an Enums called `Option` that can either be whatever you want, or a None. This is the only way to have a Null value in your code, specify it with the Enums: `let absent_number: Option<i32> = None;` You have to convert an `Option<T>` to a `T` before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle

The `match` expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value. `match` is like an `if` that can return any kind of value (not only Boolean as `if`). When the match expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm, much as in a coin-sorting machine. Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. 

Rust also has a pattern ( `_` ) we can use when we don’t want to list all possible values. 


The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest: In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.


# Chap 7: organization

A crate is file with some code; can be a binary (src/main.rs) or a library (src/lib.rs). Each crate has a crate root: src/main.rs is the crate root of a binary crate with the same name as the package; similary, src/lib.rs is the crate root of a library crate with the same names as the package. Cargo passes the crate root files to rustc to build the library or binary. A package can have multiple binary crates by placing files in the src/bin directory.

# Extra what I've learnt

- Try to do it general: `first_world(&str) -> &str` works with string litteral as well as with String: see [here](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)

- When parameters in a function are related but separated, that is not good! area(width, height) not good since the 2 parameters are related (we want to compute the area of a rectangle, not of an height with width), from [here](https://doc.rust-lang.org/book/ch05-02-example-structs.html)


# Chap 15: smart pointers

Smart pointers are similar to C++ smart pointers: they are stored on the stack but they point to data on the heap and they own the data. Smart pointers are implemented with Struct with the `Deref` and `Drop` traits:

- Deref: used to write code that works with either references or smart pointers, e.g. `assert_eq!([2, 3, 4], nice_slice)` where `nice_slice` is a slice, that is a reference on a vec: `&a[1..4]`, where `a` is a vec. See ` rustlings/exercises/primitive_types/primitive_types4.rs`. `deref` is like in C++: the operator `*` which is the inver operation of the ref operator `&`. To enable dereferencing with the `*` operator, we implement the `Deref` trait, which is used to implement pointers then.

- Drop: destructor

**recursive types:**

# Memento

shadowing, borrowing, heap and mutability, difference between smart pointers and normal struct

# Why rust? book

unsafe means that you can write a program that is not well-defined: meaning that you can produce undesired behaviours without raising any errors. Rust unsafe exists.

# TODO

continue the book and continue rustlings: exercises/primitive_types/primitive_types4.rs hint to understand why the test does not take a ref as input but a normal array (should be cap 15 smart pointers, in part 15.2) !

