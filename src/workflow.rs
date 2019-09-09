use std::collections::HashMap;

use serde::{Deserialize};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Workflow {
    pub on: Trigger,
    pub name: String,
    pub jobs: HashMap<String, Job>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Trigger {
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
pub struct TriggerPushInner {
    pub branches: Option<StringList>,
    pub tags: Option<StringList>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TriggerScheduleInner {
    pub cron: String,
    pub branches: Option<StringList>,
    pub tags: Option<StringList>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Job {
    pub needs: Option<StringList>,
    #[serde(rename = "if")]
    pub conditional: Option<String>,
    pub strategy: Option<Strategy>,
    pub name: Option<String>,
    #[serde(rename = "runs-on")]
    pub runs_on: Option<String>,
    #[serde(rename = "timeout-minutes")]
    pub timeout_minutes: Option<u8>,
    #[serde(rename = "cancel-timeout-minutes")]
    pub cancel_timeout_minutes: Option<u8>,
    #[serde(rename = "continue-on-error")]
    pub continue_on_error: Option<bool>,
    pub container: Option<Container>,
    pub services: Option<HashMap<String,Container>>,
    pub steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Strategy {
    #[serde(rename = "fail-fast")]
    pub fail_fast: bool,
    #[serde(rename = "max-parallel")]
    pub max_parallel: bool,
    pub matrix: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Container {
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
pub struct Step {
    pub name: String,
    pub run: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum StringList {
    Atom(String),
    List(Vec<String>),
}

pub fn parse(s: &str) -> Result<Workflow,String> {
    match serde_yaml::from_str(s) {
        Ok(w) => Ok(w),
        Err(e) => Err(e.to_string()),
    }
}
