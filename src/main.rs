use std::fs;
use std::path::PathBuf;

use clap::Parser;
use curl::easy::Easy;

/// A tool to fetch a specified gitignore template
#[derive(Parser, Debug)]
struct Args {
    /// The repository to pull templates from (default: "github/gitignore")
    #[clap(short, long)]
    source: Option<String>,

    /// Overwrite existing .gitignore file (default: false)
    #[clap(short, long)]
    force: bool,

    /// The name of the template to fetch (case-sensitive)
    #[clap(required = true)]
    template_name: String,
}

fn main() {
    let args = Args::parse();
    
    let source: String = match args.source {
        Some(s) => s,
        None => "github/gitignore".to_string()
    };

    let template_name: String = args.template_name;
    let url = format!("https://raw.githubusercontent.com/{}/main/{}.gitignore", source, template_name);

    let mut data = vec![];

    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    easy.perform().unwrap();

    let data = String::from_utf8(data).unwrap();
    
    match PathBuf::from(".gitignore").exists() {
        true => {
            match args.force {
                true => {
                    fs::write(".gitignore", data).unwrap();
                }
                false => {
                    println!("File .gitignore already exists. Use --force to overwrite.");
                }
            }
        },
        false => {
            fs::write(".gitignore", data).unwrap();
        }
    } 
}
