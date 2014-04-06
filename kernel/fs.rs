/* kernel::fs.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::container::Container;
use core::cmp::Eq;
use core::mem::transmute;
use kernel::cstr::cstr;
use kernel::vec::Vec;
use kernel::*;
use kernel::memory::Allocator;
use super::super::platform::*;
use kernel::cstr::cstr;
use super::super::platform::*;

pub struct dir {
    path: Option<cstr>,
    dir_name: Option<cstr>,
    dir_children: Option<Vec<dir>>,
    file_children: Option<Vec<file>>
}

pub struct file {
    path: Option<cstr>,
    file_name: Option<cstr>,
    value: Option<cstr>
}

impl dir {
    pub unsafe fn new_dir(p: cstr, n: cstr) -> dir {
        let this = dir {
            path: Some(p),
            dir_name: Some(n),
            dir_children: Some(Vec::new()),
            file_children: Some(Vec::new())
        };
        this
    }

    // pub unsafe fn copy_dir(self) -> dir {
    //     let retdir = dir {
    //         path: self.path,
    //         dir_name: self.dir_name,
    //         dir_children: self.dir_children,
    //         file_children: self.file_children
    //     };
    //     retdir
    // }

    pub unsafe fn get_path(self) -> cstr {
        match self.path {
            Some(p) => p,
            None => cstr::from_str(&"Error: no path")
        }
    }

    pub unsafe fn add_dir(&mut self, d: dir) {
        match self.dir_children {
            Some(ref mut children) => { children.push(d); },
            None => ()
        }
    }

    pub unsafe fn rm_dir(&mut self) -> Option<dir> {
        match self.dir_children {
            Some(ref mut children) => {
                children.pop()
            },
            None => None
        }
    }

    pub unsafe fn get_dir(&mut self) -> Option<dir> {
        match self.dir_children {
            Some(ref mut children) => {
                children.top()
                // let popped_dir = match children.pop() { 
                //     Some(d) => d,
                //     None => dir::new_dir(cstr::from_str(&"fail"), cstr::from_str(&"fail"))
                // };
                // let 
                // children.push(popped_dir);
                // Some(popped_dir)
            },
            None => None
        }
    }

    // pub unsafe fn add_file(&mut self, f: ~file) {
    //     self.child_file = Some(f);
    // }
}

impl file {
    pub unsafe fn new_file(p: cstr, f_name: cstr) -> file {
        let this = file {
            path: Some(p),
            file_name: Some(f_name),
            value: None
        };
        this
    }

    pub unsafe fn read_file(&self) -> cstr {
        match self.value {
            Some(v) => v,
            None => cstr::from_str(&"Error: no value assigned")
        }
    }

    // pub unsafe fn write_file(&self, val: cstr) -> file {
    //     let mut new_file = self.copy_file();
    //     new_file.value = Some(val);
    //     new_file
    // }

    // pub unsafe fn copy_file(&self) -> file {
    //     let new_file = file {
    //         is_dir: false,
    //         name: self.name,
    //         value: self.value
    //     };
    //     new_file
    // }

    // pub unsafe fn exists(&self, current_dir: dir) -> bool {
    //     match current_dir.child_file {
    //         Some(f) => {
    //             match (f.name, self.name) {
    //                 (Some(n1), Some(mut n2)) => {
    //                     if n1.streq(n2.as_str()) {
    //                         true
    //                     } else {
    //                         false
    //                     }
    //                 },
    //                 _ => false
    //             }
    //         },
    //         None => false
    //     }    
    // }
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
