use std::fs;
use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Debug, PartialEq, Deserialize)]
struct Workflow {
    on: Trigger,
    name: String,
    jobs: HashMap<String, Job>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum Trigger {
    TriggerAtom(String),
    TriggerList(Vec<String>),
    TriggerPush {
        push: TriggerPushInner,
    },
    TriggerSchedule {
        schedule: Vec<TriggerScheduleInner>,
    },
}

#[derive(Debug, PartialEq, Deserialize)]
struct TriggerPushInner {
    branches: Option<StringList>,
    tags: Option<StringList>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct TriggerScheduleInner {
    cron: String,
    branches: Option<StringList>,
    tags: Option<StringList>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Job {
    needs: Option<StringList>,
    #[serde(rename = "if")]
    conditional: Option<String>,
    strategy: Option<Strategy>,
    name: Option<String>,
    #[serde(rename = "runs-on")]
    runs_on: Option<String>,
    #[serde(rename = "timeout-minutes")]
    timeout_minutes: Option<u8>,
    #[serde(rename = "cancel-timeout-minutes")]
    cancel_timeout_minutes: Option<u8>,
    #[serde(rename = "continue-on-error")]
    continue_on_error: Option<bool>,
    container: Option<Container>,
    services: Option<HashMap<String,Container>>,
    steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Strategy {
    #[serde(rename = "fail-fast")]
    fail_fast: bool,
    #[serde(rename = "max-parallel")]
    max_parallel: bool,
    matrix: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum Container {
    Name(String),
    Properties {
        image: String,
        options: String,
        env: HashMap<String,String>,
        ports: Vec<String>,
        volumes: Vec<String>,
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Step {
    name: String,
    run: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum StringList {
    Atom(String),
    List(Vec<String>),
}

fn main() {
    let s = fs::read_to_string("example.yml")
        .expect("Something went wrong reading the file");
    let workflow: Workflow = serde_yaml::from_str(&s)
        .expect("Something went wrong parsing the workflow");
    print!("{:?}", workflow)
}
