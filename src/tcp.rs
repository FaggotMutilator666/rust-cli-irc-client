use std::net::{TcpListener, TcpStream};
use std::io::Read;

pub struct Server {
    listener: TcpListener
}

//add match error catching
impl Server {
    pub fn new(l: &str) -> Server {
        Server { listener: match TcpListener::bind(l) {
            Ok(li) => li, Err(_) => panic!("[-]Cannot bind local server.")
        }}
    }

    pub fn start<F>(&self, func: F) where F : FnMut(String) {
        let mut closure = func;
        loop {
            for stream in self.listener.incoming() {
                match stream {
                    Ok(s) => Server::handle(s, &mut closure),
                    Err(_) => {}
                };
            }
        }
    }

    fn handle<F>(s: TcpStream, func: &mut F) where F : FnMut(String){
        let reader = TcpReader::new(s);
        for chunk in reader {
            for line in chunk {
                func(format!("{}{}", line, "\r\n"));
            }
        }

    }
}



pub struct TcpReader {
    stream: TcpStream,
    clean: String,
    vec: Vec<String>,
}

impl TcpReader {
    pub fn new(st: TcpStream) -> TcpReader {
        TcpReader{
            stream: st,
            clean: String::new(),
            vec: Vec::new()
        }
    }
}

impl Iterator for TcpReader {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
		self.vec = Vec::new();
		let mut line;
		line = [0; 2048];
		let result = self.stream.read(&mut line);
		let response = String::from_utf8_lossy(&line);
		match result {
			Ok(_) => {
				for ch in response.chars() {
					if ch as u32 != 0 { self.clean.push(ch); }
					if ch == '\n' {
						let tmp = self.clean.clone();
						self.clean.clear();
						self.vec.push(tmp);
					}
				}
			},
			Err(_) => panic!("[-]Cannot read from server.")
		}
		return Some(self.vec.clone());
	}
}
