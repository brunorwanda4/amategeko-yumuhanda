use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=generated/questions.json");
    println!("cargo:rerun-if-changed=generated/questions_en.json");

    let rw_path = Path::new("generated/questions.json");
    let en_path = Path::new("generated/questions_en.json");

    if !rw_path.exists() || !en_path.exists() {
        panic!("Missing generated question bank: crates/web/generated/questions.json and questions_en.json are required. Please run 'bun website/scripts/prepare-questions.ts' before building crates/web.");
    }
}
