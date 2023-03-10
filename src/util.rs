use serde_json::Value;
use std::fs;
pub fn get_parameter(param_name: &str) -> String {
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let parameters = fs::read_to_string(format!("{}/src/{}", path, "parameters.json"))
        .expect("Unable to read file");
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(parameters.as_str()).unwrap();
    let mut return_value = String::new();
    return_value = v[param_name].as_str().clone().unwrap().to_string();
    // Access parts of the data by indexing with square brackets.
    return return_value;
}
