#[macro_use]
extern crate log;

use warp::Filter;
use std::sync::Arc;
use tokio::sync::{Mutex, watch};

use camera_core::{capture, image};
use std::env::args;

use camera_core::capture::Frame;
use image::{ImageBuffer, Rgb, ColorType};
use std::borrow::Cow;

type Im = ImageBuffer<Rgb<u8>, Frame>;

#[tokio::main]
async fn main() {
    let args = args().collect::<Vec<String>>();

    let camera_idx: u32 = args[1].parse().unwrap();
    let fps: f64 = args[2].parse().unwrap();
    let width: u32 = args[3].parse().unwrap();
    let height: u32 = args[4].parse().unwrap();
    // let img_fmt = &args[3];
    let quality: u8 = args[5].parse().unwrap();


    let mut capture = capture::create(camera_idx).unwrap()
        .fps(fps).unwrap()
        .resolution(width, height).unwrap()
        .start().unwrap();

    let cap: Im = capture.next().unwrap();

    let mut buffer = Vec::new();
    let mut encoder = image::jpeg::JPEGEncoder::new_with_quality(&mut buffer, quality);
    encoder.encode(&cap, cap.width(), cap.height(), ColorType::Rgb8).unwrap();

    let (wtx, wrx) = watch::channel(Cow::from(buffer));

    tokio::task::spawn_blocking(move || {
        let mut buffer = Cow::from(Vec::new());
        loop {
            let mut encoder = image::jpeg::JPEGEncoder::new_with_quality(buffer.to_mut(), quality);
            encoder.encode(&cap, cap.width(), cap.height(), ColorType::Rgb8).unwrap();

            if let Err(e) = wtx.broadcast(buffer.clone()) {
                error!("{}", e);
            };

            buffer.to_mut().clear();
        }
    });

    let static_img = warp::path("static")
        .and(warp::get())
        .map(|| {
            "todo"
        });

    let camera = warp::path("camera").and(static_img);

    warp::serve(camera)
        .run(([127, 0, 0, 1], 3030))
        .await;
}