# Rusty-Actions

A rust translation of [@actions/github](https://www.npmjs.com/package/@actions/github).

![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)

Find the [documentation here](https://docs.rs/actions-github).

![Crates.io Total Downloads](https://img.shields.io/crates/d/actions-github)

## Supported features

- `get_input`
- `set_output`
- `get_context`: Returns a hydrated context object
- logging
  - `debug`
  - `info`
  - `notice` (with annotations)
  - `warn` (with annotations)
  - `error` (with annotations)
  - `is_debug`

## Installation

`cargo add actions-github`

Latest version available is [![latest version](https://img.shields.io/crates/v/actions-github)](https://crates.io/crates/actions-github)

## Usage

```rust,ignore
// Obtain the context from the action worker
use actions_github::context::get_context;
use actions_github::logger;

logger::info("Obtaining context");
let data = get_context().unwrap();

logger::debug(format!("Event is {}", data.event_name).as_str());

// Produce an output
set_output("is_pr", (ctx.event_name == "pull_request").to_string());
```

Works well with [`octocrab`](https://crates.io/crates/octocrab/):

```rust,ignore
use actions_github::core::{get_input, set_output};
use actions_github::context::get_context;
use octocrab::Octocrab;

let token = get_input("GITHUB_TOKEN").unwrap();

let crab = Octocrab::builder().personal_token(token).build();
octocrab::initialise(crab.unwrap());

let context = get_context();
let org = context.repo.owner;
let repo = context.repo.repo;

let pulls = octocrab::instance().pulls(owner, repo).list()

// Output how many PRs are in the repository
set_output("PRs", pulls.len().to_string());
```
