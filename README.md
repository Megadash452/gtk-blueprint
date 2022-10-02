# GTK Blueprint for Rust

This crate lets you use the [Blueprint](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/) syntax for the UI in GTK applications built with Rust.

For this to work the Blueprint Compiler directory must be found in the **$PATH** environtment variable as *"blueprint-compiler"*, or it can be included in the project root in `blueprint-compiler/blueprint-compiler.py`.

If the Blueprint Compiler is not found, your project will not compile. If the Blueprint compiler fails to compile the file to UI XML for whatever reason (file not found, bad syntax, etc.) your program will also fail to compile.

----

## Include Blueprint

Use the `include_blp!()` macro to compile the Blueprint to the format that GtkBuilder can read (UI XML). The macro takes a file path relative to the *project root*. Similarly to how the `include_str!()` macro works, the compiled UI XML is embedded in the Rust source file as a `&'static str`.

### Example

```rust
let builder = gtk::Builder::from_string(include_blp!("./src/window.blp"));
// OR without ./ prefixed to the path
let builder = gtk::Builder::from_string(include_blp!("src/window.blp"));
```

## Blueprint Map

This macro generates a static [Map](https://docs.rs/phf/0.11.1/phf/struct.Map.html) of all the blueprints (files ending in `.blp`) in your project. It compiles the files and uses them as the *Values* for the map, with the *Keys* being the path of that blueprint relative to the *Project Root*. Then use [get_blp](https://docs.rs/gtk-blueprint/latest/gtk_blueprint/macro.get_blp.html) to use the `&str` of the compiled blueprint.

The input for this macro is the path (relative to the *Project Root*) that it should start looking for blueprint files.

The advantage of this is when there is a blueprint that is used multiple times in your code, it will only be embedded once in the binary, where [include_blp](https://docs.rs/gtk-blueprint/latest/gtk_blueprint/macro.get_blp.html) would embed it as many times as it is used.

### Example

```rust
use gtk_blueprint::get_blp;

gtk_blueprint::gen_blp_map!("");

fn main() {
    let builder = gtk::Builder::from_string(get_blp!("./src/window.blp"));
    // OR without ./ prefixed to the path
    let builder = gtk::Builder::from_string(get_blp!("src/window.blp"));
}
```

## Bug

If you make changes to any of your Blueprint files but don't make changes to the Rust code, cargo will think that it does not need to recompile the project because the code hasn't changed and it will run with the old version of the blueprint. As a work-around, create a file `build.rs` at the root of your project and put this in it:
```rust
fn main() {
    println!("cargo:rerun-if-changed=**/*.blp");
}
```