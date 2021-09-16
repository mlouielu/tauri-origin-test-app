Tauri Origin Test
=================

The bug from webkit let tauri cannot communicate with CORS:
https://bugs.webkit.org/show_bug.cgi?id=229034

This is a PoC for Origin test, by using `yarn tauri:serve`,
the counter will increase by second, but `yarn tauri:build`'s AppImage won't,
and will receive CORS error.

# How-To-Run

## Server

```
$ cd server
$ cargo run
```


## Tauri

```
$ cd tauri-origin-test-app
$ yarn tauri:serve
$ yarn tauri:build
```
