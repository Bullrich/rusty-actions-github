use actions_github::core::set_output;
use actions_github::error::ActionsError;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = ["test", "value", "very long string"])]
fn set_output_benchmark(value: &str) {
    match set_output("name", value) {
        Ok(_) => {}
        Err(err) => panic!("{:#?}", err)
    }
}