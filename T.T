   Compiling server v0.1.0 (/home/gonik/Documents/pruebas/server)
warning: unused import: `http::request`
 --> src/core/request/mod.rs:8:5
  |
8 | use http::request;
  |     ^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `StatusCode` and `request`
 --> src/core/router/mod.rs:1:12
  |
1 | use http::{request, StatusCode};
  |            ^^^^^^^  ^^^^^^^^^^

warning: unused import: `crate::Response`
 --> src/core/router/mod.rs:4:5
  |
4 | use crate::Response;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `std::fmt::Debug`
 --> src/core/router/mod.rs:6:5
  |
6 | use std::fmt::Debug;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `Request`
  --> src/core/router/mod.rs:22:26
   |
22 |     use super::{Handler, Request};
   |                          ^^^^^^^

warning: unused imports: `Read` and `Write`
 --> src/core/server/mod.rs:6:33
  |
6 | use std::io::{BufReader, Error, Read, Write};
  |                                 ^^^^  ^^^^^

warning: unused variable: `handler`
  --> src/core/router/mod.rs:23:16
   |
23 |     pub fn get(handler: impl Handler) -> Response {
   |                ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_handler`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> src/core/server/mod.rs:68:17
   |
68 |             let mut st = stream.map_err(|err| ServerError::new(err))?;
   |                 ----^^
   |                 |
   |                 help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: unused variable: `request`
  --> src/lib.rs:12:21
   |
12 | pub fn base_handler(request: Request) -> String {
   |                     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_request`

warning: enum `RequestHeaderType` is never used
  --> src/core/request/mod.rs:10:6
   |
10 | enum RequestHeaderType {
   |      ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: fields `version` and `headers` are never read
  --> src/core/request/mod.rs:21:5
   |
18 | pub struct Request {
   |            ------- fields in this struct
...
21 |     version: String,
   |     ^^^^^^^
22 |     pub(crate) headers: Headers,
   |                ^^^^^^^
   |
   = note: `Request` has derived impls for the traits `Clone` and `Debug`, but these are intentionally ignored during dead code analysis

warning: struct `ConnectionError` is never constructed
  --> src/core/request/mod.rs:25:12
   |
25 | pub struct ConnectionError {
   |            ^^^^^^^^^^^^^^^
   |
   = note: `ConnectionError` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: enum `ResponseHeaderType` is never used
  --> src/core/response/mod.rs:51:6
   |
51 | enum ResponseHeaderType {
   |      ^^^^^^^^^^^^^^^^^^

warning: field `state` is never read
  --> src/core/server/mod.rs:21:5
   |
20 | pub struct Server {
   |            ------ field in this struct
21 |     state: ServerState,
   |     ^^^^^

warning: field `inner` is never read
  --> src/core/server/mod.rs:26:5
   |
25 | pub struct ServerError {
   |            ----------- field in this struct
26 |     inner: String,
   |     ^^^^^
   |
   = note: `ServerError` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: `server` (lib) generated 15 warnings (run `cargo fix --lib -p server` to apply 7 suggestions)
warning: unused imports: `Handler`, `Response`, `base_handler`, `handlers::get`, and `other_handler`
 --> src/bin/server.rs:4:5
  |
4 |     base_handler, handlers::get, other_handler, Handler, Request, Response, ResponseBody, Router,
  |     ^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^           ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `req`
  --> src/bin/server.rs:16:23
   |
16 | fn some_kinda_handler(req: &Request) -> OuterHashmap {
   |                       ^^^ help: if this is intentional, prefix it with an underscore: `_req`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `server` (bin "server") generated 2 warnings (run `cargo fix --bin "server"` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 0.89s
     Running `target/release/server`
