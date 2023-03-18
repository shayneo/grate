use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the migration, this will be the suffix of the file
    #[arg(short, long)]
    name: String,

    /// Directory that the migration files should land in
    #[arg(short, long, default_value_t = String::from("migrations"))]
    path: String,
}

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let secs = n.as_secs();
            let args = Cli::parse();
            let mig_path = args.path;
            let mig_name = args.name;
            let dir_path = Path::new(&mig_path);
            fs::create_dir_all(dir_path).unwrap_or_else(|e| panic!("Error creating dir: {}", e));

            let migration_files = vec!["up", "down"];
            for suffix in migration_files.iter() {
                let path_str = format!("{}/{}_{}.{}.sql", mig_path, secs, mig_name, suffix);
                let path = Path::new(&path_str);

                let display = path.display();
                match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", display, why),
                    Ok(_) => {
                        println!("✅ created {}", display)
                    }
                };
            }
        }
        Err(err) => panic!("yikkkkkes -- {}", err),
    }
}
