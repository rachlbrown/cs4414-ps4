/* kernel::fs.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::container::Container;
use core::cmp::Eq;
use core::mem::{move_val_init, transmute};
use core::ptr::{read_ptr};
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

pub struct curr_dir {
    p: *dir
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
    //         p: self.p,
    //         mut_p: self.mut_p,
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

    // pub unsafe fn get_dir(&mut self) -> Option<dir> {
    //     match self.dir_children {
    //         Some(ref mut children) => {
    //             children.top()
    //         },
    //         None => None
    //     }
    // }

    pub unsafe fn get_dir(self, name: cstr) -> cstr {
        let mut retstr = cstr::new(256);
        match self.dir_children {
            Some(mut children) => {
                // children.move_iter();
                while children.len() != 0 {
                    let mut top_dir = children.pop();
                    match top_dir {
                        Some(d) => {
                            match d.dir_name {
                                Some(mut n) => {
                                    if name.streq(n.as_str()) {
                                        retstr = cstr::from_str(&"found the dir!")
                                    } else {
                                        retstr = cstr::from_str(&"dir not found!")
                                    }
                                },
                                None => retstr = cstr::from_str(&"dir not found!")
                            }
                        },
                        None => retstr = cstr::from_str(&"dir not found!")
                    }
                }
            },
            None => retstr = cstr::from_str(&"dir not found!")
        };
        retstr
    }

    pub unsafe fn get_file(&mut self) -> Option<file> {
        match self.file_children {
            Some(ref mut children) => {
                children.top()
            },
            None => None
        }
    }

    pub unsafe fn add_file(&mut self, f: file) {
        match self.file_children {
            Some(ref mut children) => { children.push(f); },
            None => ()
        }
    }

    pub unsafe fn access_dir(ptr: *dir) -> dir {
        read_ptr(ptr)
    }
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

    pub unsafe fn write_file(&mut self, val: cstr) {
        self.value = Some(val);
    }

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

impl curr_dir {
    pub unsafe fn get_current_dir(&self) -> dir {
        read_ptr(self.p)
    }
}
