# HTTP for Lunch

This is an HTTP server which was built during the [September Rust for Lunch meetup](https://lunch.rs/meetups/2024-09-17/).

The code has been left exactly as it was at the end of the meet-up, `dbg!` statements and all!

## Building & running

Normal `cargo` behaviour:

```sh
cargo run
```

## Testing

We ran the following requests against the server.

Happy path:

```sh
$ curl -D - http://127.0.0.1:8080/hello
HTTP/1.1 200 Rust for Lunch
Content-Length: 13

Hello, World!
```

Method not allowed:

```sh
$ curl -X POST -D - http://127.0.0.1:8080/hello
HTTP/1.1 405 Method Not Allowed
```

HTTP version not supported:

```sh
$ curl --http1.0 -D - http://127.0.0.1:8080/hello
HTTP/1.1 505 HTTP Version not supported
```

Not found:

```sh
$ curl -D - http://127.0.0.1:8080/bye
HTTP/1.1 404 Not Found
```
