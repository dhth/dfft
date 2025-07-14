# Contributing

## Audio Files

Audio files are located in `src/assets/` and are embedded into the binary at
compile time:
- `create.wav` - File creation sound
- `modify.wav` - File modification sound
- `remove.wav` - File removal sound
- `error.wav` - Error notification sound

### Converting and Optimizing Audio Files

If you want to add custom audio files, use these FFmpeg commands:

**Convert MP3 to WAV:**

```bash
ffmpeg -i input.mp3 output.wav
```

**Convert MP3 to WAV with silence trimming:**

```bash
ffmpeg \
    -i input.mp3 \
    -af "silenceremove=start_periods=1:start_silence=0.05:start_threshold=-50dB,areverse,silenceremove=start_periods=1:start_silence=0.05:start_threshold=-50dB,areverse" \
    output.wav -y
```

Explanation of the silence trimming command:
- Removes silence longer than 0.05 seconds from the start and end
- Uses -50dB as the silence threshold
- Significantly reduces file size while preserving audio quality
- WAV format provides better compatibility with the rodio audio library
