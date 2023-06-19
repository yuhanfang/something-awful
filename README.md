# Something Awful

This is an early-stage unofficial client to
[Something Awful](https://forums.somethingawful.com).

Some of the things you can do:

-   Read public user profiles
-   Fetch individual posts from threads
-   Get the status of bookmarked threads

The client assumes that you have a registered account.
[Register here](https://store.somethingawful.com/products/register.php).

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/something-awful.svg
[crates-url]: https://crates.io/crates/something-awful
[docs-badge]: https://img.shields.io/docsrs/something-awful
[docs-url]: https://docs.rs/something-awful
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/yuhanfang/something-awful/blob/master/LICENSE

See
[tail-something-awful.rs](https://github.com/yuhanfang/something-awful/blob/main/src/bin/tail-something-awful.rs)
for an end-to-end example that uses the client to tail updates to bookmarked
threads.

Put Something Awful in a tmux pane and never get anything done at work ever
again! Example output:

```sh
$ ./tail-something-awful
Logged in.

----------
 /\_/\
( o.o )
 > ^ <

thread: here is an example title
url: https://forums.somethingawful.com/showthread.php?threadid=12345&goto=newpost
author: Somebody Cool
time: Jun 18, 2023 22:08
----------
Read a markdown-formatted synopsis from the comfort of your terminal!

The program caches your credentials by default in .something-awful.token and
also caches seen post history in .something-awful.history to avoid showing
duplicate messages.
```
