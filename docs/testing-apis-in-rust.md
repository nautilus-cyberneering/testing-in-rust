# Testing APIs in Rust

- Org: [Nautilus Cyberneering](https://github.com/Nautilus-Cyberneering).
- Author: [Jose Celano](https://github.com/josecelano).

## Introduction

We (Torrust Team) are working on a [BitTorrent tracker](https://github.com/torrust/torrust-tracker) in Rust. The tracker has a REST API. During a refactor, we unintentionally changed one of the JSON resources in an endpoint. We changed one of the domain structs, and that change was propagated to the exposed endpoint resource. That happened because we automatically convert domain structs into JSON.

To avoid this kind of regression error, we added a test for the API endpoint. We decided to do it as simply as possible. Our plan was:

- Create a new integration test.
- Execute the web server which runs the REST API.
- Use an HTTP client to make a request to the API.

## Integration test setup

The setup was easy and fast. The only decision we had to make was whether we should use the low-level "hyper" dependency for the HTTP client or the "reqwest" dependency. In the end, we chose "reqwest" because we were using JSON resources, and it seemed easier to use (or at least less verbose than the low-level package).

We copied a lot of code from this article:

[End-to-end testing for Rust web services](https://blog.logrocket.com/end-to-end-testing-for-rust-web-services/) by [Mario Zupan](https://github.com/zupzup). Especially the solution to run an out-of-process dependency before running the test.

We needed to run the web server before executing the test.

Mario's solution is slightly different because he uses a single server instance for all the tests. We wanted to have an independent instance for each test because sometimes we need to change the web server's configuration for the test.

Mario's solution uses an `AtomicBool` flag to ensure it does not run more than one instance because all the tests share the same instance.

We initially created a struct to run the server like this:

```Rust
pub struct ApiServerStarter {
    pub started: AtomicBool,
    pub job: Option<JoinHandle<()>>,
}

impl Default for ApiServerStarter {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiServerStarter {
    pub fn new() -> Self {
        Self {
            started: AtomicBool::new(false),
            job: None,
        }
    }

    pub async fn start(&mut self, addr: SocketAddr) {
        if !self.started.load(Ordering::Relaxed) {
            self.job = Some(tokio::spawn(async move {
                start_server(addr).await;
            }));

            self.started.store(true, Ordering::Relaxed);

            // Wait to give time to the API server to be ready to accept requests
            sleep(Duration::from_millis(100)).await;
        }
    }
}
```

That was the first version, and we changed but before explaining the latest version, we want to explain why we needed to change it.

## ConnectionRefused error

After setting everything up, we ran the test and got this error message: "Connection refused".

```s
test it_should_greeting_you ... FAILED

failures:

---- it_should_greeting_you stdout ----
Server running in: http://127.0.0.1:3030/hello/warp
thread 'it_should_greeting_you' panicked at 'called `Result::unwrap()` on an `Err` value: reqwest::Error { kind: Request, url: Url { scheme: "http", cannot_be_a_base: false, username: "", password: None, host: Some(Ipv4(127.0.0.1)), port: Some(3030), path: "/hello/warp", query: None, fragment: None }, source: hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }', tests/testing-apis-in-rust.rs:17:43
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

We had two possible reasons for that error:

1. The way we were starting the web server in the test was wrong. We could have missed something.
2. For some reason, the web server was not ready.

We tried different solutions, for example, using a `static` server like in Mario's solution, but we got the same error. Finally, I realized Mario was adding a "sleep" after starting the server. Adding a delay after starting the web server gives the server time to be ready to receive requests.

```Rust
pub async fn start(&mut self, addr: SocketAddr) {
    if !self.started.load(Ordering::Relaxed) {
        self.job = Some(tokio::spawn(async move {
            start_server(addr).await;
        }));

        // ...

        // Wait to give time to the API server to be ready to accept requests
        sleep(Duration::from_millis(100)).await;
    }
}
```

That worked was we were not happy with adding a random sleep time. Because that:

- Make tests slower.
- Could make the test fail if, for some reason, the web server takes more than 100 milliseconds to be ready.

We were thinking about alternative solutions like this:

```Rust
pub async fn start(&mut self, addr: SocketAddr) {
    if !self.started.load(Ordering::Relaxed) {
        let job = tokio::spawn(async move {
            start_server(addr).await;
        });

        // ...

        job.await.unwrap();
    }
}
```

In this case, we tried to wait until the job was done, but It did not work because the web server is an infinite loop, and the job does not finish.

By reading the Tokio library documentation, we realized we could send messages from the job to the main process using a [channel](https://docs.rs/tokio/latest/tokio/sync/index.html#mpsc-channel). We changed the test implementation to this:

```Rust
#[tokio::test]
async fn it_should_greeting_you() {
    let bind_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030);

    start_server_and_wait(bind_address).await;

    let url = format!("http://{}/hello/{}", &bind_address, "warp");

    let content = reqwest::get(url).await.unwrap().text().await.unwrap();

    assert_eq!(content, "Hello, warp!");
}

async fn start_server_and_wait(addr: SocketAddr) {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let started = true;
        tx.send(started).await.unwrap();
        start_server(addr).await;
    });

    while let Some(res) = rx.recv().await {
        if res {
            break;
        }
    }
}
```

Now, we send a message from the child thread to the parent, and the parent waits until it receives the message. It works, or at least we could remove the nondeterministic "sleep". We send the message when the new thread is being executed but before the web server is started.

We suppose that means the problem was not the time needed by the web server to be ready, which should be short. The problem was the child process had yet to be created when the test was executed. We also suppose the "sleep" sentence works because it switches the execution to the child process.

If the server took longer to start maybe, we would need to add a "sleep" anyway.

This is a better solution because we do not wait a random time. And we can keep the production code the same.

## Conclusion

Testing code which uses concurrency and infinite loops can be very tricky. We have had this type of problem before. This solution could be a good pattern since we did not change the production code.

Do you have a better solution? Please open an issue or PR in this repo and let us know.

## Links

- [End-to-end testing for Rust web services](https://blog.logrocket.com/end-to-end-testing-for-rust-web-services/) by [Mario Zupan](https://github.com/zupzup).
