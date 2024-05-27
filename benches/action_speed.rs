use actions_github::core::{get_input, set_output};
use actions_github::logger::{debug_log, error_log, info, is_debug, notice_log, warn_log};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = ["test", "value", "very long string"])]
fn set_output_benchmark(value: &str) {
    match set_output("name", value) {
        Ok(_) => {}
        Err(err) => panic!("{:#?}", err),
    }
}

#[divan::bench(args = ["name", "place", "owner"])]
fn get_input_benchmark(value: &str) {
    match get_input(value) {
        Ok(_) => panic!("{} should not be available", value),
        Err(_) => {}
    }
}

#[divan::bench]
fn get_debug_benchmark() {
    if !is_debug() {
        panic!("It should be debug")
    }
}

#[divan::bench(args = ["hi", "example", "long words here"])]
fn log_benchmark(msg: &str) {
    debug_log(msg);
    info(msg);
    warn_log(msg);
    error_log(msg);
    notice_log(msg);
}
