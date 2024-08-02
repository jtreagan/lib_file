#![allow(dead_code)]
#![allow(unused)]


/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Notes  ~~~~~~~~~~~~~~~~~~~~~~~~~~


 */

/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Goals  ~~~~~~~~~~~~~~~~~~~~~~~~~~



 */


use lib_file::file_mngmnt::*;
use fltk::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::convert::From;
use lib_file::file_fltk::*;

const PROGRAM_TITLE: &str = "File Management Functions";
const VERSION: &str = "0.1.5";

fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}

fn main() {

    let fullpath = file_fullpath_fltr("*.lst");
    println!("\n {:?} \n", fullpath);

    let pathonly = file_pathonly_fltr("*.lst");
    println!("\n {:?} \n", pathonly);

    let nameonly = file_nameonly_fltr("*.lst");
    println!("\n {:?} \n", nameonly);



}



