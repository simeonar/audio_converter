use std::env;
use std::fs;
use std::process::Command;

fn main() {
    // Get current directory
    let dir_path = env::current_dir().unwrap();
    let entries = fs::read_dir(dir_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let input_path = entry.path();

        // Check each file if it's .m4a file
        if input_path.is_file() && input_path.extension().unwrap() == "m4a" {
            // Prepare the output .mp3 file path
            let output_path = input_path.with_extension("mp3");

            // Call ffmpeg to convert the file
            let _ = Command::new("ffmpeg")
                .arg("-i")
                .arg(&input_path)
                .arg("-vn")
                .arg("-ar")
                .arg("44100")
                .arg("-ac")
                .arg("2")
                .arg("-b:a")
                .arg("192k")
                .arg(output_path.clone()) // Clone output_path here
                .output()
                .expect("Failed to execute command");

            // success message
            println!("Converted {:?}", output_path);
        }
    }
}