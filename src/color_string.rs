extern crate term_painter;

use self::term_painter::{ToStyle, Color};
use self::term_painter::Color::*;
use std::fmt;
use std::ops::Deref;

pub struct ColorString {
    cont: Vec<ColorWord>
}

pub struct ColorWord {
    cont: String,
    color: self::term_painter::Color
}

impl fmt::Display for ColorWord {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.color.paint(&self.cont))
	}
}

impl Clone for ColorString {
	fn clone(&self) -> Self {
		ColorString{ cont: self.cont.clone() }
	}
}

impl Clone for ColorWord {
	fn clone(&self) -> Self {
		color(self.cont.clone(), self.color)
	}
}

impl fmt::Display for ColorString {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for word in &self.cont {
			let _ = write!(f, "{}", word);
		}
		Ok(())
	}
}

impl ColorString {
	pub fn new(f: Vec<ColorWord>) -> ColorString {
		ColorString {
			cont: f
		}
	}
/*
	pub fn empty_string() -> ColorString {
		ColorString::new(vec![plain("")])
	}

	pub fn to_string(&self) -> String {
		let mut ret = String::new();
		for word in &self.cont {
			ret.push_str(&**word);
		}
		ret
	}
*/
    pub fn push_str(&mut self,s: &str) {
        self.cont.push(ColorWord{ cont: String::from(s), color: White });
    }
}

impl Deref for ColorWord {
	type Target = String;

	fn deref(&self) -> &String {
		&self.cont
	}
}

pub fn color(s: String, col: self::term_painter::Color) -> ColorWord {
	ColorWord {
		cont: s,
		color: col
	}
}



pub fn plain(s: &str) -> ColorWord {
	ColorWord {
		cont: String::from(s),
		color: White
	}
}
