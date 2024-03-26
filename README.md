# OGG/Vorbis Audio Player

Simple CLI program utilising GStreamer to play or record an OGG/Vorbis audio file. Made for practicing Rust and learning the GStreamer framework.

## Run

It requires GStreamer to be installed on the system to run it through Cargo. The binary has only been tested on MacOS.

Can run in two modes; **play** or **record**. Playing will play the specified OGG file. Recording will display a list of available audio sources and record from the selected source. The recording will be saved to a file named `recording.ogg` in the current directory.

**For the time being, the recording mode will stop recording after 10 seconds.**

### Cargo

#### Play a file

```bash
$ cargo run -- <path-to-ogg-file>
```

#### Record from microphone

```bash
$ cargo run -- record
```

### Binary

#### Play a file

```bash
$ ./rusty-ogg <path-to-ogg-file>
```

#### Record from microphone

```bash
$ ./rusty-ogg record
```

### Example usage

```bash
$ cargo run -- record
# or
$ ./rusty-ogg record

Available audio sources:
    1. Built-in Microphone
    2. MacBook Pro Microphone
Choose device number:
$ 2
Chosen for recording: MacBook Pro Microphone
# after 10 seconds
Sending EOS...
$ ls
recording.ogg
```
