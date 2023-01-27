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

fn get_template_data(url: &str) -> Result<String, curl::Error> {
    let mut data = vec![];

    let mut easy = Easy::new();
    easy.url(url)?;
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    easy.perform()?;

    let data = String::from_utf8(data)
        .expect("Failed to parse template data from UTF-8");
    Ok(data)
}

fn main() {
    let args = Args::parse();
    
    let source: String = match args.source {
        Some(s) => s,
        None => "github/gitignore".to_string()
    };

    let template_name: String = args.template_name;
    let url = format!("https://raw.githubusercontent.com/{}/main/{}.gitignore", source, template_name);

    let data = get_template_data(&url)
        .expect("Failed to fetch template data");
    
    if  PathBuf::from(".gitignore").exists() && !args.force {
        println!("File .gitignore already exists. Use --force to overwrite.");
        return
    }

    fs::write(".gitignore", data)
        .expect("Failed to write to .gitignore file");
}
