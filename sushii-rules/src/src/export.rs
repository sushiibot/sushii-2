mod model;

fn main() {
    let schema = schemars::schema_for!(model::Rule);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
