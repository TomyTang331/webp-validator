use webp_validator::validate_webp;

fn main() {
    println!("webp validator - usage example\n");
    println!("tip: run 'cargo test' to execute full test suite\n");

    let test_file = "images/static.webp";

    println!("validating: {}", test_file);
    match validate_webp(test_file) {
        Ok(info) => {
            println!("valid webp file");
            println!("  dimensions: {}x{}", info.width, info.height);
            println!("  has alpha: {}", if info.has_alpha { "yes" } else { "no" });
            println!(
                "  is animated: {}",
                if info.is_animated { "yes" } else { "no" }
            );
            if info.is_animated {
                println!("  frames: {}", info.num_frames);
            }
        }
        Err(e) => {
            println!("validation failed: {}", e);
        }
    }

    println!("\nrun 'cargo test -- --nocapture' to see detailed test output");
}
