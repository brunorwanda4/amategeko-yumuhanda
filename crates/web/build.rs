use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=QUESTION_SET");
    println!("cargo:rerun-if-env-changed=CARGO_WEB_BUILD");
    println!("cargo:rerun-if-changed=../../website/content/samples/questions.json");
    println!("cargo:rerun-if-changed=../../assets/questions.json");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("web_questions.json");

    let question_set = env::var("QUESTION_SET").unwrap_or_else(|_| "samples".to_string());
    let is_strict_build = env::var("CARGO_WEB_BUILD").is_ok();

    if question_set == "full" {
        let full_path = Path::new("../../assets/questions.json");
        if !full_path.exists() {
            panic!("assets/questions.json not found for QUESTION_SET=full");
        }
        let content = fs::read_to_string(full_path).expect("Failed to read assets/questions.json");
        fs::write(&dest_path, content).expect("Failed to write web_questions.json");
    } else {
        let samples_path = Path::new("../../website/content/samples/questions.json");
        let mut valid = false;
        let mut content = String::new();

        if samples_path.exists() {
            if let Ok(c) = fs::read_to_string(samples_path) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&c) {
                    if let Some(arr) = val.as_array() {
                        if arr.len() >= 25 {
                            valid = true;
                            content = c;
                        }
                    }
                }
            }
        }

        if valid {
            fs::write(&dest_path, content).expect("Failed to write web_questions.json");
        } else if is_strict_build {
            panic!("website/content/samples/questions.json is missing or has fewer than 25 questions. Never invent questions.");
        } else {
            println!("cargo:warning=website/content/samples/questions.json is missing or < 25 questions; using empty bank for cargo check");
            fs::write(&dest_path, "[]").expect("Failed to write web_questions.json");
        }
    }
}
