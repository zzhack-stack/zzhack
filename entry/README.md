## How to run dev server
Well, SSR is a experimental feature of Yew. You change the source code everytime you need to build both client bundle and server source code.

We recommaned you install cargo-watch, and watch the whole workspace, if source code was changed, then rebuild the app:

### Build client bundle
```shell
cargo watch -C entry -i dist -i public -- trunk build
```

### Run server

```shell
cargo watch -C entry -- cargo run --features=ssr --bin zzhack_main -- --dir dist
```

## TailwindCSS
```shell
npx tailwindcss -i ./entry/styles/input.css -o ./entry/styles/output.css -- --watch
```
