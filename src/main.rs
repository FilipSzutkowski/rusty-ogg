use std::env::args;

use gstreamer::{
    element_factory::ElementBuilder, prelude::*, ClockTime, Element, ElementFactory, MessageView,
    Pipeline, State,
};
use rusty_ogg::record_mic;

fn main() {
    let mut args = args();

    // Skip first argument
    args.next();

    let first_arg = args.next().expect("Run with 'record' as first argument for microphone recording, or specify a file path to playback an 'ogg' file.");

    if first_arg == "record" {
        record_mic();
        return;
    }

    let file_path = first_arg;

    println!("{file_path}");
    gstreamer::init().expect("Could not initialize GStreamer");

    let pipeline = Pipeline::with_name("audio-player");
    let source = make_element("filesrc", "file-source")
        .property_from_str("location", &file_path)
        .build()
        .expect("Could not build file source element");
    let demuxer = make_element("oggdemux", "ogg-demuxer")
        .build()
        .expect("Could not build demuxer");
    let decoder = make_element("vorbisdec", "vorbis-decoder")
        .build()
        .expect("Could not build decoder");
    let conv = make_element("audioconvert", "converter")
        .build()
        .expect("Could not build file source element");
    let sink = make_element("autoaudiosink", "audio-output")
        .build()
        .expect("Could not build file source element");

    pipeline
        .add_many([&source, &demuxer, &decoder, &conv, &sink])
        .expect("Could not add elements to pipeline");

    // file source -> ogg-demuxer
    source
        .link(&demuxer)
        .expect("Could not link file source to demuxer");

    // Decoder -> converter -> output
    Element::link_many([&decoder, &conv, &sink])
        .expect("Could not link decoder, converter and sink");

    demuxer.connect_pad_added(move |_, pad| {
        //
        println!("Dynamic pad created. Linking demuxer/decoder");
        let sink_pad = decoder
            .static_pad("sink")
            .expect("Could not get decoder's sink pad");

        pad.link(&sink_pad)
            .expect("Could not link demuxer's pad to decoder's sink");
    });

    pipeline
        .set_state(State::Playing)
        .expect("Could not start playing");

    let bus = pipeline.bus().expect("Could not create bus");

    println!("Running...");
    // Wait for events from bus
    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(_) => {
                println!("End of stream, bye");
                break;
            }
            MessageView::Error(err) => {
                eprintln!("Error: {err}");
                break;
            }
            _ => (),
        }
    }
    // Out of waiting for events, finish up.

    println!("Stopping playback");
    pipeline
        .set_state(State::Null)
        .expect("Could not stop playback");
}

fn make_element<'a>(fact_name: &'a str, elm_name: &'a str) -> ElementBuilder<'a> {
    ElementFactory::make(fact_name).name(elm_name)
}
