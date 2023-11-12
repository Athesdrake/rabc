pub mod abc;
pub mod error;
pub mod stream;
pub mod swf;

pub use abc::AbcFile;
pub use stream::{StreamReader, StreamWriter};
pub use swf::{Compression, Movie};

#[cfg(test)]
mod tests {
    use crate::{
        stream::StreamReader,
        stream::StreamWriter,
        swf::{
            tags::{Tag, TagID},
            Compression, Movie,
        },
    };
    use std::{fs::File, io::Read};

    #[test]
    fn it_works() {
        let now = std::time::Instant::now();
        let mut file = File::open("test.swf").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let mut stream = StreamReader::new(buf);
        let movie = Movie::read(&mut stream).unwrap();
        println!("Parsing took {}ms", now.elapsed().as_millis());

        assert_eq!(movie.version, 14);
        assert_eq!(movie.compression, Compression::None);
        assert_eq!(movie.file_length, 2828881);
        assert_eq!(movie.framecount, 1);
        assert_eq!(movie.framerate, 60.0);
        assert_eq!(movie.framesize.min.x, 0);
        assert_eq!(movie.framesize.min.y, 0);
        assert_eq!(movie.framesize.max.x, 16000);
        assert_eq!(movie.framesize.max.y, 12000);

        for tag in movie.tags {
            let tag_id: TagID = (&tag).into();
            println!("[{} id:0x{:0>2x}]", tag_id, tag.id());
            match tag {
                Tag::Metadata(meta) => {
                    println!("{}", meta.metadata);
                }
                Tag::DoABC(abc_tag) => {
                    let abc = &abc_tag.abcfile;
                    println!("{} lazy:{}", abc_tag.name, abc_tag.lazy);
                    println!("Abc {}.{}", abc.version.major, abc.version.minor);
                    println!("\tmethods: {}", abc.methods.len());
                    println!("\tclasses: {}", abc.classes.len());
                    println!("\tscripts: {}", abc.scripts.len());
                    println!("\tmetadatas: {}", abc.metadatas.len());
                }
                Tag::Unknown(unk) => {
                    println!("id:{}", unk.id);
                }
                _ => {}
            };
        }
    }

    #[test]
    fn test_write() {
        let mut file = File::open("test.swf").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let mut stream = StreamReader::new(buf);
        let mut movie = Movie::read(&mut stream).unwrap();

        movie.compression = Compression::Lzma;

        let now = std::time::Instant::now();
        let mut out_stream = StreamWriter::new(Vec::with_capacity(stream.len() as usize));
        movie.write(&mut out_stream).unwrap();
        println!("Writing took {}ms", now.elapsed().as_millis());
        let out_file = File::create("test_out.swf").unwrap();
        out_stream.to_file(out_file).unwrap();
        println!("Writing to disk took {}ms", now.elapsed().as_millis());

        let mut file = File::open("test_out.swf").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let mut stream = StreamReader::new(buf);
        let _movie = Movie::read(&mut stream).unwrap();
    }
}
