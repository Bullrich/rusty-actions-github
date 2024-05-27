# Rusty-Actions

A rust translation of [@actions/github](https://www.npmjs.com/package/@actions/github).

![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)

Find the [documentation here](https://docs.rs/actions-github).

[![latest version](https://img.shields.io/crates/v/actions-github)](https://crates.io/crates/actions-github)
![Crates.io Total Downloads](https://img.shields.io/crates/d/actions-github)

**Work in progress**: This library is being developed.

## Work in progress

- [x] Context object
- [x] get_input method
- [x] set_output method
- [ ] logging methods

## Installation

`cargo add actions-github`

## Usage

```rust
use actions_github::context::get_context;

let data = get_context().unwrap();
println!("Event is {}", data.event_name);
```

Works well with [`octocrab`](https://crates.io/crates/octocrab/):

```rust
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
set_output("PRs", pulls.len().to_string);
```
