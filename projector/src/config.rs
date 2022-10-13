use std::path::PathBuf;
use crate::opts::Opts;

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
pub struct Config {
    pub operation: Operation,
    pub pwd: PathBuf,
    pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
    type Error = anyhow::Error;

    fn try_from(value: Opts) -> Result<Self> {
        let operation = value.args.try_into()?;
        let config = get_config(value.config)?;
        let pwd = get_pwd(value.pwd)?;

        return Ok(Config {
            operation,
            config,
            pwd
        })
    }
}

#[derive(Debug)]
pub enum Operation {
    Print(Option<String>),
    Add(String, String),
    Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut value = value;
        if value.len() == 0 {
            return Ok(Operation::Print(None));
        }

        let term = value.get(0).expect("Expect 1 argument to exist");
        if term == "add" {
            if value.len() != 3 {
                return Err(anyhow!(
                    "Expected two arguments but got {}",
                    value.len() - 1
                ));
            }
            let mut drain = value.drain(1..=2);
            return Ok(Operation::Add(
                drain.next().expect("to exist"),
                drain.next().expect("to exist"),
            ));
        }
        if term == "rm" {
            if value.len() != 2 {
                return Err(anyhow!(
                    "Expected two arguments but got {}",
                    value.len() - 1
                ));
            }
            let arg = value.pop().expect("to exist");
            return Ok(Operation::Remove(arg));
        }

        if value.len() > 1 {
            return Err(anyhow!(
                "Operation print expects 0 or 1 arguments but got {}",
                value.len() - 1
            ));
        }
        let arg = value.pop().expect("to exist");
        return Ok(Operation::Print(Some(arg)));
    }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(v) = config {
        return Ok(v);
    }

    let mut loc = PathBuf::from("~/Library/Preferences/");

    loc.push("projector");
    loc.push("projector.json");

    return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(pwd) = pwd {
        return Ok(pwd);
    }

    let pwd = std::env::current_dir().context("errored getting current directory")?;
    return Ok(pwd);
}
