# Memory management
With data on the heap (that is that do not have the `Copy` trait) you have three options, both involves creating a new pointer:

1. **moving:** the new created pointer takes the ownership. Rust copies the pointer (shallow copy) + invalidates the 1st pointer BUT NOT THE DATA POINTED! If you try to use the invalidated pointer, you'll get the value borrowed after move, see [Figure4-2](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html) and see rustling/scratch/moving1.rs

2. **borrowing:** the new created pointer does not takes the ownership. It cannot modify the object (aka cannot borrow as mutable) 

3. **borrowing mutably:** there can be only one pointer accessing the data in the same scope (but there can be two pointers sequentially), see [video Gary](https://youtu.be/79phqVpE7cU?t=50. You can achieve this by either creating a mutatble ref `&mut` or in the prototype of a function, as in `rustling/move_semantics3.rs`. See [Figure4-5](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html).

Scope means `{}` but also the last time a binding is mentioned (again as in [Gary's video](https://youtu.be/79phqVpE7cU?t=506).

So, with a string s, depending on what you want to do, you can: 
1. reading (&s): just need to read the data (borrowing)
2. mutating (&mut s): read to write the data (borrowing mutable, only 1 owner at the time)
3. consuming (s): the variable wont be needed later on (moving)

**Ownership rules:** value is a name bound to an object,
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time for each value.
3. When the owner goes out of scope, the value will be dropped.

When data is on the heap, the value `s1` bound to the data is only a pointer! Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance. To create deep copies `clone` trait.


# Data modelling

if you need to create an object that can be shared by either two different kind of resources (such as shared with one or more recipients **or** received from a source), see [here](https://www.reddit.com/r/rust/comments/l594zl/everywhere_i_go_i_miss_rusts_enums/gkteafc?utm_source=share&utm_medium=web2x&context=3)

