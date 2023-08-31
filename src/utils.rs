use serde_json::Value;
use std::fs;
use std::path::Path;
use url::Url;
/**
 * Get the full filename from the docker server url.
 *
 * @param url The server url.
 * @return The full file path.
*/
pub fn config_filename_from_server_url(url: String) -> String {
    let formatted_url;

    // For some reason Docker is not consistent with what it sends, sometimes it's base path, sometimes it's
    // the full URL, so we need to make sure everything is parsed to a base url kind of format.

    // Here we ensure that the URL starts with https to make the Url::parse method happy :)
    if !url.starts_with("http://") && !url.starts_with("https://") {
        formatted_url = format!("https://{}", url);
    } else {
        formatted_url = url;
    }

    let parsed_url = Url::parse(formatted_url.as_str()).expect("Failed to parse URL");
    let base_url = parsed_url.host_str().expect("No host in URL");

    return String::from(base_url);
}

/**
 * Get the full path to the config directory.
 *
 * @return The full path to the config directory.
*/
pub fn get_config_directory() -> String {
    let home = std::env::var("HOME").expect("Cannot get home directory.");

    format!("{}/.docker-credential-manager/configs", home)
}

/**
 * Get the full path a the config file.
 *
 * @param path The path to the config file.
 * @return The full path to the config file.
*/
pub fn get_config_file_path(path: String) -> String {
    let config_directory = get_config_directory();

    format!("{}/{}", config_directory, path)
}

/**
 * Get a key from a config value.
 *
 * @param config The config value.
 * @param key The key to get.
 * @return The value of the key.
*/
pub fn get_key_from_config_value(config: &Value, key: &str) -> String {
    let value = config[key]
        .as_str()
        .expect(&format!("Cannot get {} from json. Key: {}", key, key));

    value.to_string()
}

/**
* Create the config directory.
*/
pub fn create_config_directory_if_doesnt_exist() -> () {
    let config_directory = get_config_directory();

    let path = Path::new(&config_directory);

    if !path.exists() {
        fs::create_dir_all(path).expect("Cannot create config directory.");
    }
}
