use proc_macro::{self, TokenStream};
use syn::LitStr;
use std::process::Command;
use std::io::ErrorKind;


#[proc_macro]
/// Includes a GtkUI comiled from a Blueprint file as a string.
///
/// The file is located relative to the *Project Root* directory. The provided path
/// is interpreted in a platform-specific way at compile time. So, for instance, an
/// invocation with a Windows path containing backslashes \ would not compile
/// correctly on Unix.
///
///This macro will yield an expression of type `&'static str` which is the compiled
/// UI XML of the Blueprint.
pub fn include_blp(input: TokenStream) -> TokenStream {
    let ast: LitStr = syn::parse(input).unwrap();
    
    match compile_blp(&ast.value()) {
        Ok(xml) => xml.parse().unwrap(),
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


// Look for any .blp files in the project (ignoring target and git dirs)
// 
// This function is recursive, and calls itself when it hits a directory. The `path`
// argument should be "." in the starting call.
// 
// Returns a Vec with the paths of all the .blp files to be compiled.
// fn search_blps(path: &str) -> Vec<String> {
//     let mut paths = Vec::<String>::new();

//     for dir_entry in std::fs::read_dir(path).unwrap() {
//         let dir_entry = dir_entry.unwrap();
//         // Shadowing entry_name is necessary so the &str isn't dropped
//         let entry_name = dir_entry.file_name();
//         let entry_name = match entry_name.to_str() {
//             Some(entry_name) => entry_name,
//             None => continue // name cannot be converted to utf8 string
//         };
//         let entry_type = match dir_entry.file_type() {
//             Ok(entry_type) => entry_type,
//             Err(_) => continue
//         };

//         // Ignore ".git" and "target" directories
//         if entry_type.is_dir() && entry_name != ".git" && entry_name != "target" {
//             // push all blp files found in this directory
//             for path in search_blps(&format!("{path}/{entry_name}")) {
//                 paths.push(path);
//             }
//         }
//         else if ( entry_type.is_file() || entry_type.is_symlink() ) &&
//             entry_name.strip_suffix(".blp").is_some()
//         {
//             // Found Blueprint file
//             paths.push(format!("{path}/{entry_name}"));
//         }
//     }

//     paths
// }


/// Uses the installed blueprint-compiler python script to compile `.blp` files to
/// `UI XML` that can be used by *GtkBuilder*. The compiler needs to be accessible
/// through the **$PATH** envirnoment variable.
fn compile_blp(path: &str) -> Result<String, String> {
    // TODO: Invoke blueprint-compiler from $PATH
    let mut compiler = Command::new("/home/marti/source/blueprint-compiler/blueprint-compiler.py");
    compiler.arg("compile");
    compiler.arg(path);

    // Try to run blueprint-compiler and Check that compiler is installed in the system
    let output = match compiler.output() {
        Ok(output) => output,
        Err(error) =>
            return if error.kind() == ErrorKind::NotFound {
                Err("Blueprint Compiler not found. Make sure it is in $PATH".to_string())
            } else {
                Err("Unknown error occurred while invoking compiler".to_string())
            }
    };

    let compiled_blp = String::from_utf8(output.stdout).unwrap();
    
    match output.status.code() {
        Some(code) if code > 0 => println!("blueprint-compiler exit code: {}", code),
        _ => {}
    };

    if output.status.success() {
        Ok(format!("r###\"{}\"###", compiled_blp))
    } else {
        // When blueprint-compiler reaches an error in the blueprint file's source code
        // it will exit with 1 and the error info in stdout. Other errors will be written
        // to stderr. To show all errors, return Err Result with both stdout and stderr
        Err(match output.status.code() {
            Some(code) => format!("blueprint-compiler exit code: {}\n{}\n{}", code, compiled_blp, String::from_utf8(output.stderr).unwrap()),
            None => format!("{}\n{}", compiled_blp, String::from_utf8(output.stderr).unwrap())
        })
    }
}