use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::{env, process::Command, process::exit};

const RUNTIME_PAYLOAD: &str = env!("RUNTIME_PAYLOAD");

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RuntimePayload {
    path: String,

    #[serde(default)]
    args: Vec<String>,

    #[serde(default)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ReportPayload {
    args: Vec<String>,
    envs: Vec<(String, String)>,
}

impl ReportPayload {
    pub fn new() -> Self {
        let mut envs: Vec<(String, String)> = std::env::vars().collect();
        envs.sort_by_key(|(a, _)| a.clone());

        Self {
            args: std::env::args().collect(),
            envs,
        }
    }
}

fn main() {
    if RUNTIME_PAYLOAD.is_empty() {
        setup();
    } else {
        report();
        launch();
    }
}

fn setup() {
    todo!();
}

fn report() {
    let report = ReportPayload::new();
    let text = serde_json::to_string(&report).unwrap();
    let mut file = File::create("bin-proxy_report.json").unwrap();
    file.write_all(text.as_bytes()).unwrap();
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
