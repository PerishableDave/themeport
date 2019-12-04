use std::io::BufReader;
use std::io::Read;
use quick_xml::Reader;
use quick_xml::events::Event;

pub fn parse<R: Read>(buf: BufReader<R>) {
    let mut parser = Reader::from_reader(buf);
    let mut event_buf = Vec::new();
    loop {
        match parser.read_event(&mut event_buf) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8(e.name().to_vec()).unwrap();
                println!("Tag: {}", name);
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at pos: {}", parser.buffer_position()),
            _ => (),
        }
    }
}
