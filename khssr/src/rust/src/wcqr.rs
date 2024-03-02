use bardecoder;
use nokhwa::{
    Camera,
    utils::{
        CameraIndex, 
        RequestedFormat, 
        RequestedFormatType
    },
    pixel_format::RgbAFormat
};
use std::{thread, time::Duration};

pub fn scan() -> String {
    //get camera
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate)
    ).unwrap();

    //loop until valid scan
    'res_loop: loop {

        let scan = loop {
            if let Ok(buf) = cam.frame() {
                if let Ok(dec) = buf.decode_image::<RgbAFormat>() {
                    break dec;
                }
            } thread::sleep(Duration::from_secs(5));
        };

        let mut scn = Vec::new();
        for thing in bardecoder::default_decoder().decode(&scan) { 
            if let Ok(el) = thing {scn.push(el);}
            else {thread::sleep(Duration::from_secs(5)); continue 'res_loop;}
        }
        if scn.len() != 0 {break scn;}

    }[0] //There is absolutely no circumstance in which this is longer than 1
}