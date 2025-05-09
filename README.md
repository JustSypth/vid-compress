# vid-compress  

  #### An application to compress videos with simple UI.
  #### Supported platforms: Linux, Windows

## Features
  All features listed below contribute in resulting smaller output sizes:
  - Set compression intensity with a CRF value
  - Set audio bitrate
  - Option to have the output be HEVC
  - Encoding presets

## Download
  [Download the latest release here](https://github.com/JustSypth/vid-compress/releases/latest) or by going to the [releases](https://github.com/JustSypth/vid-compress/releases/).

## Usage
  1. Unpack the downloaded release archive and execute vid-compress.
  2. Select the video by either clicking "Select" or by dragging a video file to the window.
  3. Adjust the CRF <i> (compression) </i> value if needed. (The default value should be enough)
  4. Click the "Start" button to begin.

#### Additional:
  **-Preset:** Controls the speed of video encoding.   
    Slower presets are recommended because they produce better results and smaller file sizes
    
  **-Audio Bitrate:** Sets the audio's bitrate.   
    Lowering the bitrate decreases the result size but also decreases audio quality.

  **-Hevc:** Transcodes the video into HEVC which reduces result size but also takes longer to finish.

## Building
  #### Requirements:
  - To compile the application you must have [rust-lang](https://www.rust-lang.org/) installed (cargo, rustc)

  #### 1. Compile the watchdog:
  1. Navigate into `./src-watchdog`
  4. Run `cargo build --release`
  5. Navigate into `./target/release`
  6. Move `vid-compress-watchdog` to `/vid-compress/bin/` and rename it into `vid-compress-ffmpeg`
  
  #### 2. Download pre-compiled static ffmpeg build:
  1. Download the binaries here: [FFmpeg Linux](https://johnvansickle.com/ffmpeg/) | [FFmpeg Windows](https://www.gyan.dev/ffmpeg/builds/)
  2. Move the ffmpeg binary into `/vid-compress/bin/`

  (In case you don't want to download these pre-compiled builds then you can just compile your own.)

  #### 3. Compile the app:
  1. Navigate into `./src-tauri`
  2. Run `cargo build --release`
  3. The compiled app will be located in `./target/release/`
