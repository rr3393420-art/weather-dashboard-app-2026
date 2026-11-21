# To-do List

A simple Rust command-line to-do app. Tasks persist as JSON in `todos.json` in the current directory, so they are still there when you restart the app.

## Usage

```sh
cargo run -- add "Buy groceries"
cargo run -- add "Call the dentist"
cargo run -- list
cargo run -- done 1
cargo run -- delete 2
```

Running `cargo run` without a command lists tasks. Use `cargo run -- help` to see the available commands.

Set `TODO_FILE` to store the JSON data at another path, for example:

```sh
TODO_FILE="$HOME/.todos.json" cargo run -- list
```

## Tests

```sh
cargo test
```
