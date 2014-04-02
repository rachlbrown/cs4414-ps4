/* kernel::cstr.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::container::Container;
use core::cmp::Eq;
use core::mem::transmute;
use kernel::*;
use kernel::memory::Allocator;
use super::super::platform::*;

pub struct cstr {
	p: *mut u8,
	p_cstr_i: uint,
	max: uint 
}

impl cstr {
	pub unsafe fn new(size: uint) -> cstr {
		// Sometimes this doesn't allocate enough memory and gets stuck...
		let (x, y) = heap.alloc(size);
		let this = cstr {
			p: x,
			p_cstr_i: 0,
			max: y
		};
		*(((this.p as uint)+this.p_cstr_i) as *mut char) = '\0';
		this
	}

#[allow(dead_code)]
	pub unsafe fn from_str(s: &str) -> cstr {
		let mut retstr = cstr::new(256);
		for c in slice::iter(as_bytes(s)) {
			retstr.add_char(*c);
		};
		retstr
	}

	pub unsafe fn as_ptr(&self) -> *u8 {
		transmute(self.p)
	}

	pub unsafe fn as_str(&mut self) -> &str {
        transmute((self.p, self.p_cstr_i))
	}

	// HELP THIS DOESN'T WORK THERE IS NO GARBAGE COLLECTION!!!
	// -- TODO: exchange_malloc, exchange_free
#[allow(dead_code)]
	unsafe fn destroy(&mut self) { heap.free(self.p); }

	pub unsafe fn add_char(&mut self, x: u8) -> bool{
		if (self.p_cstr_i == self.max) { return false; }
		*(((self.p as uint)+self.p_cstr_i) as *mut u8) = x;
		self.p_cstr_i += 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	pub unsafe fn delete_char(&mut self) -> bool {
		if (self.p_cstr_i == 0) { return false; }
		self.p_cstr_i -= 1;
		*(((self.p as uint)+self.p_cstr_i) as *mut char) = '\0';
		true
	}

	pub unsafe fn reset(&mut self) {
		self.p_cstr_i = 0; 
		*(self.p as *mut char) = '\0';
	}

#[allow(dead_code)]

	pub unsafe fn streq(&self, other: &str) -> bool {
		let mut selfp: uint = self.p as uint;
		for c in slice::iter(as_bytes(other)) {
			if( *c != *(selfp as *u8) ) { return false; }
			selfp += 1;
		};
		*(selfp as *char) == '\0'
	}

	pub unsafe fn getarg(&mut self, delim: char, mut k: uint) -> Option<cstr> {
		let mut ind: uint = 0;
		let mut found = k == 0;
		let mut selfp: uint = self.p as uint;
		let mut s = cstr::new(256);
		loop {
			if (*(selfp as *char) == '\0') { 
				// End of string
				if (found) { return Some(s); }
				else { return None; }
			};
			if (*(selfp as *u8) == delim as u8) { 
				if (found) { return Some(s); }
				k -= 1;
			};
			if (found) {
				s.add_char(*(selfp as *u8));
			};
			found = k == 0;
			selfp += 1;
			ind += 1;
			if (ind == self.max) { 
				//putstr(&"\nSomething broke!");
				return None; 
			}
		}
	}

#[allow(dead_code)]
	pub unsafe fn split(&mut self, delim: char) -> (cstr, cstr) {
		let mut selfp: uint = self.p as uint;
		let mut beg = cstr::new(256);
		let mut end = cstr::new(256);
		let mut found = false;
		loop {
			if (*(selfp as *char) == '\0') { 
				return (beg, end);
			} else if (*(selfp as *u8) == delim as u8) && !found {
				found = true;
			} else if (!found) {
				beg.add_char(*(selfp as *u8));
			} else if (found) {
				end.add_char(*(selfp as *u8));
			};
			selfp += 1;
		}
	}
}

impl Container for cstr {

#[allow(dead_code)]
	fn len(&self) -> uint { self.p_cstr_i }

}

impl Eq for cstr {

#[allow(dead_code)]
#[inline]
	fn eq(&self, other: &cstr) -> bool {
		if (self.len() != other.len()) { return false; }
		else {
			let mut x = 0;
			let mut selfp: uint = self.p as uint;
			let mut otherp: uint = other.p as uint;
			unsafe {
				while x < self.len() {
					if (*((selfp + x) as *u8) != *((otherp + x) as *u8)) { return false; }
					x += 1;
				}
			}
			true
		}
	}

}