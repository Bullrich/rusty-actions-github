use std::{env, fs, path::Path};

use json::JsonValue;

#[derive(Debug, Clone)]
pub struct Context {
    payload: JsonValue,
    event_name: String,
    sha: String,
    ref_: String,
    workflow: String,
    action: String,
    actor: String,
    job: String,
    run_attempt: u8,
    run_number: u8,
    run_id: u128,
    api_url: String,
    server_url: String,
    graphql_url: String,
    repo: Repo,
}

#[derive(Debug, Clone)]
struct Repo {
    owner: String,
    repo: String,
}

pub fn get_context() -> Context {
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

    let repo: Repo = get_repo(&payload);

    Context {
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
    }
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

fn get_repo(payload: &JsonValue) -> Repo {
    // We try to get it from the environment variable
    if let Ok(repository) = env::var("GITHUB_REPOSITORY") {
        let owner_repo = repository.split("/").collect::<Vec<&str>>();

        return Repo {
            owner: owner_repo[0].to_string(),
            repo: owner_repo[1].to_string(),
        };
    }

    let owner = payload["repository"]["login"]["login"].clone();
    let repo = payload["repository"]["name"].clone();

    if owner.is_null() || repo.is_null() {
        panic!("context.repo requires a GITHUB_REPOSITORY environment variable like 'owner/repo'")
    } else {
        return Repo {
            owner: owner.to_string(),
            repo: repo.to_string(),
        };
    }
}
