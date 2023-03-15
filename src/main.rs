use std::time::{SystemTime,UNIX_EPOCH};

use std::fs::File;
use std::path::Path;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    name: String,
}



fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let args = Cli::parse();
            let mig_path = String::from("./migrations");
            let mig_name = args.name;
            
            let migration_files = vec!["up", "down"];
            for suffix in migration_files.iter() {
                let path_str = format!("{}/{}_{}.{}.sql", mig_path, secs, mig_name, suffix);
                let path = Path::new(&path_str);

                let display = path.display();
                match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", display, why),
                    Ok(_) => {
                        println!("✅ created {}", display)
                    },
                };
            }
        },
        Err(err) => panic!("yikkkkkes -- {}", err)
    }
}
