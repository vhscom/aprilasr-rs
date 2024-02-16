#!/usr/bin/env -S sh -euo pipefail

# Check if ffmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "Error: ffmpeg is not installed. Please install ffmpeg before running this script."
    exit 1
fi

curl -O https://www.wavsource.com/snds_2020-10-01_3728627494378403/movies/rain_man/rain_man_leaving.wav
ffmpeg -i ./rain_man_leaving.wav -af "aformat=sample_fmts=s16:channel_layouts=mono:sample_rates=16000" mono_16bit16khz.wav
rm rain_man_leaving.wav
