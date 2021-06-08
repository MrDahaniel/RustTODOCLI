mod to_do;
use to_do::to_do_factory;

mod state;
use state::read_file;

use std::env;

use serde_json::{value::Value, Map};

mod processes;
use processes::processes_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let state: Map<String, Value> = read_file(&String::from("./state.json"));

    let status: String;

    match &state.get(*&title) {
        Some(result) => status = result.to_string().replace("\"", ""),
        None => status = "pending".to_string(),
    }

    let item = to_do_factory(&status, title).expect(&status);

    processes_input(item, command.to_string(), &state);
}
