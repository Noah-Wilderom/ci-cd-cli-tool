use clap::{Parser, Subcommand};
use webbrowser;
use std::env;
use std::fs;


#[allow(non_snake_case)]
mod App {
    pub mod parser;
}

use App::parser::YamlParser;


#[derive(Parser)]
#[command(name = "Continuous Integration Tool")]
#[command(author = "Noah W. <wilderomnoah@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "https://noahdev.nl/ci-tool", long_about = None)]
struct Cli {

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Auth {},
    Test {}
}

fn make_auth_url() -> String {
  let params = [
    String::from("redirect_uri=http://localhost:8000"),
    String::from("response_type=code"),
    String::from("approval_prompt=auto"),
  ]
  .join("&");
  format!("http://cit-api.test/oauth/authorize?{}", params)
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Auth { }) => {
            let auth_url = make_auth_url();
            if webbrowser::open(&auth_url).is_err() {
                // Try manually
                println!("Visit the following URL to authorize your app with CIT Api:");
                println!("{}\n", auth_url);
            }
        }
        Some(Commands::Test{ }) => {
            let current_dir = env::current_dir().expect("Failed to load working directory");
            let deploy_dir = current_dir.join(".cit/deploy");

            if let Ok(entries) = fs::read_dir(&deploy_dir) {
                for entry in entries {
                    if let Ok(dir_entry) = entry {
                        let file_path = dir_entry.path();

                        if file_path.is_file() {
                            if let Some(file_name) = file_path.file_name() {
                                if let Some(file_name_str) = file_name.to_str() {
                                    YamlParser::new().load(file_name_str);
                                }
                            }
                        }
                    }
                }
            }

        }
        None => {}
    }

    // Continued program logic goes here...
}
