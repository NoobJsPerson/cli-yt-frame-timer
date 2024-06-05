# cli-yt-frame-timer
A CLI tool to retime YouTube Speedruns
### How to use
1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository
4. Go to the Youtube video you want to retime
5. Find the start and end frame of the run (use , and . to go frame by frame)
6. When you find the start or end frame, right click on the video and click "Copy Debug Info" and paste it into a text file (one for the start frame and one for the end frame)
4. While in the cloned repository, run `cargo run <PATH_FILE_PATH> <END_FILE_PATH> <FRAMERATE>`