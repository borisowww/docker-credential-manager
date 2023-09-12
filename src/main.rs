mod utils;

use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::stdin;
use std::path::Path;

extern crate base64;

fn main() {
    // Read the standard input line.
    let mut std_input = String::new();
    stdin()
        .read_line(&mut std_input)
        .expect("Failed to read line.");
    let args: Vec<String> = env::args().collect();

    // Print the arguments
    // Check if config directory exists, if not create it.
    // This is a bit hacky, possibly an installer that sets up the config directory would be better.
    utils::create_config_directory_if_doesnt_exist();

    return match args[1].as_str() {
        "get" => read_credentials_file(std_input),
        "store" => store_credentials_file(std_input),
        "list" => list_credential_files(),
        _ => exit_with_error(format!("Unknown command: {}", args[1])),
    };
}

fn exit_with_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

/**
 * Store the credentials file.
 *
 * @param content The content of the credentials file.
*/
fn store_credentials_file(content: String) {
    let config: Value;
    match serde_json::from_str::<Value>(&content) {
        Err(why) => {
            exit_with_error(format!("Cannot parse json: {}", why));
        }
        Ok(json) => {
            config = json;

            let server_url = utils::get_key_from_config_value(&config, "ServerURL");
            // Escape the url backslashes, so it can be a directory.
            let mut file_path = utils::config_filename_from_server_url(server_url.to_string());

            // Prefix with the config directory.
            file_path = utils::get_config_file_path(file_path);

            // Create the file.
            let file_path = Path::new(&file_path);
            let file = File::create(&file_path);

            let mut file = file.expect("Cannot create file.");

            file.write_all(content.as_bytes())
                .expect("Cannot write content to file.");
        }
    }
}

/**
 * Read the credentials file.
 *
 * @param content The content of the credentials file.
*/
fn read_credentials_file(content: String) -> () {
    // Build a file path based on the server url.
    let path = utils::config_filename_from_server_url(content);
    let file_path = utils::get_config_file_path(path);

    match File::open(file_path) {
        Err(why) => {
            exit_with_error(format!("Cannot open file: {}", why));
        }
        Ok(mut file) => {
            let mut buffer = String::new();

            file.read_to_string(&mut buffer).expect("Cannot read file.");

            io::stdout()
                .write_all(buffer.as_ref())
                .expect("Cannot write to stdout.")
        }
    };
}

/**
 * List all the credential files.
*/
fn list_credential_files() -> () {
    let mut filenames: HashMap<String, String> = HashMap::new();

    let directory = utils::get_config_directory();
    let entries = fs::read_dir(&directory).expect("Could not read configurations directory.");

    // Loop through the entries and add them to the hashmap.
    for entry in entries {
        let filename = entry
            .expect("Can't open directory entry")
            .file_name()
            .into_string()
            .expect("Some error");

        let file_content =
            fs::read_to_string(format!("{}/{}", &directory, &filename)).expect("Cannot read file.");

        let json = serde_json::from_str::<Value>(&file_content).expect("can't load json");

        let username = json.get("Username").expect("can't get username");

        filenames.insert(filename, username.to_string());
    }

    // Convert the hashmap to json.
    let json =
        serde_json::to_string(&filenames).expect("Cannot convert to string for some reason?");

    // Write the json to stdout.
    io::stdout()
        .write(json.as_bytes())
        .expect("Can't write to stdout.");
}
