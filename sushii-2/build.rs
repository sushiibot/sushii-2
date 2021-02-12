use vergen::vergen;
use vergen::Config;

fn main() {
    let config = Config::default();

    // Generate the 'cargo:' instruction output
    vergen(config).expect("Unable to generate the cargo keys!");
}
