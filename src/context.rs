use std::{env, fs, path::Path};

use json::JsonValue;
use crate::error::ActionsError;

/// Context class injected by the action worker.
///
/// Find detailed documentation in
/// [GitHub's documentation for Contexts](https://docs.github.com/en/actions/learn-github-actions/contexts)
#[derive(Debug, Clone)]
pub struct Context {
    pub payload: JsonValue,
    pub event_name: String,
    pub sha: String,
    pub ref_: String,
    pub workflow: String,
    pub action: String,
    pub actor: String,
    pub job: String,
    pub run_attempt: u8,
    pub run_number: u8,
    pub run_id: u128,
    pub api_url: String,
    pub server_url: String,
    pub graphql_url: String,
    pub repo: Repo,
}

#[derive(Debug, Clone)]
pub struct Repo {
    owner: String,
    repo: String,
}

/// Gets the context object injected by the GitHub action
///
/// ```
/// # use Context;
/// let data = get_context().unwrap();
/// println!("Event is {}", data.event_name);
/// ```
pub fn get_context() -> Result<Context, ActionsError> {
    let mut payload: JsonValue = JsonValue::Null;

    if let Ok(github_event_path) = env::var("GITHUB_EVENT_PATH") {
        if Path::new(&github_event_path).exists() {
            match fs::read_to_string(&github_event_path) {
                Ok(content) => match json::parse(&content) {
                    Ok(parsed_json) => payload = parsed_json,
                    Err(err) => println!("Failed to parse JSON: {}", err),
                },
                Err(err) => println!("Failed to read file: {}", err),
            }
        } else {
            println!("GITHUB_EVENT_PATH {} does not exist", github_event_path)
        }
    }

    let repo: Repo = get_repo(&payload)?;

    Ok(Context {
        payload,
        event_name: get_env("GITHUB_EVENT_NAME"),
        sha: get_env("GITHUB_SHA"),
        ref_: get_env("GITHUB_REF"),
        workflow: get_env("GITHUB_WORKFLOW"),
        action: get_env("GITHUB_ACTION"),
        actor: get_env("GITHUB_ACTOR"),
        job: get_env("GITHUB_JOB"),
        run_attempt: get_env("GITHUB_RUN_ATTEMPT").parse::<u8>().unwrap(),
        run_number: get_env("GITHUB_RUN_NUMBER").parse::<u8>().unwrap(),
        run_id: get_env("GITHUB_RUN_ID").parse::<u128>().unwrap(),
        api_url: get_env_or("GITHUB_API_URL", "https://api.github.com"),
        server_url: get_env_or("GITHUB_SERVER_URL", "https://github.com"),
        graphql_url: get_env_or("GITHUB_GRAPHQL_URL", "https://api.github.com/graphql"),
        repo,
    })
}

fn get_env(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(var) => var,
        Err(_) => panic!("Variable {} not defined", var_name),
    }
}

fn get_env_or(var_name: &str, default: &str) -> String {
    match env::var(var_name) {
        Ok(var) => var,
        Err(_) => default.to_string(),
    }
}

fn get_repo(payload: &JsonValue) -> Result<Repo,ActionsError> {
    // We try to get it from the environment variable
    if let Ok(repository) = env::var("GITHUB_REPOSITORY") {
        let owner_repo = repository.split("/").collect::<Vec<&str>>();

        return Ok(Repo {
            owner: owner_repo[0].to_string(),
            repo: owner_repo[1].to_string(),
        });
    }

    let owner = payload["repository"]["login"]["login"].clone();
    let repo = payload["repository"]["name"].clone();

    return if owner.is_null() || repo.is_null() {
        Err(ActionsError::Context("context.repo requires a GITHUB_REPOSITORY environment variable like 'owner/repo'".to_string()))
    } else {
        Ok(Repo {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }
}
