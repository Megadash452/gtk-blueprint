# GTK Blueprint for Rust

This crate lets you use the [Blueprint](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/) syntax for the UI in GTK applications built with Rust.

For this to work the Blueprint Compiler directory must be found in the $PATH environtment variable or it can be included in the project root in `blueprint-compiler/blueprint-compiler.py`. If the Blueprint Compiler is not found your project will not compile.

----

Use the `include_blp!()` macro to compile the Blueprint to the format that GtkBuilder can read (UI XML). The macro takes a file path relative to the *project root*. Similarly to how the `include_str!()` macro works, the compiled UI XML is included in the Rust source file as a `&'static str`.

If the Blueprint compiler fails to compile the file to UI XML for whatever reason (file not found, bad syntax, etc.) your program will also fail to compile.

## Example

```rust
let builder = gtk::Builder::new();
builder.add_from_string(include_blp!("./src/window.blp")).expect("error parsing UI XML");
```
