use nix::unistd::execvp;
use std::env;
use std::ffi::CString;
use std::process;

fn main() {
    let args: Vec<CString> = env::args()
        // Skip the path of this file.
        .skip(1)
        .map(|a| CString::new(a).expect("Invalid string argument."))
        .collect();
    let executable = match args.first() {
        Some(e) => e,
        None => {
            println!("Usage: listen <executable> [args]");
            process::exit(1)
        }
    };

    execvp(&executable, &args[..]).expect("Couldn't execute given command.");
}
