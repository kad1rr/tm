use std::process::{Command, exit};
use std::time::Instant;
use std::env;

fn main() {
    let mut parts = Vec::<String>::new();

    if env::args().len() == 1 {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input).expect("Error: Unable to read input");
        
        parts = input.trim().split_whitespace().map(|s| s.to_string()).collect();
    } else {
        parts = env::args().skip(1).collect();
    }

    if parts.is_empty() {
        eprintln!("Error: Empty input");
        exit(1);
    }

    let current_dir = env::current_dir().expect("Error: Unable to get the current directory");

    // Execute the command based on the target operating system.
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .current_dir(current_dir)
            .args(&parts)
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .current_dir(current_dir)
            .args(&parts)
            .output()
    };

    let command_start_time = Instant::now();
    let tm_start_time = Instant::now();


    match output {
        Ok(output) => {
            if output.status.success() {
                println!(
                    "\x1b[1;32mSuccessfully completed!\x1b[0m\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                eprintln!(
                    "\x1b[1;31mError occurred!\x1b[0m\n{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            println!("Command Elapsed Time: {:?}", command_start_time.elapsed());
        }
        Err(e) => {
            eprintln!("\x1b[1;31mError: {}\x1b[0m", e);
        }
    }

    println!("TM Elapsed Time: {:?}", tm_start_time.elapsed());

}
