Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. :)

Rust is a systems programming language.

It's easier because you can iterate very often while writing code the write-compile-correct cycle using `cargo run`: this commands will remove lots of bugs by indicating errors at compile time != python. In this way, it's easier to test also, since most of the errors are already caught. :)

Great package managing system `cargo` that allows a identical and consistent respecting the requirements specified in the `Cargo.tolm` so Rustaceans are able to write smaller projects that are assembled from a number of packages. :)


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

**Tuples** have fixed length. Can be unpacked as in python. Python `__getitem__` is `.`

**Arrays** have fixed length. They have all elements of the same type. Array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array. You can init an array like this `let a = [3;5]` that will create an array of 5 elements, all 3s.  Python `__getitem__` is `[]` like in python. Invalid indexing (like out of bound) does not result in compilation error but in panicking (runtime error). This is the first example of Rust’s safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing :). Iterating on arrays, see loops section.

**range:** `(1..4)`

**strings:** are mutable, != literals which are not. E.g. `let mut s = String::from("hello");` is string used when we don't know the value while coding. `let s = "hello";` is a literal, immutable, like C++ if I remember correctly.

## 3.3 Functions

`fn` keyword to declare a function.

**Statements** are instructions that perform some action and do not return a value. Function definitions are also statements. You can't do `x = y = 6` since `y = 6` does not return anything != C.
 
**Expressions** evaluate to a resulting value. If is an expression. The block that we use to create new scopes, `{}`, e.g.: 
```
let y = {
	let x = 3;
	x + 1  // do not use ';' else if won't return anything, it will become a statement instead of an expression?
};
```
so `y` will be equal to `4`, != python.

**ternary operator:** `let y = if condition { 5 } else { 3 };`. Remember this is wrong `let number = if condition { 5 } else { "six" };` because `if and else have incompatible types` :)

#### Functions with Return Values

Used `->` to define the type of the value returned. In `{}` block, **no semicolon because it’s an expression whose value we want to return!!**, see **expression** example

