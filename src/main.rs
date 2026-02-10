use serde::{Deserialize, Serialize};
use std::{env, process::Command, process::exit};

const RUNTIME_PAYLOAD: &str = env!("RUNTIME_PAYLOAD");

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RuntimePayload {
    path: String,
    args: Vec<String>,
    envs: Vec<(String, String)>,
}

impl RuntimePayload {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_envs(&self) -> &Vec<(String, String)> {
        &self.envs
    }
}

fn main() {
    if RUNTIME_PAYLOAD.is_empty() {
        setup();
    } else {
        launch();
    }
}

fn setup() {
    todo!();
}

fn launch() {
    let payload: RuntimePayload = serde_json::from_str(RUNTIME_PAYLOAD).unwrap();

    let path = payload.get_path();
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut envs: Vec<(String, String)> = env::vars().collect();

    for it in payload.get_args() {
        args.push(it.clone());
    }

    for it in payload.get_envs() {
        envs.push(it.clone());
    }

    let mut cmd = Command::new(path);
    cmd.args(args);
    cmd.envs(envs);

    let status = cmd.status().unwrap();

    exit(status.code().unwrap_or(1));
}
