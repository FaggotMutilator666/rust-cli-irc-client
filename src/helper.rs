extern crate term_painter;

use parser::Message;
use color_string::*;

use self::term_painter::Color::*;

pub fn get_privmsg_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![color(mess.prefix.data, BrightRed),
				plain(" | "),
				color(mess.content, Cyan)
				])
}
pub fn get_notice_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![plain("!@| "),
				color(mess.content, Cyan),
				])
}
pub fn get_mode_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![plain("!m| "),
				color(mess.prefix.data, BrightRed),
				plain(" -> "),
				color(mess.content, Cyan)
				])
}
pub fn get_quit_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![plain("!q| "),
				color(mess.prefix.data, BrightRed),
				plain(" -- "),
				color(mess.content, Cyan)
				])
}
pub fn get_join_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![plain("!j| "),
				color(mess.prefix.data, BrightRed),
				])
}
pub fn get_nick_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![
        plain("!n| "),
        color(mess.prefix.data, BrightRed),
        plain(" -> "),
        color(mess.content, BrightRed),
        ])
}
pub fn get_misc_fmt(mess: Message) -> ColorString {
    ColorString::new(vec![plain("!@["),
				color(mess.command.data, Blue),
				plain("]| "),
				color(mess.content, Cyan)
				])
}


//FINISH THIS
pub fn parse_local(s: String) -> String {
    String::new()
}

pub fn is_ping(s: &String) -> bool {
    &s[0..4] == "PING"
}

pub fn ping_response(s: String) -> String {
    format!("PONG :{}", &s[6..])
}
