use std::{collections::HashMap, io, thread, time::Duration};

use gstreamer::{
    event, glib::List, prelude::*, ClockTime, Device, DeviceMonitor, Element, ElementFactory,
    MessageView, Pipeline, State,
};

pub fn record_mic() {
    init_gstreamer();
    let monitor = DeviceMonitor::new();

    monitor.add_filter(Some("Audio/Source"), None);
    monitor.start().expect("Monitor could not start");

    println!("Probing for devices...");

    let devices = &monitor.devices();
    let chosen_device = choose_device(devices);
    monitor.stop();
    println!("Chosen for recording: {}\n", chosen_device.display_name());

    let pipeline = Pipeline::with_name("pipeline");
    let src = chosen_device
        .create_element(Some("source"))
        .expect("Could not create element");
    let dec = ElementFactory::make("decodebin")
        .name("decoder")
        .build()
        .expect("Could not create decoder");

    pipeline
        .add_many([&src, &dec])
        .expect("Add many src and dec");

    Element::link_many([&src, &dec]).expect("Link src and dec");

    let pipeline_weak = pipeline.downgrade();

    dec.connect_pad_added(move |_, src_pad| {
        let Some(pipeline) = pipeline_weak.upgrade() else {
            eprintln!("Could not upgrade pipeline");
            return;
        };
        let is_audio = src_pad
            .current_caps()
            .and_then(|caps| caps.structure(0).map(|s| s.name().starts_with("audio/")));
        let is_audio = match is_audio {
            None => {
                eprintln!("Failed to get media type");
                return;
            }
            Some(is_actually_audio) => {
                if !is_actually_audio {
                    return;
                }

                is_actually_audio
            }
        };

        println!("Got audio: {is_audio}");

        let queue = ElementFactory::make("queue").build().expect("Queue");
        let convert = ElementFactory::make("audioconvert")
            .build()
            .expect("Audioconvert");
        let encoder = ElementFactory::make("vorbisenc").build().expect("Encoder");
        let mux = ElementFactory::make("oggmux").build().expect("Mux");
        let sink = ElementFactory::make("filesink")
            .property_from_str("location", "recording.ogg")
            .build()
            .expect("Filesink");
        let elements = &[&queue, &convert, &encoder, &mux, &sink];

        pipeline.add_many(elements).expect("Add queue, conv, sink");
        Element::link_many(elements).expect("Link queue, conv, sink");

        for e in elements {
            e.sync_state_with_parent().expect("Could not sync state");
        }

        let sink_pad = queue.static_pad("sink").expect("Queue has no sinkpad");
        src_pad
            .link(&sink_pad)
            .expect("Cannot link decode to queue");
    });

    pipeline.set_state(State::Playing).expect("Set playing");

    let bus = pipeline.bus().expect("Bus creation");
    let weak_pipe = pipeline.downgrade();

    thread::spawn(move || {
        println!("Recording will stop in 10 seconds");
        thread::sleep(Duration::from_secs(10));
        let pipeline = weak_pipe.upgrade().expect("No upgrade");
        let stop_event = event::Eos::new();

        pipeline.send_event(stop_event);
    });

    for msg in bus.iter_timed(ClockTime::NONE) {
        match msg.view() {
            MessageView::Warning(warn) => {
                println!("{warn}");
            }
            MessageView::Eos(_) => break,
            MessageView::Error(err) => {
                eprintln!("{err}");
            }
            _ => (),
        };
    }

    pipeline.set_state(State::Null).expect("Setting to null");
}

fn init_gstreamer() {
    gstreamer::init().expect("Could not init GStreamer");
}

fn choose_device(devices: &List<Device>) -> &Device {
    println!("Available audio devices: ");
    let mut map: HashMap<usize, &Device> = HashMap::new();

    for (i, device) in devices.into_iter().enumerate() {
        let nr = i + 1;
        println!("\t{nr}. {}", device.display_name());
        map.insert(i, &device);
    }

    println!("Choose device number: ");
    let selected_device_idx: usize;

    loop {
        let mut chosen = String::new();

        io::stdin().read_line(&mut chosen).unwrap();

        let selected_nr: usize = chosen.trim().parse().expect("Could not parse the choice");
        selected_device_idx = selected_nr - 1;
        break;
    }

    let (_, d) = map
        .get_key_value(&selected_device_idx)
        .expect("Provided device not found");

    d
}
