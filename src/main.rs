extern crate image;
extern crate rand;

use rand::Rng;
use std::ascii::AsciiExt;
use std::error;
use std::str;
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

use image::io::Reader;
use image::ColorType;

#[derive(Debug, Clone)]
struct DecodeError;

const INSTAGRAM_W: u32 = 1080;
const INSTAGRAM_H: u32 = 1350;
const INSTAGRAM_PIXELS: u32 = 1458000;
const INSTAGRAM_CHARS: usize = 4374000;
const INSTAGRAM_WORDS: u32 = 930638; // avg 4.7 chars

// https://github.com/amephraim/nlp/blob/master/texts/J.%20K.%20Rowling%20-%20Harry%20Potter%201%20-%20Sorcerer's%20Stone.txt

fn encode_fixed_size(str: &str, path: &str, w: u32, h: u32) {
    let str_as_bytes = str.as_bytes();

    image::save_buffer(path, str_as_bytes, w, h, ColorType::Rgb8).unwrap();
}

fn encode(str: &str, path: &str) {
    let str_as_bytes = str.as_bytes();

    println!("bytes len: {}", str_as_bytes.len());
    let root = ((str_as_bytes.len() as f64) / 3.0).sqrt().ceil() as u32;
    println!("root: {}", root);
    let pixels = root.pow(2);
    println!("pixels: {}", pixels);
    let remainder = pixels * 3 - (str_as_bytes.len() as u32);
    println!("remainder: {}", remainder);
    let pad = (0..remainder).map(|_| " ").collect::<String>();
    let padded = format!("{str}{pad}");
    let str = padded.as_bytes();
    image::save_buffer(path, str, root, root, ColorType::Rgb8).unwrap();
}

fn decode(path: &str) -> Result<String> {
    let img = Reader::open(path)?;
    let decoded_img = img.decode()?;
    let rgb_vec = decoded_img.to_rgb8().into_raw();
    let string = String::from_utf8(rgb_vec)?;
    Ok(string)
}

fn main() {
    let harry: Vec<u8> = vec![0; INSTAGRAM_CHARS]
        .iter()
        .map(|_| {
            let mut r = 255;
            while r > 127 {
                r = rand::thread_rng().gen();
            }
            r
        })
        .collect::<Vec<u8>>();

    let harry_str = str::from_utf8(&harry).expect("Conversion to str error");

    encode_fixed_size(harry_str, "harry.bmp", INSTAGRAM_W, INSTAGRAM_H)

    // let decoded = decode("oscar.jpg").unwrap();
    // println!("{}", decoded);
    // let test = "åäö";
    // encode(test, "test.png");
    // let decoded = decode("test.png").unwrap();
    // println!("{}", decoded)
}
