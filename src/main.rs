extern crate clap;
use clap::{Arg, App};
use std::path::Path;
use std::process::Command;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn link(existing_file: &str, new_link:&str) -> () {
    //println!("Linking {} to {}", existing_file, new_link);

    let status = Command::new("ln")
        .arg("-s")
        .arg(existing_file)
        .arg(new_link)
        .status().unwrap_or_else(|e| {
        panic!("failed to execute process, bad process! {}", e)
    });

    match status.code() {
        Some(c) => std::process::exit(c),
        // not sure if I care about the signal info here, do you?
        None => panic!("Empty status code, killed by a signal? Ouch!")
    };
}

fn repoint_existing_link(sa: Path, sb: Path) -> () {
    let meta_a = sa.metadata();
    let meta_b = sb.metadata();
    let metas = (sa.metadata(), sb.metadata());
    match metas {
        (Ok(a), Ok(b) => {
            let links = (a.is_symlink(), b.is_sylink());
            match links {
                (true, false) => link(b.to_str().unwrap(), a.to_str().unwrap()),
                (false, true) => link(a.to_str().unwrap(), b.to_str().unwrap()),
                (true, true) => exit_for_humans("Both paths are symlinks!", -1),
                (false, false) => exit_for_humans("Both paths are files!", -1)
            };
        },
        (Err(a), Ok(b)) => exit_for_humans("Failed to get metadata for <A>", -1),
        (Ok(a), Err(b)) => exit_for_humans("Failed to get metadata for <B>", -1),
        (Err(a), Err(b)) => exit_for_humans("Failed to get metadata for <A> and <B>", -1)
    };
}

fn exit_for_humans(msg: &str, status: i32) -> () {
    // ie, don't print panic() garbage
    println!("{}", msg);
    std::process::exit(status);
}

fn main() {

    let matches = App::new("simlink")
                      .version(VERSION)
                      .about("ln -s, meow/miaow")
                      .arg(Arg::with_name("A")
                               .help("Maybe a file, maybe a link you want to make?")
                               .required(true)
                               .index(1))
                      .arg(Arg::with_name("B")
                               .help("Maybe a file, maybe a link you want to make?")
                               .required(true)
                               .index(2))
                      .get_matches();

    // A + B are required, clap will exit before unwrap() might fail
    let file_a = matches.value_of("A").unwrap();
    let file_b = matches.value_of("B").unwrap();

    let path_a = Path::new(file_a);
    let path_b = Path::new(file_b);

    if let None = path_a.to_str() {
        exit_for_humans("Invalid filename for <A>", -1);
    };

    if let None = path_b.to_str() {
        exit_for_humans("Invalid filename for <B>", -1);
    };

    let sa = path_a.to_str().unwrap();
    let sb = path_b.to_str().unwrap();

    let paths = (path_a.exists(), path_b.exists());
    match paths {
        (true, false) => link(sa, sb),
        (false, true) => link(sb, sa),
        (true, true) => repoint_existing_link(path_a, path_b),
        (false, false) => exit_for_humans("Neither path exists", -1)
    };

}
