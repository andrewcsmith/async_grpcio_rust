# RPC Problems

Nutshell of this comes down to that I can't seem to figure out exactly how to
get the server to not block the main thread, and to respond to everything in
parallel. Do this:

Terminal #1

```
cargo run --bin server
```

Terminal #2

```
cargo run --bin client
```

And you should see that the server is processing each item sequentially,
blocking the entire thread on the line `thread::sleep_ms(1000)` on every
iteration. Resource Monitor (on Windows) is definitely spinning up the
correct number of threads. Help?