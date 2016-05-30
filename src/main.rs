//todo:
//add Admin/Voiced/other name coloring.
//shell.

mod irc;
mod parser;
mod helper;
mod color_string;
mod tcp;

use std::thread;
use std::net::TcpStream;
use std::io::Write;


fn main() {
    let mut stream = TcpStream::connect("irc.sorcery.net:6667").unwrap();
    let listener = irc::IrcListener::new(Box::new(stream.try_clone().unwrap()));
    thread::spawn(move || {
        let server = tcp::Server::new("localhost:300");
        let _ = stream.write(b"NICK AryChr\r\nUSER AryChr test test :Windows 3.1\r\n");
        server.start(|s: String| {
            let _ = stream.write(s.as_bytes());
            println!("[X]");
        });
    });
    listener.start(on_line);
}

fn on_line(s: String, last_prefix: &mut String){
    let mut mess = match parser::to_message(&s) {
        Ok(m) => m,
        Err(s) => {
            println!("{}", s);
            return;
        }
    };
    let pref = mess.get_prefix_clone();
    if &*pref == &*last_prefix {
        mess.set_prefix("~");
    }else {
        last_prefix.clear();
        last_prefix.push_str(&pref);
    }

    print!("{}", parser::parse_message(mess));

}
