use nix::unistd::{execvp, fork, ForkResult, Pid};
use std::convert::Infallible;
use std::env;
use std::ffi::CString;
use std::process;

fn tracee() -> Infallible {
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
    println!("I'll be traced.");

    execvp(&executable, &args[..]).expect("Couldn't execute given command.")
}

fn tracer(pid: Pid) {
    println!("Tracing {}", pid);
}

fn main() {
    match unsafe { fork() }.expect("Could not fork.") {
        ForkResult::Parent { child } => {
            tracer(child);
        }
        ForkResult::Child => {
            tracee();
        }
    };
}
