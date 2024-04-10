# Static vs Dynamic mocks in Rust

- Org: [Nautilus Cyberneering](https://github.com/Nautilus-Cyberneering).
- Author: [Jose Celano](https://github.com/josecelano).

Mocking is a crucial aspect of testing software applications. It allows developers to isolate units of code for testing by substituting real dependencies with mock objects. In Rust, where performance and safety are paramount, the choice between dynamic dispatch and generics for mocking can significantly impact both code design and runtime behavior. In this article, we'll delve into the nuances of mocking using dynamic dispatch and generics in Rust, exploring their implications for performance and code structure.

## Introduction

Mocking is a technique used in software testing to replace real dependencies with simulated objects, allowing developers to test code in isolation. In Rust, a statically-typed and compiled language, mocking can be achieved through two main mechanisms: dynamic dispatch and generics. Each approach has its advantages and trade-offs, which we'll discuss in detail.

This article is related to [Custom mocks in Rust](./custom-mocks-in-rust.md). In that previous article we build a custom mock using dynamic dispatch. In general, we think that could be the first approach for someone coming from dynamic languages. I guess we tend to map traits in Rust to interfaces in other languages. Sometimes that direct mapping can affect performance.

## Dynamic Dispatch: An Overview

Dynamic dispatch in Rust involves using trait objects to achieve runtime polymorphism. It allows for flexibility in swapping out implementations at runtime, making it suitable for scenarios where the concrete implementation is determined dynamically.

Let's consider an example where we have an application with a `UserRepository` trait and multiple implementations. We have chosen this example because it's pretty common domain and pattern. Very often repositories implies accessing to databases and that is too slow for unit tests. So they are usually replaced by in-memory implementation to run unit tests faster. In this example we have two implementations in memory just to keep things simple for this example.

You can find the whole code in [example03](./../src/example03/).

Here's how dynamic dispatch might be utilized:

```rust
pub trait UserRepository {
    fn add_user(&mut self, user: User);
    fn get_user(&self, name: &str) -> Option<&User>;
    fn get_all_users(&self) -> Vec<&User>;
}

pub struct App {
    user_repository: Box<dyn UserRepository>,
}

impl App {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn run(&mut self) -> String {
        // Code that interacts with UserRepository
    }
}
```

In this setup, `App` uses dynamic dispatch by accepting a `Box<dyn UserRepository>` as a parameter. While this approach provides flexibility, it incurs runtime overhead due to virtual function calls and heap allocations.

For production we are using the `BTreeMapRepository`:

```rust
let mut app = App::new(Box::new(user_repository::BTreeMapRepository::default()))
```

And for testing we use a different implementation:

```rust
let mut app = App::new(Box::new(VecRepository::default()));
```

## Performance Implications

The use of dynamic dispatch can introduce runtime overhead, impacting performance-sensitive applications. Each virtual function call involves a lookup in the vtable, incurring a small but non-negligible cost. Additionally, heap allocations for trait objects can lead to increased memory usage and potential cache misses.

In performance-critical scenarios, such as high-throughput systems or embedded environments, these overheads can be significant and may necessitate alternative approaches.

You can check how the performance decreases by adding some extra users to the repository. With 10_000_000 extra users the static version takes 20.385s and the dynamic version 20.455s.

```console
time cargo run --bin dynamic-mocks-in-rust
real 0m29.261s
user 0m20.455s
sys 0m4.758s
```

```console
time cargo run --bin static-mocks-in-rust
real 0m29.002s
user 0m20.385s
sys  0m4.935s
```

## Generics: The Solution for Performance

Generics offer a compile-time mechanism for achieving polymorphism without sacrificing performance. By specifying generic type parameters, Rust generates specialized code for each concrete type, eliminating runtime dispatch overhead.

Let's refactor our `App` struct to use generics instead of dynamic dispatch:

```rust
pub struct App<T: UserRepository> {
    user_repository: T,
}

impl<T: UserRepository> App<T> {
    pub fn new(user_repository: T) -> Self {
        Self { user_repository }
    }

    pub fn run(&mut self) -> String {
        // Code that interacts with UserRepository
    }
}
```

You can find the whole refactored code in [example04](./../src/example04/).

In this version, `App` is generic over any type that implements `UserRepository`. This allows for direct function calls and avoids heap allocations and vtable lookups associated with dynamic dispatch. The resulting code is more efficient and suitable for performance-sensitive applications.

## When to Use Each Approach

The choice between dynamic dispatch and generics depends on the specific requirements of your application:

- __Dynamic Dispatch:__ Use dynamic dispatch when flexibility in choosing implementations at runtime is paramount. This approach is suitable for scenarios where the concrete implementation may vary dynamically, such as plugin systems or configuration-driven applications.

- __Generics:__ Prefer generics when performance is a primary concern and the set of possible implementations is known at compile time. Generics enable compile-time specialization, leading to more efficient code execution. This approach is well-suited for performance-critical applications where every CPU cycle counts.

## Conclusion

Mocking is an essential tool in the arsenal of software developers, enabling effective testing and ensuring the reliability of code. In Rust, the choice between dynamic dispatch and generics for mocking can have significant implications for both code structure and runtime performance.

By understanding the trade-offs between these approaches, developers can make informed decisions when designing and testing their applications. Whether prioritizing flexibility or performance, Rust provides the tools necessary to build robust and efficient software systems.

We should avoid making the software slower to make it testable.

## Credits

[ChatGPT](<https://chat.openai.com/>).
