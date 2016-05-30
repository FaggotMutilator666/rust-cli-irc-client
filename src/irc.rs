use std::net::TcpStream;
use tcp::TcpReader;
use std::io::Write;
use helper;

pub struct IrcListener {
    stream: TcpStream,
    last_prefix: String
}

impl IrcListener {
    pub fn new(st: Box<TcpStream>) -> IrcListener {
        IrcListener { stream: *st, last_prefix: String::from("") }
    }

    pub fn start<F>(mut self, func: F) where F : Fn(String, &mut String) {
        let mut reader = TcpReader::new(self.stream.try_clone().unwrap());
        loop {
            match reader.next() {
                Some(s) => {
					for line in s {
                        if !helper::is_ping(&line) {
                            func(line, &mut self.last_prefix);
                        } else {
                            let _ = self.stream.write(helper::ping_response(line).as_bytes());
                        }
					}
				}
				None => { continue; }
            }
        }

    }

}
