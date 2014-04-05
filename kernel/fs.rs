/* kernel::fs.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::container::Container;
use core::cmp::Eq;
use core::mem::transmute;
use kernel::cstr::cstr;
use kernel::*;
use kernel::memory::Allocator;
use super::super::platform::*;
use kernel::cstr::cstr;
use super::super::platform::*;


pub struct dir {
   is_dir: bool,
   name: Option<cstr>,
   // child_dir: Option<dir>,
   child_file: Option<file>
}

pub struct file {
    is_dir: bool,
    name: Option<cstr>,
    value: Option<cstr>
}

impl dir {
    pub unsafe fn new_dir() -> dir {
        let this = dir {
            is_dir: true,
            name: None,
            // child_dir: None,
            child_file: None
        };
        this
    }
}

impl file {
    pub unsafe fn new_file(f_name: cstr) -> file {
        let this = file {
            is_dir: false,
            name: Some(f_name),
            value: Some(cstr::from_str(&""))
        };
        this
    }

    pub unsafe fn read_file(&self, current_dir: dir) -> cstr {
        match current_dir.child_file {
            Some(f) => {
                match (f.name, self.name) {
                    (Some(n1), Some(n2)) => {
                        let mut mut_n2 = n2;
                        if n1.streq(mut_n2.as_str()) {
                            match f.value {
                                Some(v) => v,
                                None => cstr::from_str(&"Error: no value assigned")
                            }
                        } else {
                            cstr::from_str(&"File does not exist.")
                        }
                    },
                    _ => cstr::from_str(&"File does not exist.")
                }
            },
            None => cstr::from_str(&"File does not exist.")
        }
    }
}

//THINGS THE TAS DID THAT I'M CHOOSING TO IGNORE RIGHT NOW
// use core::*;
// use core::str::*;
// use core::option::{Some, Option, None}; // Match statement
// use core::iter::Iterator;
// use core::vec::Vec;
// use super::super::platform::*;


// pub fn open(node: *tree_node, file: cstr) -> (*tree_node, bool, bool)
// {
//     if dir.isLeaf() || file == ""
//     {
// 	return (node, dir.isLeaf(), file == "");
//     }
//     let mut children: uint = (*node).child_count;
//     let mut i: uint = 0;
//     let mut split = file.before('/');
//     while i < children
//     {
// 	if (*node).children[i].name == k
// 	{
// 	    return open((*node).children[i], file.remainder('/'));
// 	} else
// 	{
// 	    i += 1;
// 	}
//     }
//     return cstr::new();
// }

// pub fn append(node: *tree_node, file: cstr, content: cstr) -> bool
// {
//     let (mut f, _, _) = open(node, file);
//     if f == cstr::new()
//     {
// 	return false;
//     }
//     let mut x = 0;
//     let mut f_contents = (*f).contents;
//     while x < content.len()
//     {
// 	let b = f_contents.push_char(content.char_at(x));
// 	if !b
// 	{
// 	    return false;
// 	}
//     }
//     let (*f).contents = f_contents;
//     return true;
// }

// pub fn new(node: *tree_node, dir: cstr, name: cstr) -> bool
// {
//     let (mut n, _, _) = open(node, file);
//     if !n.isLeaf()
//     {
// 	n.insert(name);
//     }
//     return false;

// }
