# OGG/Vorbis Audio Player

Simple program utilising GStreamer to play an OGG/Vorbis audio file. Made for practicing Rust and learning the GStreamer framework.

## Run

It requires GStreamer to be installed on the system to run it through Cargo.

```bash
$ cargo run -- <path-to-ogg-file>
```

It will play the audio file to selected audio output and exit when it's done.
