# zzhack
WIP

## How to start dev server
SSR is a experimental feature of Yew. you need to build both client bundle and server source code when the source code was changed.

We recommaned you install `cargo-watch` to trigger command execution. 
```shell
Cargo install cargo-watch
```

Go to the root of project. Build client bundle and watch:
```shell
cargo watch -C entry -i dist -i public -i styles -- trunk build
```

Then build server bin and watch:
```shell
cargo watch -C entry -- cargo run --features=ssr --bin zzhack_main -- --dir dist
```


### TailwindCSS
zzhack depends on `TailwindCSS` for CSS compilation, run the following command to get start:
```shell
npx tailwindcss -i ./entry/styles/input.css -o ./entry/styles/output.css -- --watch
```

