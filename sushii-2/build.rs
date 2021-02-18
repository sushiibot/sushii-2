use vergen::ConstantsFlags;
use vergen::gen;

fn main() {
    let flags = ConstantsFlags::all();

    // Generate the 'cargo:' instruction output
    gen(flags).expect("Unable to generate the cargo keys!");
}
