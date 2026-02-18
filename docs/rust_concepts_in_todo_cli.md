# Rust Concepts in todo_cli

A guide to the key Rust ideas and syntax found in this project, with comparisons to C# and C++ where applicable.

---

## Table of Contents

1. [Ownership, Borrowing, and References](#1-ownership-borrowing-and-references)
2. [Mutability](#2-mutability)
3. [String vs &str](#3-string-vs-str)
4. [Structs and impl Blocks](#4-structs-and-impl-blocks)
5. [Traits](#5-traits)
6. [derive Macros](#6-derive-macros)
7. [Enums: Option and Result](#7-enums-option-and-result)
8. [Pattern Matching with match](#8-pattern-matching-with-match)
9. [The ? Operator](#9-the--operator)
10. [Closures](#10-closures)
11. [Iterators](#11-iterators)
12. [Modules and Visibility](#12-modules-and-visibility)
13. [Testing](#13-testing)
14. [Box\<dyn Error\> and Trait Objects](#14-boxdyn-error-and-trait-objects)
15. [Sources](#15-sources)

---

## 1. Ownership, Borrowing, and References

This is the concept that makes Rust fundamentally different from both C# and C++. Every value in Rust has exactly one **owner**. When the owner goes out of scope, the value is automatically cleaned up (called **dropping**).

**Ownership rules:**
- Each value has a single owner at any time.
- When ownership is transferred (a **move**), the original variable can no longer be used.
- When the owner goes out of scope, the value is dropped.

**In todo_cli** (`main.rs:19`):
```rust
let mut manager = TaskManager::new();  // main() owns manager
```

When `main()` ends, `manager` is dropped automatically. No `delete`, no garbage collector.

**Borrowing** lets you pass a reference to a value without transferring ownership. There are two kinds:

| | Rust | C++ | C# |
|---|---|---|---|
| Immutable reference | `&T` | `const T&` | Passing a `readonly` ref or just passing a reference type |
| Mutable reference | `&mut T` | `T&` | Passing a reference type (all references are mutable in C#) |

**In todo_cli** (`main.rs:81, 90`):
```rust
fn view_tasks(tasks: &[Task], show_task_id: bool)     // borrows tasks immutably
fn complete_task(manager: &mut TaskManager)             // borrows manager mutably
```

**The key rule:** you can have either *one* mutable reference **or** *any number* of immutable references at a time, never both. This is enforced at compile time by the **borrow checker** and prevents data races entirely.

**C++ comparison:** C++ has references (`&`) and pointers (`*`), but nothing prevents you from holding a dangling pointer or having simultaneous mutable access from multiple places. Rust's borrow checker catches these at compile time.

**C# comparison:** C# uses a garbage collector to manage memory, so you never have dangling references to heap objects. But C# doesn't prevent data races on shared mutable state — you need `lock` statements or `Concurrent` collections for that.

> **Read more:** [What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) | [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

---

## 2. Mutability

In Rust, variables are **immutable by default**. You must opt into mutability with `mut`.

**In todo_cli** (`main.rs:19`):
```rust
let mut manager = TaskManager::new();   // mutable — we can call add_task(), remove_task(), etc.
let filename = "task_database.xml";     // immutable — it never changes
```

**In method signatures** (`task_manager.rs:14`):
```rust
pub fn set_tasks(&mut self, new_tasks: Vec<Task>)   // needs mutable access to self
pub fn fetch_tasks(&self) -> &[Task]                 // only needs read access
```

| | Rust | C++ | C# |
|---|---|---|---|
| Immutable by default | `let x = 5;` | `const int x = 5;` | `const int x = 5;` (compile-time only) |
| Mutable | `let mut x = 5;` | `int x = 5;` | `int x = 5;` (mutable by default) |

C++ and C# are mutable by default, const/readonly is opt-in. Rust flips this: immutability is the default, mutability is opt-in. This encourages safer code.

> **Read more:** [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

---

## 3. String vs &str

Rust has two main string types:

| Type | What it is | Analogy |
|---|---|---|
| `String` | Owned, heap-allocated, growable string | Like C# `string` or C++ `std::string` |
| `&str` | Borrowed reference to a string slice | Like C# `ReadOnlySpan<char>` or C++ `std::string_view` |

**In todo_cli** (`models.rs:5-6`):
```rust
pub struct Task {
    pub description: String,    // Task owns this string data
    pub due_date: String,       // Task owns this too
    pub important: bool,
}
```

**In todo_cli** (`task_manager.rs:26`):
```rust
pub fn save_tasks(&self, filename: &str) -> Result<(), Box<dyn Error>>
//                         ^^^^ borrowed — we just need to read the filename, not own it
```

**Rule of thumb:** Use `String` when you need to own and store string data. Use `&str` for function parameters when you just need to read the string.

The `.to_string()` calls you see throughout the code (e.g., `"hello".to_string()`) convert a `&str` literal into an owned `String`.

> **Read more:** [The String Type](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

## 4. Structs and impl Blocks

Rust structs are similar to C# classes/structs and C++ structs but with no inheritance.

**In todo_cli** (`models.rs:4-8`):
```rust
pub struct Task {
    pub description: String,
    pub due_date: String,
    pub important: bool,
}
```

Methods are defined in a separate `impl` block rather than inside the struct definition:

**In todo_cli** (`task_manager.rs:9-37`):
```rust
impl TaskManager {
    pub fn new() -> Self {                    // associated function (like a static method)
        TaskManager { tasks: Vec::new() }
    }

    pub fn fetch_tasks(&self) -> &[Task] {    // method (takes &self)
        &self.tasks
    }
}
```

| Concept | Rust | C++ | C# |
|---|---|---|---|
| Data definition | `struct Task { ... }` | `struct Task { ... };` | `class Task { ... }` |
| Methods | `impl Task { fn foo(&self) }` | Defined inside struct or class | Defined inside class |
| Constructor | `fn new() -> Self` (convention) | Constructor `Task()` | Constructor `Task()` |
| Static methods | `fn new() -> Self` (no `self` param) | `static Task create()` | `static Task Create()` |
| `self` / `this` | Explicit `&self` parameter | Implicit `this` pointer | Implicit `this` reference |

**Key difference:** Rust has no constructors. The convention is a `fn new() -> Self` associated function. There's no `new` keyword — `TaskManager::new()` is just a regular function call that returns a value.

**No inheritance:** Rust structs cannot inherit from other structs. Code reuse is achieved through traits (see below) and composition.

> **Read more:** [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) | [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

---

## 5. Traits

Traits are Rust's version of interfaces. They define shared behavior that types can implement.

**In todo_cli** (`models.rs:10-19`):
```rust
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let important_str = if self.important { "Yes" } else { "No" };
        write!(f, "Description: {}\nDue Date: {}\nImportant: {}",
               self.description, self.due_date, important_str)
    }
}
```

This implements the `Display` trait, which is what allows `println!("{}", task)` to work. It's similar to overriding `ToString()` in C# or `operator<<` in C++.

| Concept | Rust | C++ | C# |
|---|---|---|---|
| Define behavior contract | `trait Display { fn fmt(...) }` | `class IDisplay { virtual ... = 0; }` | `interface IDisplay { string Format(); }` |
| Implement for a type | `impl Display for Task { ... }` | `class Task : public IDisplay { ... }` | `class Task : IDisplay { ... }` |
| Formatting | `Display` trait | `operator<<` overload | `ToString()` override |

**Key advantage over C#/C++:** You can implement a trait for a type you didn't write (called "retroactive conformance"). For example, you could implement your own trait for `String` or `Vec`. In C#, you can't add interface implementations to types from other packages.

> **Read more:** [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html) | [Interfaces — Rust for C#/.NET Developers](https://microsoft.github.io/rust-for-dotnet-devs/latest/language/custom-types/interfaces.html)

---

## 6. derive Macros

The `#[derive(...)]` attribute automatically generates trait implementations for your struct.

**In todo_cli** (`models.rs:3`):
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct Task { ... }
```

| Derive | What it generates | C# equivalent | C++ equivalent |
|---|---|---|---|
| `Debug` | Debug formatting (`{:?}` in print) | Auto-generated in `record` types | No direct equivalent (write your own `operator<<`) |
| `PartialEq` | `==` and `!=` operators | `IEquatable<T>` / `Equals()` | `operator==` |
| `Clone` | `.clone()` method for deep copy | `ICloneable` / copy constructor | Copy constructor |

Without `PartialEq`, you couldn't use `assert_eq!` in tests. Without `Clone`, you couldn't call `.clone()` to duplicate a `Task`.

> **Read more:** [Appendix C: Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

---

## 7. Enums: Option and Result

Rust's enums are far more powerful than C#/C++ enums. They can hold data inside each variant (called "algebraic data types" or "tagged unions").

### Option\<T\>

Represents a value that might not exist. Rust has **no null** — `Option` is the replacement.

```rust
enum Option<T> {
    Some(T),    // there is a value
    None,       // there is no value
}
```

**In todo_cli** (`task_manager.rs:31-37`):
```rust
pub fn remove_task(&mut self, task_index: usize) -> Option<Task> {
    if task_index < self.tasks.len() {
        Some(self.tasks.remove(task_index))
    } else {
        None
    }
}
```

| Concept | Rust | C# | C++ |
|---|---|---|---|
| "No value" | `Option::None` | `null` / `Nullable<T>` | `std::nullopt` / `nullptr` |
| "Has value" | `Option::Some(value)` | The value itself | `std::optional<T>` |
| Check + extract | `match`, `if let`, `.unwrap()` | Null check (`if (x != null)`) | `.has_value()` / `.value()` |

The advantage: Rust forces you to handle the `None` case at compile time. You can never get a NullReferenceException.

### Result\<T, E\>

Represents an operation that can succeed or fail. This is Rust's primary error handling mechanism.

```rust
enum Result<T, E> {
    Ok(T),      // success, contains the value
    Err(E),     // failure, contains the error
}
```

**In todo_cli** (`xml_parser.rs:9`):
```rust
pub fn read(filename: &str) -> Result<Vec<Task>, Box<dyn Error>>
//                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//                              Ok = Vec<Task>, Err = any error type
```

| Concept | Rust | C# | C++ |
|---|---|---|---|
| Success | `Ok(value)` | Return the value | Return the value |
| Failure | `Err(error)` | `throw new Exception(...)` | `throw std::exception(...)` |
| Handle error | `match` / `?` operator | `try/catch` | `try/catch` |

Rust doesn't have exceptions. Errors are values that must be explicitly handled, which makes it impossible to accidentally ignore an error.

> **Read more:** [The Option Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) | [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

---

## 8. Pattern Matching with match

`match` is like a `switch` statement on steroids. It can destructure enums, tuples, and structs, and the compiler ensures all cases are handled.

**In todo_cli** (`main.rs:31-48`):
```rust
match command.as_str() {
    "new" => match create_new_task() {
        Ok(task) => manager.add_task(task),
        Err(e) => {
            eprintln!("Error creating task: {e}");
            process::exit(1);
        }
    },
    "view" => { view_tasks(manager.fetch_tasks(), false); }
    "done" => { complete_task(&mut manager); }
    other => {                          // catch-all, binds the value to `other`
        eprintln!("Unknown command '{other}'.");
        process::exit(1);
    }
}
```

**Destructuring enums** (`xml_parser.rs:19-20`):
```rust
match event? {
    XmlEvent::StartElement { name, .. } => { ... }  // destructure, ignore other fields with ..
    XmlEvent::EndElement { name } => { ... }
    XmlEvent::Characters(text) => { ... }            // extract the inner String
    _ => {}                                           // ignore everything else
}
```

**let-else pattern** (`main.rs:14-17`):
```rust
let Some(command) = args.get(1) else {
    eprintln!("Usage: todo_cli <new|view|done>");
    process::exit(1);
};
```

This tries to match `args.get(1)` (which returns `Option<&String>`) against `Some(command)`. If it's `None`, the `else` block runs. This is a concise way to "unwrap or exit."

**if let** (`xml_parser.rs:34`):
```rust
if let Some(task) = current_task.take() {
    task_list.push(task);
}
```

This is a shorthand for when you only care about one variant.

| | Rust | C# | C++ |
|---|---|---|---|
| Basic | `match x { ... }` | `switch (x) { ... }` | `switch (x) { ... }` |
| Exhaustiveness | Compiler-enforced (must handle all cases) | Not enforced | Not enforced |
| Destructuring | Yes, deeply nested | Limited (C# 8+ pattern matching) | No |
| Bind values | `other => ...` captures the value | `var x when ...` (C# 8+) | No |

> **Read more:** [The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html) | [Concise Control Flow with if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)

---

## 9. The ? Operator

The `?` operator is syntactic sugar for propagating errors. If the expression is `Err`, return early with that error. If `Ok`, unwrap the value.

**In todo_cli** (`main.rs:59`):
```rust
fn create_new_task() -> Result<Task, std::io::Error> {
    let description = read_trimmed_line()?;   // if Err, return the error immediately
    // ...
}
```

This is equivalent to:
```rust
let description = match read_trimmed_line() {
    Ok(val) => val,
    Err(e) => return Err(e),
};
```

**C# equivalent:**
```csharp
// C# doesn't have a direct equivalent. The closest patterns are:
// 1. Exceptions (which propagate automatically via throw)
// 2. The null-conditional operator ?. (but for null, not errors)
```

**C++ equivalent:**
```cpp
// No equivalent. C++ uses exceptions or manual error code checking.
```

The `?` operator can only be used in functions that return `Result` (or `Option`). This is why `create_new_task()` has `-> Result<Task, std::io::Error>` as its return type.

> **Read more:** [A Shortcut for Propagating Errors: the ? Operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)

---

## 10. Closures

Closures are anonymous functions that can capture variables from their surrounding scope.

**In todo_cli** (`xml_parser.rs:76-81`):
```rust
let mut write_field = |name: &str, data: &str| -> Result<(), Box<dyn Error>> {
    writer.write(XmlWriteEvent::start_element(name))?;
    writer.write(XmlWriteEvent::characters(data))?;
    writer.write(XmlWriteEvent::end_element())?;
    Ok(())
};
```

This closure captures `writer` from the surrounding scope by mutable reference, then is called like a regular function: `write_field("Description", &task.description)?;`.

| | Rust | C# | C++ |
|---|---|---|---|
| Syntax | `\|x, y\| { body }` | `(x, y) => { body }` | `[&](auto x, auto y) { body; }` |
| Capture | Automatic (borrow, mut borrow, or move) | Automatic (by reference) | Explicit (`[&]`, `[=]`, `[x]`) |
| Type | Each closure has a unique, anonymous type | `Func<T>` / `Action<T>` | Each lambda has a unique type |

> **Read more:** [Closures: Anonymous Functions that Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html)

---

## 11. Iterators

Rust iterators are lazy and chainable, similar to C# LINQ or C++ ranges.

**In todo_cli** (`main.rs:82`):
```rust
for (i, task) in tasks.iter().enumerate() {
```

- `.iter()` creates an iterator that borrows each element.
- `.enumerate()` wraps each item into a `(index, value)` tuple.
- The `for` loop destructures each tuple into `(i, task)`.

**Another example** (`main.rs:11`):
```rust
let args: Vec<String> = std::env::args().collect();
```

`.collect()` consumes an iterator and gathers results into a collection. The `: Vec<String>` type annotation tells Rust what to collect into.

| | Rust | C# | C++ |
|---|---|---|---|
| Enumerate | `.iter().enumerate()` | `.Select((item, i) => ...)` | ranges + `views::enumerate` (C++23) |
| Collect into list | `.collect::<Vec<_>>()` | `.ToList()` | ranges `\| ranges::to<vector>()` |
| Lazy by default | Yes | Yes (LINQ) | Yes (ranges) |

> **Read more:** [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

---

## 12. Modules and Visibility

Rust's module system controls code organization and what is visible outside a module.

**In todo_cli** (`main.rs:1-3`):
```rust
mod models;          // declares the module, loads from src/models.rs
mod task_manager;
mod xml_parser;
```

**Imports** (`main.rs:5-8`):
```rust
use crate::models::Task;            // bring Task into scope from the crate root
use std::io::stdin;                  // from the standard library
use task_manager::TaskManager;       // relative path (sibling module)
```

**Aliased import** (`task_manager.rs:2`):
```rust
use crate::xml_parser::write as write_to_xml;   // rename to avoid ambiguity
```

**Visibility:** Everything is private by default. Add `pub` to make it public.

```rust
pub struct TaskManager {    // struct is public
    tasks: Vec<Task>,       // field is private (no pub) — only this module can access it
}
```

| | Rust | C# | C++ |
|---|---|---|---|
| Private by default | Yes | No (`internal` by default for classes) | No (struct members `public` by default) |
| Public keyword | `pub` | `public` | `public:` |
| File = module | `mod foo;` loads `foo.rs` | `namespace` + `using` | `#include` |
| Import | `use std::io::stdin;` | `using System.IO;` | `#include <iostream>` |

> **Read more:** [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## 13. Testing

Rust has a built-in test framework. Tests live alongside the code they test, inside a `#[cfg(test)]` module.

**In todo_cli** (`task_manager.rs:40-113`):
```rust
#[cfg(test)]                    // only compiled when running tests
mod tests {
    use super::*;               // import everything from the parent module

    #[test]                     // marks this function as a test
    fn test_new_creates_empty_manager() {
        let manager = TaskManager::new();
        assert!(manager.fetch_tasks().is_empty());
    }

    #[test]
    fn test_set_and_fetch_tasks() {
        let mut manager = TaskManager::new();
        let tasks = sample_tasks();
        manager.set_tasks(tasks.clone());
        assert_eq!(manager.fetch_tasks(), tasks);   // requires PartialEq
    }
}
```

| | Rust | C# | C++ |
|---|---|---|---|
| Test attribute | `#[test]` | `[Test]` (NUnit) / `[Fact]` (xUnit) | `TEST()` (Google Test) |
| Assert equal | `assert_eq!(a, b)` | `Assert.AreEqual(a, b)` | `EXPECT_EQ(a, b)` |
| Assert true | `assert!(expr)` | `Assert.IsTrue(expr)` | `EXPECT_TRUE(expr)` |
| Run tests | `cargo test` | `dotnet test` | `ctest` or test runner |
| Test location | Same file, `#[cfg(test)]` module | Separate project | Separate file |

The `use super::*;` line imports everything from the parent module (including private items), which is why tests can access private fields and functions.

> **Read more:** [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)

---

## 14. Box\<dyn Error\> and Trait Objects

**In todo_cli** (`xml_parser.rs:9`):
```rust
pub fn read(filename: &str) -> Result<Vec<Task>, Box<dyn Error>>
```

Breaking this down:
- `dyn Error` — a **trait object**: any type that implements the `Error` trait. The `dyn` keyword means "dynamic dispatch" (resolved at runtime via a vtable).
- `Box<dyn Error>` — a heap-allocated trait object. `Box` is needed because trait objects have no known size at compile time.

This is Rust's way of saying "this function can return any kind of error." It's similar to:
- **C#:** Returning `Exception` (the base class of all exceptions).
- **C++:** Catching `std::exception&` (the base class).

| | Rust | C# | C++ |
|---|---|---|---|
| Type-erased error | `Box<dyn Error>` | `Exception` | `std::exception` |
| Heap allocation | `Box<T>` | All classes are heap-allocated | `std::unique_ptr<T>` |
| Dynamic dispatch | `dyn Trait` | Interface references / virtual methods | `virtual` methods |

> **Read more:** [Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) | [Using Box\<T\> to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)

---

## 15. Sources

### Official Rust Documentation (doc.rust-lang.org)
- [What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The String Type](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Appendix C: Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [The Option Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)
- [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Concise Control Flow with if let](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Using Box\<T\>](https://doc.rust-lang.org/book/ch15-01-box.html)

### Cross-Language Comparisons
- [Interfaces — Rust for C#/.NET Developers (Microsoft)](https://microsoft.github.io/rust-for-dotnet-devs/latest/language/custom-types/interfaces.html)
- [Rust Traits for C# Developers](https://www.matthewathomas.com/programming/2022/02/08/rust-traits-for-csharp-devs.html)
- [Rust Trait vs. C++ Abstract Class](https://www.rangakrish.com/index.php/2022/04/03/rust-trait-vs-c-abstract-class/)
- [Abstract Classes, Interfaces, and Dynamic Dispatch — C++ to Rust Phrasebook (Brown University)](https://cel.cs.brown.edu/crp/idioms/data_modeling/abstract_classes.html)
- [Abstraction Without Overhead: Traits in Rust (Official Rust Blog)](https://blog.rust-lang.org/2015/05/11/traits.html)
