use vergen::gen;
use vergen::ConstantsFlags;

fn main() {
    let flags = ConstantsFlags::all();

    // Generate the 'cargo:' instruction output
    gen(flags).expect("Unable to generate the cargo keys!");
}
