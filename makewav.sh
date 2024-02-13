#!/usr/bin/env -S sh -euo pipefail
curl -O https://www.wavsource.com/snds_2020-10-01_3728627494378403/movies/rain_man/rain_man_leaving.wav
ffmpeg -i ./rain_man_leaving.wav -af "aformat=sample_fmts=s16:channel_layouts=mono:sample_rates=16000" mono_16bit16khz.wav
rm rain_man_leaving.wav
