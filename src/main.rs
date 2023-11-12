use rabc::{Movie, StreamReader};
use std::{fs::File, io::Read};

const GIGA: f64 = 1024.0 * 1024.0 * 1024.0;

fn oldmain() {
    let now: std::time::Instant = std::time::Instant::now();
    let mut file = File::open("tfm_packed.swf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let took = now.elapsed();
    println!("Read file: {}µs", took.as_micros());
    println!(
        "Read speed: {:.2} GB/s",
        (buf.len() as f64) / (took.as_secs_f64() * GIGA)
    );

    let mut stream = StreamReader::new(buf);
    let movie = Movie::read(&mut stream).unwrap();
    println!("Parsing took {}ms", now.elapsed().as_millis());
    println!("fps: {}", movie.framerate);
}

fn main() {
    oldmain();
    let mut file = File::open("tfm_packed.swf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut stream = StreamReader::new(buf);
    let _movie = Movie::read(&mut stream).unwrap();

    let mut file = File::open("test_lzma.swf").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let mut stream = StreamReader::new(buf);
    let _movie = Movie::read(&mut stream).unwrap();
}
