use std::{fs, io};
use std::process::{Command, Output};

use serde_derive::Deserialize;
use warp::{Filter, http::Response};

#[derive(Deserialize, Clone)]
struct ConfiguredScript {
    command: String,
    arguments: Vec<String>,
    name: String,
}

#[derive(Deserialize, Clone)]
struct ScappConfig {
    script: Vec<ConfiguredScript>
}

impl ScappConfig {
    fn config_for_name(&self, name: String) -> Option<&ConfiguredScript> {
        self.script.iter().find(|script| script.name == name)
    }
}

#[tokio::main]
async fn main() {
    let read_config = fs::read_to_string("/etc/scaap/scaap.toml").unwrap();
    let parsed_config = toml::from_str(&read_config);
    let scapp_config: ScappConfig = parsed_config.unwrap();
    let scripts = scapp_config.clone();
    println!("Starting script as a service with config {}", read_config);
    let script = move |name| execute_script(name, scripts.clone()).map(|result: io::Result<Output>| result.map(|output: Output| if output.status.success() {
        Response::builder().status(200).body(String::from_utf8(output.stdout).unwrap())
    } else {
        Response::builder().status(500).body(String::from_utf8(output.stderr).unwrap())
    }).unwrap_or_else(|err| Response::builder().status(503).body(err.to_string()))).unwrap_or_else(|| Response::builder().status(404).body("No such script defined".parse().unwrap()));
    let script_execution = warp::path!("script" / String).map(script);
    let routes = warp::get().and(script_execution);
    println!("Starting http server on 3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn execute_script(name: String, config: ScappConfig) -> Option<io::Result<Output>> {
    let configured_script = config.config_for_name(name);
    configured_script.map(|script| {
        Command::new(&script.command)
            .args(&script.arguments)
            .output()
    })
}