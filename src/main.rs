use std::env;
use std::fs;
use std::process::Command;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use indicatif::ProgressBar;
use ctrlc;

fn main() {
    // Get current directory
    let dir_path = env::current_dir().unwrap();
    let entries = fs::read_dir(dir_path).unwrap();

    // Handle CTRL+C to stop the operation cleanly
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    for entry in entries {
        let entry = entry.unwrap();
        let input_path = entry.path();

        // Check each file if it's .m4a file
        if input_path.is_file() && input_path.extension().unwrap() == "m4a" {
            // Prepare the output .mp3 file path
            let output_path = input_path.with_extension("mp3");
            let output_path_clone = output_path.clone();

            // Print processing message
            println!("Processing {:?}", input_path);

            // Start a loading indicator
            let pb = ProgressBar::new_spinner();
            pb.enable_steady_tick(100);

            // Create a channel to communicate between threads
            let (tx, rx) = mpsc::channel();

            // Run the ffmpeg command in a different thread
            thread::spawn(move || {
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
                    .arg(output_path_clone)
                    .output()
                    .expect("Failed to execute command");
                
                // Send a signal to the main thread that the process is finished
                tx.send(()).unwrap();
            });

            // Spin until the thread finishes or the user hits CTRL + C
            loop {
                if !running.load(Ordering::SeqCst) {
                    pb.finish_with_message("stopped early");
                    return;
                }

                // Check if the ffmpeg thread has finished
                if let Ok(_) = rx.try_recv() {
                    pb.finish_with_message("done");
                    break;
                }
                
                // Keep the main thread busy, delay is needed to limit CPU usage
                thread::sleep(std::time::Duration::from_millis(50));
                pb.tick();
            }

            // Print success message
            println!("Converted {:?}", output_path);
        }
    }
}