# Audio Converter

A simple utility written in Rust to convert audio files from one format to another. This utility uses `ffmpeg` for conversion.

## Prerequisites

This utility requires `ffmpeg` to be installed and accessible in the system's `PATH`. If it isn't already installed, you can find installation instructions [here](https://ffmpeg.org/download.html).

## Usage

The script currently converts all `.m4a` files in the same directory to `.mp3` format.

1. Copy your `.m4a` files into the same directory as the compiled binary.
2. Run the binary using `cargo run --release`.

The program will automatically convert all `.m4a` files to `.mp3` format.

## Known Issues

The current implementation uses a spinner progress bar to indicate that the conversion process is still ongoing. However, we can't get the exact status of the conversion from `ffmpeg` to update the spinner progress due to the lacks of direct communication with the spawned `ffmpeg` process.

At the moment, the spinner stops when a file's conversion is completed, and the `CTRL + C` signal is correctly handled to stop the running operation, which is important for cleaning up and not leaving orphaned processes.

## Future Improvements

To provide a better user experience, future versions of this utility could include:

- Allowing the user to specify the input and output directories through command-line arguments.
- Converting different input formats, not only `.m4a`.
- Updating the progress bar based on `ffmpeg`'s conversion progress, this might require parsing `ffmpeg`'s log.

## Contributing

Any contributions are welcome. Feel free to fork the project, make your changes, and submit a pull request.