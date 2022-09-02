use std::io::{self, Write, stdin, stdout};
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

fn tokenize(s: String) -> Vec<String> {
    let split: Vec<String> = s.as_str().split_whitespace().map(String::from).collect();
    split
}

fn run(argv: Vec<&str>, exe_path_list: &Vec<&str>) -> u8 {

    match argv[0] {
        // BUILTINS
        "exit" => {
            std::process::exit(0);
        },
        "hi" => {
            println!("Hello!");
            return 0;
        },

        // if it's not a builtin
        exe => { 

            for exe_path in exe_path_list {
                match fs::read_dir(exe_path) {
                    Err(E) => {panic!("FUCK! {}", E)},
                    Ok(paths) => {
                        for path in paths {

                            // generate the name of the exe
                            let unwrapped = path.unwrap();
                            let new_path = unwrapped.path();
                            let stem = new_path.file_stem().unwrap();
                            let found_exe = stem.to_str().unwrap();

                            if found_exe == exe {

                                match Command::new(new_path.to_str().unwrap()).args(&argv[1..]).status() {
                                    Ok(S) => {
                                        match S.code() {
                                            Some(code) => {return (code as u8);},
                                            None => {return 130;}
                                        }},
                                    Err(E) => {println!("Could not execute function."); return 126},
                                }
                                             


                            }
                        }
                    }
                }
            }

            
        }
    }

    println!("Error, unknown command."); 
    return 127;
}

fn main() {
    let mut exe_path_list: Vec<&str> = vec!["./target/debug/"];

    loop {
        print!("> "); // print the prompt  
        stdout().flush().unwrap(); // flush stdout


        // read input into the buffer 
        let mut buffer = String::new();
        
        if let Err(E) = stdin().read_line(&mut buffer) {
            println!("Unexpected error: {}", E);
            std::process::exit(1);
        };
        // dbg!(&buffer);


        // tokenize
        let argv_string: Vec<String> = tokenize(buffer);
        let argv: Vec<&str> = argv_string.iter().map(|x| x as &str).collect();

        //dbg!(&argv);

        if (argv.len() == 0) {
            continue;
        }


        // execute
        let exit = run(argv, &exe_path_list);

        print!("(exit {}) ", exit);

        

    }

}

// BUILTIN FUNCTIONS

fn builtin_exit() {
    
}