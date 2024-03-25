use gstreamer::{element_factory::ElementBuilder, prelude::*, ElementFactory, Pipeline};
static _FILE_PATH: &str = "media/italo_disco.ogg";

fn main() {
    gstreamer::init().expect("Could not initialize GStreamer");

    let pipeline = Pipeline::with_name("audio-player");
    let source = make_element("filesrc", "file-source")
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
}

fn make_element<'a>(fact_name: &'a str, elm_name: &'a str) -> ElementBuilder<'a> {
    ElementFactory::make(fact_name).name(elm_name)
}
