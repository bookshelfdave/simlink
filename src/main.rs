extern crate clap;
use clap::{Arg, App};
use std::path::Path;
use std::process::Command;

fn link(existing_file: &str, new_link:&str) {
    println!("Linking {} to {}", existing_file, new_link);

    let status = Command::new("ln")
        .arg("-s")
        .arg(existing_file)
        .arg(new_link)
        .status().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    println!("process exited with: {}", status);

}

fn main() {

    let matches = App::new("simlink")
                      .version("1.0")
                      .author("Dave Parfitt <diparfitt@gmail.com>")
                      .about("ln -s, go away")
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
    println!("Value for config: {} {}", file_a, file_b);

    let path_a = Path::new(file_a);
    let path_b = Path::new(file_b);

    let sa = match path_a.to_str() {
        Some(s) => s,
        None => panic!("Invalid filename for <A>")
    };

    let sb = match path_b.to_str() {
        Some(s) => s,
        None => panic!("Invalid filename for <B>")
    };

    let paths = (path_a.exists(), path_b.exists());
    match paths {
        (true, false) => link(sa, sb),
        (false, true) => link(sb, sa),
        (true, true) => panic!("Both files already exist"),
        (false, false) => panic!("Neither file exists")
    };

}
