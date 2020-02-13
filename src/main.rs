extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use std::process::Command;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn do_link(existing_file: &str, new_link: &str, force: bool) -> () {
    //println!("Linking {} to {}", existing_file, new_link);

    let mut params = vec!["-s"];
    if force {
        params.push("-f");
    }

    let status = Command::new("ln")
        .args(&params[..])
        .arg(existing_file)
        .arg(new_link)
        .status()
        .unwrap_or_else(|e| panic!("failed to execute process, bad process! {}", e));

    match status.code() {
        Some(c) => std::process::exit(c),
        // not sure if I care about the signal info here, do you?
        None => panic!("Empty status code, killed by a signal? Ouch!"),
    };
}

fn link(existing_file: &str, new_link: &str) -> () {
    do_link(existing_file, new_link, false);
}

fn link_with_force(existing_file: &str, new_link: &str) -> () {
    do_link(existing_file, new_link, true);
}

fn repoint_existing_link(sa: &Path, sb: &Path) -> () {
    let metas = (sa.symlink_metadata(), sb.symlink_metadata());
    match metas {
        (Ok(a), Ok(b)) => {
            let links = (a.file_type().is_symlink(), b.file_type().is_symlink());
            match links {
                (true, false) => link_with_force(sb.to_str().unwrap(), sa.to_str().unwrap()),
                (false, true) => link_with_force(sa.to_str().unwrap(), sb.to_str().unwrap()),
                (true, true) => exit_for_humans("Both paths are symlinks!", -1),
                (false, false) => exit_for_humans("Both paths are files!", -1),
            };
        }
        (Err(_a), Ok(_b)) => exit_for_humans("Failed to get metadata for <A>", -1),
        (Ok(_a), Err(_b)) => exit_for_humans("Failed to get metadata for <B>", -1),
        (Err(_a), Err(_b)) => exit_for_humans("Failed to get metadata for <A> and <B>", -1),
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
        .arg(
            Arg::with_name("A")
                .help("Maybe a file, maybe a link you want to make?")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("B")
                .help("Maybe a file, maybe a link you want to make?")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("force")
                .help("Repoint an existing link")
                .short("f")
                .takes_value(false),
        )
        .get_matches();

    // A + B are required, clap will exit before unwrap() might fail
    let file_a = matches.value_of("A").unwrap();
    let file_b = matches.value_of("B").unwrap();
    let force = matches.is_present("force");

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

    let paths = (path_a.exists(), path_b.exists(), force);
    match paths {
        (true, false, _) => link(sa, sb),
        (false, true, _) => link(sb, sa),
        (true, true, true) => repoint_existing_link(path_a, path_b),
        (true, true, false) => exit_for_humans("Both paths exist (use -f to override)", -1),
        (false, false, _) => exit_for_humans("Neither path exists", -1),
    };
}