Rust will not automatically try to convert non-Boolean types to a Boolean, != python :(

#### Static methods

The `::` syntax in the `::new` line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String.

## Functions support argument destructuring

Usefull when you would like to rename parameters in your function, see [here](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1):

```
struct Something {
    field_1: i32,
    field_2: f64,
}

// The `field_1: x` and `field_2: y` parts are assigning
// the values of `field_1` to `x` and `field_2` to `y`.
// cool when you want to rename things
fn func(Something { field_1: x, field_2: y }: Something) {
    println!("x: {}, y: {}", x, y);
}
```

instead of passing the structure directly.


## 3.5 Looping

3 loops available: `loop`, `for` and `while`.

**looping over arrays:** `for element in a.iter()`

# 4 Memory management
Memory is managed through a system of ownership with a set of rules that the compiler checks at **compile** time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don't have to write and debug extra code to get this control.

## Stack & heap: ownership
All data stored on the stack must have a known, fixed size, e.g. int, floats, arrays, tuples. Data with an unknown size at compile time or a size that might change must be stored on the heap instead, i.e. string. Types such as integers that have a known size at compile time are stored entirely on the stack.

Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; that location is always at the top of the stack.

Objects' memory is freed whenever the scope of the variable of the object is out of scope -> `drop` is called and the memory associated to the variable is freed. `drop` is similar to a destructor.

**Managing heap data is why ownership exists.**

**moving vs borrowing:** from [this good blog](https://hashrust.com/blog/memory-safety-in-rust-part-2/). When using objects stored on the heap (e.g. String `s1` and `s2`), assignment `s2 = s1` creates a shallow copy + invalidates `s1`, and thus `s1` does not exist anymore, since it has been "moved" to `s2`: you cant use `s1` but you can use `s2`. !! THIS IS NOT TRUE FOR PRIMITIVE LIKE INTEGERS THAT STAY ON THE STACK (AND NOT ON THE HEAP) AND THUS HAVE THE COPY TRAIT! [see here](https://hashrust.com/blog/moves-copies-and-clones-in-rust/). Avoid 1st memory safety bug *double free* error :) When you say reference you say borrowing, but the opposite is not true, for instance a slice is another type that borrows but is not a reference. But remember **moving** (move the owner) != **borrowing** (do not move the owner), eg: 
```
let v: Vec<i32> = Vec::new(); 
let v1 = v;//v1 is the new owner 
```  
is different from:
```
let v: Vec<i32> = Vec::new();
let v1 = &v;//v1 has borrowed from v
v.len();//fine
v1.len();//also fine
```
That is you can only have 1 owner and the ownership of one object can be moved to another object; in borrowing, multiple object can ref the same object. But:
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

A crate is file with some code; can be a binary (`src/main.rs`) or a library (`src/lib.rs`). The functions within a library are used by the binaries usually.

Each crate has a crate root and `src/main.rs` is the crate root of a binary crate with the same name as the package; similarly, `src/lib.rs` is the crate root of a library crate with the same names as the package. Cargo passes the crate root files to rustc to build the library or binary. A package can have multiple binary crates by placing files in the src/bin directory.

The children modules have access to the parent modules' functions, but the opposite is not true. Rust makes everything private by default. When you have a `struct` that have some private fields, you need to create a public constructor.

[Rust's module paths are not tied directly to the file system](https://dev.to/stevepryde/intro-to-rust-modules-3g8k)! When you do `mod cat` in your main, rust will look for either `cat.rs` at the same level of your `main.rs` or for `cat/mod.rs` so a `cat` directory. Remember that everything is private by default in rust.

When you import into some files others than `main.rs` or `lib.rs` you need to add `crate::mymode::myf`, that is the keyword `crate`.

# Extra what I've learnt

- Try to do it general: `first_world(&str) -> &str` works with string literal as well as with String: see [here](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)

- When parameters in a function are related but separated, that is not good! area(width, height) not good since the 2 parameters are related (we want to compute the area of a rectangle, not of an height with width), from [here](https://doc.rust-lang.org/book/ch05-02-example-structs.html)

# Chap8: collections

## Vectors

Vector is a collection containing smart pointers [with some metadata](https://doc.rust-lang.org/nomicon/vec-layout.html). There are 2 ways to access a vector, depending if you want to make the program panicking `&v[i]` or you want silence when the data does not exit `v.get(i)`, where `i = v.len()`.

Vector can handle only data of the same type. To solve this, use enums with vectors, so vectors can store the data according to the enum just defined.

```
enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Text(String::from("blue")),
	SpreadsheetCell::Float(10.12),
	SpreadsheetCell::Float(1.2),
	SpreadsheetCell::Int(4),
];
```

## Strings

Are also collections containing smart pointers and metadata, `String` is a wrapper over a `Vec<u8>`. The operator `+` does not take the owernship of its parameter: `let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used but s2 is fine`. When using the macro `format!` same owernships rules as `+`: `format!("{}-{}-{}", s1, s2, s3);` consumes `s1`. 

Strings do not support indexing! The reason for this is that the number of bytes 

String can be interpreted by rust as bytes, scalr values or grapheme clusters (letters). Almost always go for bytes: `mystr.bytes()` but you can use also `chars()` if you want a human to read the string in char.

## Hash Maps

`HashMap<K, V>`, takes a tuple as argument and they own their data (consume). Hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type. To init a Hash Map:

```
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
	teams.into_iter().zip(initial_scores.into_iter()).collect();
```
To retrieve the value use `get` which takes as param a reference `&V` and returns a `Option<&V>`. 

The `get` in python is achieved using `entry` API which returns a enum `Entry` which can have two possible values: an enum called `OccupiedEntry` or `VacantEntry` see [here](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html). Then we can call `.or_insert(30)` on this `Entry` in order to either retrieve a mutable ref on the original value or on the new allocated value `30`. REMEMBER to deref if you want to use it, since it is a reference:

```
for word in text.split_whitespace() {
	let count = map.entry(word).or_insert(0);
	*count += 1; // deref since or_insert returns a mut ref
}
```
# Chap9: Error handling

The operator `?` can be used only with function returning an type that implements `std::ops::Try`, mainly `Result` or `Option`. Therefore, if you want to use it in your `main`, you need to change the main such that it returns a `Result`:  `main() -> Result<(), Box<dyn Error>>`, which is a collection of pointers pointing to errors or results.

`expect`, `unwrap` and `unwrap_else` are shortcuts for `match` expressions: they evaluate the `Result` by either panicking or returning the value associated to `T` (remember that `Result` is a enum of generics `Result<T, E>`).

When you `panic!` no way to recover, stop the program. On the other hand, using `Result` will allow the calling function to decide whether to recover or to `panic!`, such as:

```
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

```
or similarly,

```
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Error"),
        };

```
Remember that the operator `?` can be used to simply the pattern matching, but need to be used in a function that returns `Result` because it returns either an `Err` or a `Ok`, and must be placed after a function that returns `Result`, such as `item_quantity.parse::<i32>()?`. Indeed, if the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code. 

You can also try catch error with the `if let Ok(value)`:

```

fn main() {
    if let Ok(s) = std::str::from_utf8(&[195, 40]) {
        println!("{}", s);
    } else {
        println!("Yo")
    }
}
```

does not panick, instead print `Yo`.

# Chap10 

## 10.1 Generics

The definition of this function `fn largest<T>(list: &[T]) -> &T {` must be read as following: the function `largest` is generic over some type `T`. Remember that `T` means that it could be any type, but it must be consistent within the function. For instance, consider the structure `std::collections::HashMap`, it is defined as `pub struct HashMap<K, V, S = RandomState> { /* fields omitted */ }`, meaning that for one instance of this structure, all the keys must be of the same type, and similarly all the values too. Therefore, if you try to insert into a `HashMap` of `string, integer` an item `string, float` since this particular object contains only `string` as keys and `int` as values (!= python).

When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.

```
struct Point<T> {
    x: T,
    y: T,
}
     |----- this type in the angle brackets tells the compiler that T in Point is a generic type rather than a concrete type
     |        |----- specify that we’re implementing methods on the type Point<T>
     v        v
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

No performance overhead at runtime since Rust performs monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with, for instance it transforms `Option<T>` into `Option_i32` and `Option_f64` when writing `Option(3)` and `Option(3.2)`. 

## Traits

Traits are something multiple types can have in common. They are called `interfaces` in other programming languages and they are similar to the abstract virtual method in C++: if you use a trait for a type, you need to implement the trait for that type otherwise the code will not compile (similar to subclasses of abstract classes in C++). Traits obey to the orphan rules, that is you can only implement:

1. one of your traits on anyone's type
2. anyone's trait on one of your types (operator overloading, see [here](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) and search for *type Output = Number;*)
3. but not a foreign trait on a foreign type

**default implementations:** you can use default implementations (!= C++ abstract classes). And you call the default implementation of a trait for a type overriding the implementation of that trait (similar to using void methods but without a reference in C++, that is using the resolution statique des liens, see [here](https://youtu.be/_2pt_aZ1EjM?t=336)).

**methods for objects implementing specific trait:** traits can be used to create methods that work only with instances of types that implement that trait: `pub fn notify(item: &impl Summary)`, `notify` can be called with any objects implementing the trait `Summary`. The notation `impl` is syntactic sugar for pub `fn notify<T: Summary>(item: &T)`. When you have more than 1 argument with the same trait it is better to use that notation: `pub fn notify<T: Summary>(item1: &T, item2: &T)` instead of `pub fn notify(item1: &impl Summary, item2: &impl Summary)`.

Can specify more than 1 trait: `pub fn notify(item: &(impl Summary + Display))` or equivalently `pub fn notify<T: Summary + Display>(item: &T)`. For many traits you can use the `where` clause: 

```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
```
You can also return traits: `fn returns_summarizable() -> impl Summary`. 

By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits, see [here](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods). We can also use the notation `impl<T: Display> ToString for T` which means implement the trait `ToString` for all types defining the trait `Display`. This is called *blanket implementations*.

## Lifetimes
Lifetimes are here to avoid dangling references. Before reading the book, watch [Doug's video](https://www.youtube.com/watch?v=1QoT9fmPYr8).

One and only one owner of memory, but you can have different references to the same memory, it is independent of the data type, but used on references. Lifetimes ensure that this behaviour is safe, for all the references there is a lifetime (explicit or implicit). First example from [Doug](https://youtu.be/1QoT9fmPYr8?t=130), lifetimes are used to ensure that memory does not cleaned up before a reference can use it:

```
fn main() {
    let a;
    {
        b = String::from("Meow");
        // move b into a
        a = b;
        // but if we had used instead a ref to b, a = &b
        // error since the owner of Meow is still be, since no
        // moved occured, and once b goes out of scope it is 
        // dropped and a points to garbage!!
    }  // here b goes out of scope, but Meow still exists
    // because it was moved into a
    // a is still valid here and points to Meow
    println!("{}", a);
}
```
Lifetime denoted by `'a`. Lifetimes can be compared `'a > 'b`, when a variable outlives a reference lifetime that points to it, it is a problem, since the variable will be bound to a invalid reference. For instance, comparing two string and returns the longest, using references that is string slices. When using two arguments of a function that are references, you usually need to specify that both references have the same lifetime, because rust by default [puts different lifetimes to different references](https://youtu.be/1QoT9fmPYr8?t=1045). Lifetime `'static` ensures that the reference live for the whole code, useful in parameters of functions.

There are 3 rules that the borrow checker applies in order to assign a lifetime to each reference:

1. Each reference has one lifetime
2. If a function has only one input reference, then the output reference will have the same lifetime of the input reference
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters

# Tests
Tests fail when they panic. When using macros such as `assert_eq` or `assert_neq` you must be sure that the values tested implements both the `Debug` and the `PartialEq`traits. Macro attribute `should_panic(expected = "Guess value must be less than or equal to 100")]` with the message expected that we want to test. 

# Chap13: Closures
Unlike functions, closures can capture values from the scope in which they’re defined. Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario.


Interesting example of closures + `struct` + `HashMap` in [chap13.1, the `Cacher`](https://doc.rust-lang.org/book/ch13-01-closures.html#limitations-of-the-cacher-implementation).

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing mutably, and borrowing immutably. When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. 

# Chap15: Smart pointers

Smart pointers in C++ will free the memory automatically. `Vec` and `String` are examples of smart pointers in rust, which are basically structs implementing the `Deref` and `Drop` traits:

- Deref: `deref` is like in C++: the operator `*` which is the inverse operation of the ref operator `&`. Used to write code that works with either references or smart pointers, e.g. `assert_eq!([2, 3, 4], nice_slice)` where `nice_slice` is a slice, that is a reference on a vec: `&a[1..4]`, where `a` is a vec. See ` rustlings/exercises/primitive_types/primitive_types4.rs` [here](https://github.com/fraterenz/rustlings/blob/exercices/exercises/primitive_types/primitive_types4.rs). 

- Drop: destructor, used to free the memory

They are stored on the stack but they point to data on the heap and they own the data, on the contrary to references do not own the data! By the way, this is exactly as in C++, since the references need to point to a variable (cannot be null). 

## Boxes
`Box<T>` are stored on the stack, but the data pointed is on the heap. In C++ analogy, `let b = Box::new(3)` will correspond to the call `shared_ptr<int> p(new int(3))` which allocates some memory (on rust on the stack) for `p` knowing that it will point to some int (allocation dynamique de la memoire); then, at runtime, create a int variable (on rust at compile time?? I think at runtime, on the heap) with value of 3 and stores its address into `p`. Once deleted `p` the memory is deallocated of both `p` (and `p` is set to `nullptr`) and the `int` variable with value of 3.

Used to create recursive types, that are types representing types that have no fixed size at compile time. Since rust is a strongly-typed programming language, the compiler needs to know the required size for each **type**, using `Box` the compiler will store the pointer on the stack referring to some memory on the heap. It is like `String` that is a struct that will point to some data on the heap because it is not known at compile time: user can insert whatever he wants at runtime. 

See this example: 
```
let x = 5;
let y = Box::new(x);
```
here the value pointed by `y` is another `int` (no `x`) that has the same value of `x`, that is `5`. Watch out for moves when `x` is stored on the heap, because in that case you invalidate the variable `x`, since it has been moved to `y`.

## Determination of the space required by a non-recursive type

Rust goes through each of the variants to see which variant needs the most space, because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
Boxes provide only the indirection and heap allocation.

## Deref

Remember that `*p` means content pointed by a pointer and `&v` means a pointer (address to a value):

- `*`: input is pointer, returns value
- `&`: input is value, returns pointer

In rust you have deref coercion for all types implementing the `Deref` trait, such as `String`: it converts such a type into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `str`. The `Deref` trait to override the `*` operator on immutable references because instead of returning the value, it will return a call to the `deref` method and then the value on this reference.

## Drop

Destructor, variables are dropped in the reverse order of their creation. Explicit destructor calls not allowed, to avoid the double free error because Rust would be trying to clean up the same value twice, to do that use `std::mem::drop`.

## Reference counting

To enable multiple ownerships for reading data only, `Rc<T>` keeps track of the number of references to a value which determines whether or not a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid. If `b` and `c` share the same data `a`, we create `b`, instead of taking ownership of `a` (which results into an error when creating `c` because `value a used after move in b`), we’ll clone the pointer `Rc<List>` that `a` is holding, thereby increasing the number of references from one (`a`) to two (`a` and `b`) and letting `a` and `b` share ownership of the data in that `Rc<List>`. We’ll also clone `a` when creating `c`, increasing the number of references from two to three. Every time we call `Rc::clone`, the reference count to the data within the `Rc<List>` will increase, and the data won’t be cleaned up unless there are zero references to it. The call `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of clone do. To count the number of references pointing to some data pointed by a `Rc` you can call `Rc::strong_count(&a)`. 

## Summary


- `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at **runtime**.
 - Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable, but the rules of borrowing still apply at runtime (meaning for instance that you cannot have [2 mutable references](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#keeping-track-of-borrows-at-runtime-with-refcellt)), **interior mutability**

# Chap16: Concurrency

A process is used to run a program which can have multiple threads that can run simultaneously. There is no guarantee on the order in which threads run! The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish. Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. 

Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to the variable `v` created into the main thread will always be valid, since the main thread could terminate before the spanning thread and thus drop the variable `v`, that is the spawned thread may outlive the main thread. But you can `move` the variable `v` from the main thread into the spawned thread, and `v` will be invalidated into the main thread.

To avoid moving a value from main thread to spawned thread, you can create a `Arc` which allows multiple owners that can read only. Similar to `Rc`, you can have multiple ownership in different threads by cloning the shared value. However, since `Rc` is not thread safe, because X, you need to use `Arc` which is safe. To allow multiple ownerships clone the value `Arc::clone(&a)`. To get several mutable ownerships use `Mutex` that provides interior mutability in spawned threads using `Arc`.

# OOP in rust
No polymorphism in rust, but you can achieve dynamic dispatching (resolution dynamique des liens) using heterogenous collections like in C++, which are defined by `Box<dyn Draw>` where `Draw` is a trait. In C++ dynamic dispatch is obtained using two ingredients: pointers + virtual methods. I think the mechanism used by rust is the same as in C++, because virtual methods looks like traits, and `Box` is a pointer! The difference maybe is that not all traits can be used to create trait objects, they must obey to the following rules:

1. the return type is **not** `Self`
2. no generic type paramter

To achive static dispatch, you only need generics at trait bounds (`<T: Summary>`, trait bounds used to specify that a generic can be any type that has certain behavior): at compile time, the definitions will be [monomorphized](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics) at compile time to use the concrete types, which means that rust is fast when dealing with generics while using static dispatch. **There is a runtime cost for dynamic dispatch**. Static dispatch is used in C++ by default (like in rust) by looking at the concrete type of the variable.

Example of a program implementing a heterogeneous collection:

```
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}


impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
    pub fn add(&mut self, drawable: Box<dyn Draw>) {
        self.components.push(drawable);
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("I am a Button: {}", self.width)
    }
}

fn main() {
    let mut my_screen = Screen{components: Vec::new()};
	// dynamic allocation
    let my_button = Box::new(
        Button { width: 32, height: 1, label: String::from("world")}
    );
    my_screen.add(my_button);
    my_screen.run();
}
```

# Why rust? book

Unsafe means that you can write a program that is not well-defined: meaning that you can produce undesired behaviours without raising any errors. Rust unsafe exists.

