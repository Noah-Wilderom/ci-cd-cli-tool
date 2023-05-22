use std::io::Result;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::YamlLoader;


pub struct YamlParser {
    // Add any fields or methods you need for the YAML parser
}

impl YamlParser {
    pub fn new() -> Self {
        YamlParser {
            // Initialize any fields you need for the YAML parser
        }
    }

    pub fn load(self, file_name: &str) -> Result<()> {
        // Append the file to the current directory
        let current_dir = env::current_dir().expect("Failed to load working directory");
        let deploy_dir = current_dir.join(".cit/deploy");
        let file_path = deploy_dir.join(file_name);

        // Initialize macros
        let mut file = File::open(file_path).expect("Unable to open file");
        let mut contents = String::new();

        // Parse the file to string
        file.read_to_string(&mut contents)
        .expect("Unable to read file");

        // Parse the string to yaml
        let docs = YamlLoader::load_from_str(&contents).unwrap();

        for doc in docs {
            println!("Doc: {:?}", doc)
        }

        Ok(()) // Return Ok(()) if loading is successful
    }
}