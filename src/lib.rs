mod util;

use util::*;
use proc_macro::{self, TokenStream};
use syn::{LitStr, parse_macro_input};

const BLP_MAP_NAME: &str = "__COMPILED_BLUEPRINT_MAP__";


// TODO: #![doc(include="../README.md")] when it comes to stable


#[proc_macro]
/// Embeds a GtkUI compiled from a Blueprint file as a string to your Rust code.
/// 
/// This is meant to be used if the blueprint will only be used *once*. Otherwise
/// please see [gen_blp_map!].
/// 
/// ### Input
///
/// The file is located relative to the *Project Root* directory. The provided path
/// is interpreted in a platform-specific way at compile time. So, for instance, an
/// invocation with a Windows path containing backslashes \ would not compile
/// correctly on Unix.
/// 
/// ### Output
/// 
/// This macro will yield an expression of type `&'static str` which is the compiled
/// UI XML of the Blueprint.
/// 
/// ## Error
/// 
/// If the Blueprint compiler fails to compile the file to UI XML for whatever
/// reason (file not found, bad syntax, etc.) your program will also fail to compile.
pub fn include_blp(input: TokenStream) -> TokenStream {
    // TODO: trigger a rebuild if .blp file has changed
    let path = parse_macro_input!(input as LitStr).value();
    
    match compile_blp(&path) {
        Ok(xml) => format!("r###\"{xml}\"###").parse().unwrap(),
        Err(error) => format!("compile_error!(\"{error}\")").parse().unwrap()
    }
}


#[proc_macro]
/// Generates a [Map](https://docs.rs/phf/0.11.1/phf/struct.Map.html) of all the
/// blueprints in the project and their paths. ***Requires crate
/// [phf](https://crates.io/crates/phf) as a dependency in your project.***
/// 
/// This is meant to be used if the blueprint will be used *multiple times*.
/// Otherwise you can use [include_blp!].
/// 
/// The **Keys** are the paths to each of the blueprints that were compiled into the
/// Map. The **Values** are the compiled blueprints.
/// 
/// ## Use
/// 
/// Use the macro in the root of your `main.rs`:
/// 
/// ```rs
/// gtk_blueprint::gen_blp_map!("");
/// 
/// fn main() {
///     
/// }
/// ```
/// 
/// ### Input
/// 
/// The macro takes a **starting directory** relative to the *Project Root* as input.
/// The macro looks for `.blp` files recursively starting from that directory.
/// An empty string as input means to use the *Project Root* as the starting
/// directory.
/// 
/// To use the compiled blueprint in your code, use the [get_blp!] macro, giving it
/// the *blueprint's path* as input.
/// 
/// ### Output
/// 
/// The generated Map is a static global variable named `__COMPILED_BLUEPRINT_MAP__`.
/// The underscores are there to prevent the name from conflicting with some other
/// variable or crate. However, there is no need to use the static variable. It is
/// only there so it can be used by the [get_blp!] macro.
/// 
/// -------
/// 
/// On a project structured like this:
/// ```txt
///  │
///  ├ root.blp
///  └ src/
///    ├ main.rs
///    ├ src.blp
///    └ util/
///      ├ mod.rs
///      └ mod.blp
/// ```
/// The map would look like this:
/// ```rs
/// Map {
///   "root.blp" => "...",
///   "src/src.blp" => "...",
///   "src/util/mod.blp" => "..."
/// }
/// ```
/// Then use the compiled blueprint with `gtk::Builder`:
/// ```rs
/// let builder = gtk::Builder::from_string(get_blp!("src/src.blp"));
/// ```
/// ------
/// 
/// 
/// ## Error
/// 
/// If the Blueprint Compiler fails to compile any of the .blp files, your program
/// will fail to compile.
pub fn gen_blp_map(input: TokenStream) -> TokenStream {
    let mut map = phf_codegen::Map::<String>::new();
    let mut errors = false;

    let start_path = {
        let path = parse_macro_input!(input as LitStr).value();

        // search_blps() can't take an empty path
        if path.is_empty() {
            ".".to_string()
        } else {
            path
        }
    };
    
    for mut path in search_blps(&start_path) {
        // Remove the "./" if it exists in the path
        if let Some(new_path) = path.strip_prefix("./") {
            path = String::from(new_path);
        }

        match compile_blp(&path) {
            Ok(xml) => map.entry(path, &format!("r###\"{xml}\"###")),
            Err(error) => {
                errors = true;
                eprintln!("{error}");
                continue
            }
        };
    }

    if errors {
        format!("compile_error!(\"One or more Blueprints had errors and could not be compiled. Check output of `cargo build` for more details`\")").parse().unwrap()
    } else {
        format!("pub static {BLP_MAP_NAME}: phf::Map<&'static str, &'static str> = {};\n", map.build()).parse().unwrap()
    }
}

#[proc_macro]
/// Get the compiled blueprints generated by [gen_blp_map!]
/// 
/// The macro takes the path of the blueprint relative to the *Project Root* as
/// input. Macro results in something like `__COMPILED_BLUEPRINT_MAP__.get("path").unwrap()`,
/// and that returns the `&'static str` that is the compiled blueprint.
pub fn get_blp(input: TokenStream) -> TokenStream {
    let path = {
        let path = parse_macro_input!(input as LitStr).value();

        /* Remove the "./" if it exists in the path.
          The keys in the map will have this format: src/blp */
        if let Some(path) = path.strip_prefix("./") {
            String::from(path)
        } else {
            path
        }
    };

    if std::path::Path::new(&path).exists() {
        format!("{BLP_MAP_NAME}.get(\"{path}\").expect(\"Blueprint did not compile correctly\")").parse().unwrap()
    } else {
        format!("compile_error!(\"Error getting blueprint \\\"{path}\\\": File Not Found.\")").parse().unwrap()
    }
}