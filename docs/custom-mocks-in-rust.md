# Custom mocks in Rust

- Org: [Nautilus Cyberneering](https://github.com/Nautilus-Cyberneering).
- Author: [Jose Celano](https://github.com/josecelano).

## Introduction

I'm working in a [BitTorrent tracker](https://github.com/torrust/torrust-tracker) in Rust. It's my first Rust project. I started working on the project and learning Rust on the 6th of July, 2022.

I wanted to know the code base before starting to add features. So I started adding automated tests to get to know the code before making changes.

I've been working mainly with dynamic type languages, so when it comes to creating test doubles, I have never had any problem. Rust is not only a typed language; it also implements a "borrowing" mechanism which enforces these rules:

- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References must always be valid.

I was working on the tracker when I wanted to add a new test to check that a method for a class collaborator was called. It is a very common test. We have a Tracker, the primary domain class, that uses an `EventSender` to send events. Every time something relevant happens, the tracker sends an event so that other parts of the software can react. In this case, we have a `TrackerStatistics` class that listens to all the events and increases counters for each request type.

## Production code

The `Tracker` has a dependency which is dynamically sized `dyn EventSender`. That means we do not know the size of the type at compile time.

```rust
pub struct Tracker {
    event_sender: Rc<dyn EventSender>,
}
```

We added the trait `EventSender` because we wanted two implementations—the one we use in production code and the mock we want to use in the test. In our real case, we wanted to mock the dependency because it is an "async" function executed in a different thread. In other words, it's an out-of-process dependency. Even if it's a managed out-of-process dependency, testing with threads was very hard to do without changing the production code. I did not want to do that because I'm not particularly eager to force a change in production code just to test it if that change does not fit well. See the ["Test Induced design damage"](https://dhh.dk/2014/test-induced-design-damage.html) article.

This example is more straightforward because we are not using concurrency, and we could use the actual sender instead of the mock. But a similar solution could be applied if you are using threads or have a dependency you want to mock.

Before explaining the tests and the problem, we need to explain another thing that could be unfamiliar to developers used to dynamic languages like me. As you can see, the `Tracker` struct contains an `event_sender` which is an `Rc<dyn EventSender>` type.

We need the `dyn EventSender` type because we do not know the type at compile time.

The `Rc` (Reference Counted) is a kind of wrapper of our type that allows us to share ownership. That means we can have two copies of that "pointer" pointing at the same value.
We need that `Rc` only for the test. In the production code `EventSender` could be owned by the `Tracker` but in the test we need to keep a copy of the mock for the assertions because we need to access its state after the `Tracker` has been called.

That's all regarding the production code.

## Test code

The test we wanted to write was elementary. We only wanted to check that a given event was sent when a certain request was made. For example, if a BitTorrent client makes a connection request, we want to test that the `Tracker` sends the `Event::Connect` using the `EventSender`.

I did not want to use a mocking framework because the test was supposed to be simple and I do not like adding dependencies if there is a simpler way to do the same. Dependencies add an extra maintenance cost and a security risk (because sometimes you add more code than you need).

So, I created my custom mock for the `EventSender`. The idea was simple: I can store the event that is passed to the `send_event` function and check if it matches the one I expected.

```rust
#[derive(Clone, Copy)]
struct TrackerEventSenderMock {
    pub sent_event: Option<Event>,
}

impl TrackerEventSenderMock {
    pub fn new() -> Self {
        Self { sent_event: None }
    }
}

impl EventSender for TrackerEventSenderMock {
    fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
        self.sent_event = Some(event);
        Ok(())
    }
}

#[test]
fn the_tracker_should_send_a_connect_event_after_connecting() {
    // Test using a custom mock for the TrackerEventSender

    let event_sender = Rc::new(TrackerEventSenderMock::new());
    let tracker = Arc::new(Tracker::new(event_sender.clone()));

    tracker.connect();

    assert_eq!(event_sender.sent_event.unwrap(), Event::Connect);
}
```

That code did not work because of this line:

```rust
self.sent_event = Some(event);
```

Since the `self` reference is not mutable, you can not change the `sent_event` value.

I needed to learn how to implement it, and [Cameron](https://github.com/da2ce7) pointed me to the solution. The not mutable `self` reference does not allow you to change the attributes in the struct, but it's not recursive. Rust has some types that allow you to change the interior mutability.

Rust has a pattern called the ["Interior Mutability Pattern"](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#refcellt-and-the-interior-mutability-pattern).

> Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; typically, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.

You can "bend" Rust rules with a `RefCell` type.

> RefCell is a mutable memory location with dynamically checked borrow rules.

That means you can mutate the value inside the `RefCell` even when `RefCell` is immutable.

From the Rust book, you can see the different types to "bend" Rust rules:

- `Rc<T>` enables multiple owners of the same data.
- `Box<T>` and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time.
- `Rc<T>` allows only immutable borrows checked at compile time.
- `RefCell<T>` allows immutable or mutable borrows checked at runtime.

The final test was like this:

```rust
    #[derive(Clone)]
    struct TrackerEventSenderMock {
        pub sent_event: RefCell<Option<Event>>,
    }

    impl TrackerEventSenderMock {
        pub fn new() -> Self {
            Self {
                sent_event: RefCell::new(None),
            }
        }
    }

    impl EventSender for TrackerEventSenderMock {
        fn send_event(&self, event: Event) -> Result<(), Box<dyn Error>> {
            *self.sent_event.borrow_mut() = Some(event);

            // We return the expected value
            Ok(())
        }
    }

    #[test]
    fn the_tracker_should_send_a_connect_event_after_connecting() {
        // Test using a custom mock for the TrackerEventSender

        let event_sender = Rc::new(TrackerEventSenderMock::new());
        let tracker = Arc::new(Tracker::new(event_sender.clone()));

        tracker.connect().unwrap();

        assert_eq!(event_sender.sent_event.borrow().unwrap(), Event::Connect);
    }
```

## Conclusion

Learning how to handle pointers, references or whatever is called in other languages takes a lot of work. I remember being surprised by other languages that mutate things I did not expect to be mutable. At least with Rust, you are not surprised because the only way to do it is by knowing what you are doing and doing it explicitly.

I finally decided to use `mockall` (the mocking framework) for some reasons:

1. We want to add more tests to that project, and sooner or later, we will need more complex mocks.
2. The `mockall` readability is better because of its fluent style.
3. `mockall` allows you to be more precise, for example checking also the number of calls.

YOu can read the final solution using [mockall](https://docs.rs/mockall/latest/mockall/) [here](https://github.com/torrust/torrust-tracker/blob/develop/src/udp/handlers.rs#L426-L463):

<https://github.com/torrust/torrust-tracker/blob/develop/src/udp/handlers.rs#L426-L463>

```rust
#[tokio::test]
async fn it_should_send_the_upd4_connect_event_when_a_client_tries_to_connect_using_a_ip4_socket_address() {
    let mut stats_event_sender_mock = MockTrackerStatisticsEventSender::new();
    stats_event_sender_mock
        .expect_send_event()
        .with(eq(TrackerStatisticsEvent::Udp4Connect))
        .times(1)
        .returning(|_| Box::pin(future::ready(Some(Ok(())))));
    let stats_event_sender = Box::new(stats_event_sender_mock);

    let client_socket_address = sample_ipv4_socket_address();

    let torrent_tracker = Arc::new(
        TorrentTracker::new(default_tracker_config(), Some(stats_event_sender), StatsRepository::new()).unwrap(),
    );
    handle_connect(client_socket_address, &sample_connect_request(), torrent_tracker)
        .await
        .unwrap();
}
```

## Links

- [Type Systems in Programming Languages: Static vs. Dynamic and Strong vs. Weak](https://betterprogramming.pub/type-systems-in-programming-languages-static-vs-dynamic-and-strong-vs-weak-ed1bb542b06) by Yong Cui.

## Credits

- [Cameron Garnham](https://github.com/da2ce7) told me how to solve it.
