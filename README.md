# vid-compress  

  #### A simple application to compress videos with simple UI.
  #### Supported platforms: Linux, Windows

## Features
  All features listed below contribute in resulting smaller output sizes:
  - Set compression intensity with a CRF value
  - Set audio bitrate as well
  - Option to have the output be HEVC

## Download
  [Download the latest release here](https://github.com/JustSypth/vid-compress/releases/latest) or by downloading the latest release from the [releases](https://github.com/JustSypth/vid-compress/releases/).

## Usage
  1. Unpack the downloaded release archive and execute vid-compress.
  2. Select the video in the application by either clicking the "Select" button or by pasting the video's path inside the text box.
  3. Adjust the CRF <i> (compression) </i> value if needed. (The default value should be enough)
  4. Click the "Start" button to begin.

#### Additional:
  **-Preset:** Controls the speed of video encoding.   
    Slower presets are recommended because they produce better results and smaller file sizes
    
  **-Audio Bitrate:** Sets the audio's bitrate.   
    Lowering the bitrate decreases the result size but also decreases audio quality.

  **-Hevc:** Transcodes the video into HEVC which reduces result size but also takes longer to finish.
