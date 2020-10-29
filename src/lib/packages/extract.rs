use std::fs::File;
use std::path::{Path, PathBuf};

use ruzstd::{frame_decoder, StreamingDecoder};
use std::io;
use std::io::Read;
use zstd::block::decompress;
use zstd::Decoder;

pub async fn extract(path: PathBuf) -> std::io::Result<bool> {
    let input_file = File::open(path)?;
    let mut decoder = Decoder::new(input_file).expect("Error").finish();
    /*let mut output_file = File::create(PathBuf::from("./temp/uncompress/test")).expect("Error");
     let mut string = String::new();
    decoder.read_to_string(&mut string);*/

    println!("{:?}", decompress(decoder.buffer(), decoder.buffer().len()));
    Ok(true)
}
