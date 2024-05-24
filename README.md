# Rusty-Actions

A rust translation of [@actions/github](https://www.npmjs.com/package/@actions/github).

Find the [documentation here](https://docs.rs/actions-github).

**Work in progress**: This library is being developed.

## Work in progress
- [x] Context object
- [ ] get_input method
- [ ] set_output method
- [ ] logging methods

## Installation

`cargo add actions-github`

## Usage

```rust
use actions_github::context::get_context;

let data = get_context().unwrap();
println!("Event is {}", data.event_name);
```
