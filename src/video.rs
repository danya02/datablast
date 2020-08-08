use image::{RgbImage, load_from_memory};
use image::ImageError;
use std::process::{Command, Stdio, Child};
use std::collections::HashMap;
use std::io::Read;

fn launch(cmd: &str, args: &[&str]) -> Child {
        Command::new(cmd).args(args).stdout(Stdio::piped()).spawn().expect("Failed to start command!")
}

fn exec_command(cmd: &str, args: &[&str]) -> Vec<u8> {
    let output = launch(cmd, args).wait_with_output().expect(&format!("Failed to get output from command {:?}", cmd));
    output.stdout
}

fn bytes_to_img(data: Vec<u8>) -> Result<RgbImage, ImageError> {
    let dyn_img = load_from_memory(&data[..])?;
    Ok(dyn_img.to_rgb())
}

struct Video {
    source: String,
    frames: HashMap<usize, RgbImage>,
    framerate_numerator: Option<u32>,
    framerate_denominator: Option<u32>,
}

impl Video {
    fn fetch_framerate(&mut self) {
        let result = exec_command(&"ffprobe", &["-v", "quiet", "-select_streams v:0", "-show_entries", "stream=avg_frame_rate", "-of", "default=noprint_wrappers=1:nokey=1", &self.source]);
        let result_components = std::str::from_utf8(&result).expect("ffprobe command not valid UTF-8??").split("/").collect::<Vec<_>>();
        self.framerate_numerator = Some(result_components[0].parse().expect("Numerator not valid integer"));
        self.framerate_denominator = Some(result_components[0].parse().expect("Numerator not valid integer"));
    }
    fn get_timestamp(&mut self, frame_id: usize) -> String {
        if self.framerate_numerator.is_none() { self.fetch_framerate(); }
        // TODO: add timestamp formatting
        unimplemented!();
    }

    fn fetch_frame(&mut self, frame_id: usize) {
        unimplemented!();
        let cmd = launch("ffmpeg", &["-ss", &self.get_timestamp(frame_id), "-i", &self.source]);
        let img = bytes_to_img(Vec::new());
    }
}


#[cfg(test)]
mod tests {
    use crate::video::*;
    #[test]
    fn test_exec_cmd_works() {
        assert_eq!(exec_command(&"cat", &["test_data/plain_text.txt"]), b"Hello World!\n");
    }

    #[test]
    fn test_img_load_works() -> Result<(), ImageError> {
        bytes_to_img(exec_command(&"cat", &["test_data/image_load_test.png"]))?;
        Ok(())
    }
}
