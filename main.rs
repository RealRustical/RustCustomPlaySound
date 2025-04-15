use std::{fs::File, path};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::io;

fn main() {
   let mut idk = String::new();
   io::stdin().read_line(&mut idk).expect("Failed to read line");
  
   let mut cool = idk.trim();
   let (_stream, stream_handle) = OutputStream::try_default().unwrap();
   let file = BufReader::new(File::open(cool).unwrap());
   let source = Decoder::new(file).unwrap();
   stream_handle.play_raw(source.convert_samples());
  
   std::thread::sleep(std::time::Duration::from_secs(15));
}
