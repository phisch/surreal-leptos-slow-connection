# surreal-leptos-slow-connection

This project starts a leptos application which connects via websocket to a surrealdb running on `127.0.0.1:8000`. It measures the time this takes, and logs it to `console.log`.

I expect this to take a few milliseconds, but it currently takes roughly 5 seconds.

## Running this example

Add the rust wasm target:

```sh
rustup target add wasm32-unknown-unknown
```

Install trunk (needed to build and serve the project):
```sh
cargo install trunk
```

And finally, run the project:

```sh
trunk serve --open
```