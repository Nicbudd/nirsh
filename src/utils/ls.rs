use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Error: No args given.");
        std::process::exit(1);
    }

    let target_path = &args[1];

    let paths = fs::read_dir(target_path).unwrap();

    println!("Hmmm, I see some files here.");

    for path in paths {
        // let files;
        println!("{}", path.unwrap().path().display());
    }

}