mod util;

use util::*;
use proc_macro::{self, TokenStream};
use syn::LitStr;


#[proc_macro]
/// Includes a GtkUI compiled from a Blueprint file as a string.
///
/// The file is located relative to the *Project Root* directory. The provided path
/// is interpreted in a platform-specific way at compile time. So, for instance, an
/// invocation with a Windows path containing backslashes \ would not compile
/// correctly on Unix.
/// 
/// Will try to invoke the compiler if it is in **$PATH** or if a directory named
/// "blueprint-compiler" with *blueprint-compiler.py* is found in the *Project Root*.
///
/// This macro will yield an expression of type `&'static str` which is the compiled
/// UI XML of the Blueprint.
pub fn include_blp(input: TokenStream) -> TokenStream {
    // TODO: trigger a rebuild if .blp file has changed
    let ast: LitStr = syn::parse(input).unwrap();
    
    match compile_blp(&ast.value()) {
        Ok(xml) => format!("r###\"{}\"###", xml).parse().unwrap(),
        // TODO: compile_error!() instead of panic!()
        Err(error) => panic!("blueprint-compiler error: {}", error)
    }
}

// #[proc_macro]
// Compile a list of Blueprints into GtkUI and put them in a phf map
// pub fn blueprint_map(input: TokenStream) -> TokenStream {
    /*for path in search_blps(current_dir.to_str().unwrap()) {
        // If is absolute path, make relative path if possible
        let path = match path.strip_prefix(
            &format!("{}/", current_dir.to_str().unwrap())
        ) {
            Some(path) => String::from(path),
            None => path
        };
        // Remove the "./" if it exists in the path
        let path = match path.strip_prefix("./") {
            Some(path) => String::from(path),
            None => path
        };

        match compile_blp(&path) {
            Ok(xml) => map.entry(path, &xml),
            Err(error) => {
                errors = true;
                eprintln!("{error}");
                continue
            }
        };
    }
    format!("pub static COMPILED_BLPS: phf::Map<&'static str, &'static str> = \n{};\n", map.build())*/
// }