use std::sync::{Arc, Mutex};
use crate::capture::{ImageIterator, Frame};
use actix_web::web;
use image::{ImageBuffer, Rgb};

#[derive(Clone)]
pub struct CameraState {
    camera: Arc<Mutex<ImageIterator>>,
    last: Arc<Mutex<ImageBuffer<Rgb<u8>, Frame>>>
}

impl CameraState {
    pub async fn grab_frame(self) -> ImageBuffer<image::Rgb<u8>, Frame> {
        let img: ImageBuffer<Rgb<u8>, Frame> = web::block(move || {
            let mut cam = self.camera.lock().unwrap();
            match cam.next() {
                Some(x) => Ok(x),
                None => Err(()),
            }
        }).await.unwrap();

        img
    }
}