use helper::*;
use color_string::*;

pub struct Message {
	pub prefix: Unit,
	pub command: Unit,
	pub content: String,
	pub to: Unit
}

pub struct Unit {
	pub data: String,
	end_index: usize
}

fn get_prefix(s: &String) -> Result<Unit, ()> {
	let end = match s.find(' ') {
		Some(index) => index,
		None => { return Err(()); }
	};
	if end < 2 {
		return Err(());
	}
	let slice = String::from( &s[1..( end )] );
	let ret_data = match slice.find('!') {
		Some(index) => String::from( &slice[..index] ),
		None => slice
	};
	Ok(
		Unit{
			data: ret_data,
			end_index: end
		}
	)
}

fn get_command(s: &String, start: usize) -> Result<Unit, ()> {
	if start > s.len() {
		return Err(());
	}
	let slice = String::from( &s[(start + 1)..] );
	let end = match slice.find(' ') {
		Some(index) => index,
		None => { return Err(()); }
	};
	let ret_data = String::from( &slice[..end] );
	Ok(
		Unit{
			data: ret_data,
			end_index: (end + start + 1)
		}
	)
}

fn get_to(s: &String, start: usize) -> Result<Unit, ()> {
	if start > s.len() {
		return Err(());
	}
	let slice = String::from( &s[start..] );
	let end = match slice.find(':') {
		Some(index) => index,
		None =>  match slice.find(' '){ Some(i) => i, None => return Err(()) }
	};
	let _slice = String::from( &slice[..end] );
	Ok(
		Unit{
			data: String::from( _slice.trim() ),
			end_index: (end + start + 1)
		}
	)
}

fn get_content(s: &String, start: usize) -> String {
	if start > s.len() {
		return s.clone();
	}
	remove_n(String::from( &s[start..] ))
}

fn remove_n(s: String) -> String {
	let mut clean = String::new();
	for ch in s.chars() {
		if ch != '\n'{
			clean.push(ch);
		}
	}
	clean
}

impl Message {
	pub fn set_prefix(&mut self, s: &str) {
		self.prefix.set_data(s);
	}

	pub fn get_prefix_clone(&self) -> String {
		self.prefix.data.clone()
	}
}

impl Unit {
	fn set_data(&mut self, s: &str) {
		self.data = String::from(s);
	}
}

pub fn to_message(s: &String) -> Result<Message, &String> {
	let prefix1 = match get_prefix(&s) {
		Ok(un) => un,
		Err(_) => { return Err(s); }
	};

	let command1 = match get_command(&s, prefix1.end_index) {
		Ok(un) => un,
		Err(_) => { return Err(s); }
	};

	let to1 = match get_to(&s, command1.end_index) {
		Ok(un) => un,
		Err(_) => { return Err(s); }
	};

	let content1 = get_content(&s, to1.end_index);

	Ok(Message{
		prefix: prefix1,
		command: command1,
		to: to1,
		content: content1
	})
}


pub fn parse_message(mess: Message) -> ColorString {
    let mut ret = match &*mess.command.data.to_uppercase() {
		"PRIVMSG" => get_privmsg_fmt(mess),
		"NOTICE" => get_notice_fmt(mess),
		"MODE" => get_mode_fmt(mess),
		"QUIT" => get_quit_fmt(mess),
		"JOIN" => get_join_fmt(mess),
        "NICK" => get_nick_fmt(mess),
		_ => get_misc_fmt(mess)
	};
    ret.push_str("\n");
    ret
}
