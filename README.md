# hn_api

[![CircleCI][circle-ci-badge]][circle-ci]
[![Rust][rust-badge]][github]

A simple synchronous and asynchronous [Hacker News API (v0)](https://github.com/HackerNews/API)
client library based on reqwest and serde.

The library currently implements no caching. It simply exposes endpoints as
methods.

Furthermore, there is no realtime functionality. If you need that, you
should probably use a firebase client crate and subscribe to the live
endpoints directly.

For an example, see `examples/top.rs` and `examples/async_top.rs`.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

**Contributing**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


<!-- Badges -->
[circle-ci]: https://circleci.com/gh/dbrgn/hn_async_api/tree/master
[circle-ci-badge]: https://circleci.com/gh/dbrgn/hn_async_api/tree/master.svg?style=shield
[github]: https://github.com/dbrgn/hn_async_api
[rust-badge]: https://img.shields.io/badge/rust-2018%2B-blue.svg?maxAge=3600
