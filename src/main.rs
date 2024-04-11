use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::path::Path;
use chrono::Utc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the migration, this will be the suffix of the file
    #[arg(short, long)]
    name: String,

    /// Directory that the migration files should land in
    #[arg(short, long, default_value_t = String::from("migrations"))]
    path: String,

    /// Format of the prefix added to migration files
    #[arg(short, long, default_value_t = String::from("auto"))]
    format: String,
}

#[derive(Debug, PartialEq)]
enum PrefixType {
    DateTime,
    Unix,
    UnixMilli,
    Integer,
}


fn main() {
    let cli = Cli::parse();
    match setup_migration_files(&cli) {
        Ok(_) => println!("Migration files created successfully."),
        Err(e) => eprintln!("Failed to create migration files: {}", e),
    }
}

fn setup_migration_files(cli: &Cli) -> std::io::Result<()> {
    let dir_path = Path::new(&cli.path);
    fs::create_dir_all(dir_path)?;
    let mut prefix_type = parse_prefix_argument(&cli.format);
    let mut highest_integer: Option<i64> = None;

    if cli.format == "auto" {
        let (pt, hi) = determine_prefix_format(dir_path)?;
        prefix_type = pt;
        highest_integer = hi;
    }

    let prefix = generate_prefix(prefix_type, highest_integer);

    let migration_types = ["up", "down"];

    for migration_type in &migration_types {
        let filename = format!("{}/{}_{}.{}.sql", cli.path, prefix, cli.name, migration_type);
        let path = Path::new(&filename);
        File::create(&path)?;
        println!("✅ Created {}", path.display());
    }

    Ok(())
}

fn parse_prefix_argument(arg: &str) -> PrefixType {
    match arg {
        "datetime" => PrefixType::DateTime,
        "unix" => PrefixType::Unix,
        "unixmilli" => PrefixType::UnixMilli,
        "milli" => PrefixType::UnixMilli,
        "integer" => PrefixType::Integer,
        _ => PrefixType::UnixMilli
    }
}

fn determine_prefix_format(dir_path: &Path) -> Result<(PrefixType, Option<i64>)> {
    let mut most_recent_time = 0;
    let mut most_recent_prefix = PrefixType::UnixMilli;
    let mut highest_integer = None;

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        if entry.path().is_file() {
            if let Some(file_name) = entry.path().file_stem().and_then(|n| n.to_str()) {
                if let Some((prefix, _)) = file_name.split_once('_') {
                    let format = analyze_prefix(prefix);
                    if format != PrefixType::Integer {
                        let time = if let Ok(num) = prefix.parse::<i64>() { num } else { continue };
                        if time > most_recent_time {
                            most_recent_time = time;
                            most_recent_prefix = format;
                        }
                    } else {
                        if let Ok(num) = prefix.parse::<i64>() {
                            if highest_integer.map_or(true, |max| num > max) {
                                highest_integer = Some(num);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok((most_recent_prefix, highest_integer))
}

fn analyze_prefix(prefix: &str) -> PrefixType {
    match prefix.len() {
        14 if prefix.chars().all(char::is_numeric) => PrefixType::DateTime,
        10 if prefix.chars().all(char::is_numeric) => PrefixType::Unix,
        13 if prefix.chars().all(char::is_numeric) => PrefixType::UnixMilli,
        _ if prefix.chars().all(char::is_numeric) => PrefixType::Integer,
        _ => PrefixType::Integer,
    }
}

fn generate_prefix(prefix_type: PrefixType, highest_integer: Option<i64>) -> String {
    let now = Utc::now();
    match prefix_type {
        PrefixType::DateTime => now.format("%Y%m%d%H%M%S").to_string(),
        PrefixType::Unix => now.timestamp().to_string(),
        PrefixType::UnixMilli => now.timestamp_millis().to_string(),
        PrefixType::Integer => {
            let next_int = highest_integer.map_or(1, |max| max + 1);
            next_int.to_string()
        }
    }
}
