use std::process::Command;
use std::io::ErrorKind;

// Look for any .blp files in the project (ignoring target and git dirs)
// 
// This function is recursive, and calls itself when it hits a directory. The `path`
// argument should be "." in the starting call.
// 
// Returns a Vec with the paths of all the .blp files to be compiled.
pub fn search_blps(start_path: &str) -> Vec<String> {
    let mut paths = Vec::<String>::new();

    for dir_entry in std::fs::read_dir(start_path).unwrap() {
        let dir_entry = dir_entry.unwrap();
        // Shadowing entry_name is necessary so the &str isn't dropped
        let entry_name = dir_entry.file_name();
        let entry_name = match entry_name.to_str() {
            Some(entry_name) => entry_name,
            None => continue // name cannot be converted to utf8 string
        };
        let entry_type = match dir_entry.file_type() {
            Ok(entry_type) => entry_type,
            Err(_) => continue
        };

        // Ignore ".git" and "target" directories
        if entry_type.is_dir() && entry_name != ".git" && entry_name != "target" {
            // push all blp files found in this directory
            for path in search_blps(&format!("{start_path}/{entry_name}")) {
                paths.push(path);
            }
        }
        else if ( entry_type.is_file() || entry_type.is_symlink() ) &&
            entry_name.strip_suffix(".blp").is_some()
        {
            // Found Blueprint file
            paths.push(format!("{start_path}/{entry_name}"));
        }
    }

    paths
}


/// Uses the installed blueprint-compiler python script to compile `.blp` files to
/// `UI XML` that can be used by *GtkBuilder*. The compiler needs to be accessible
/// through the **$PATH** envirnoment variable or it needs to be in
/// "blueprint-compiler/blueprint-compiler.py" in the Project Root.
pub fn compile_blp(path: &str) -> Result<String, String> {
    /* These are commands that the function could use as the compiler. The ones with
       the "./" prefix are in the $PATH environment variable, and the ones that have
       that prefix are relative to the current project's root directory. */
    let possible_compilers = [
        // Rank by which one is more likely
        &mut Command::new("blueprint-compiler"),
        &mut Command::new("./blueprint-compiler/blueprint-compiler.py"),
        &mut Command::new("blueprint-compiler.py"),
        &mut Command::new("./blueprint-compiler/blueprint-compiler"),
    ];

    /* Try to find the right compiler in one of the above locations.
       Use whichever one is successful */
    for compiler in possible_compilers {
        compiler.arg("compile");
        compiler.arg(path);

        /* The output contains the command's exit status, stdout, and stderr.
           When Command::output() returns an Err, it means that the command could
           not run for some reason */
        let output = match compiler.output() {
            Ok(output) => output,
            Err(error) =>
                if error.kind() == ErrorKind::NotFound {
                    // Try another command
                    continue
                } else {
                    return Err(format!("Unknown error occurred while invoking compiler:\n{}", error))
                }
        };

        let compiled_blp = String::from_utf8(output.stdout).unwrap();
        let error = String::from_utf8(output.stderr).unwrap();
        
        if output.status.success() {
            return Ok(compiled_blp)
        } else {
            // When blueprint-compiler reaches an error in the blueprint file's source code
            // it will exit with 1 and the error info in stdout. Other errors will be written
            // to stderr. To show all errors, return Err Result with both stdout and stderr
            return Err(match output.status.code() {
                Some(code) => format!("blueprint-compiler exit code: {}\n{}\n{}", code, compiled_blp, error),
                None => format!("{}\n{}", compiled_blp, error)
            })
        }
    }

    Err("Blueprint Compiler not found. Make sure it is in $PATH or ./blueprint-compiler/blueprint-compiler.py".to_string())
}

// TODO:
// macro_rules! proc_macro_error {
//     ($($arg:tt)*) => {{
//         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//         res
//     }};
// }