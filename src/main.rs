#![allow(dead_code)]
#![allow(unused)]


/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Notes  ~~~~~~~~~~~~~~~~~~~~~~~~~~


 */

/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Goals  ~~~~~~~~~~~~~~~~~~~~~~~~~~

    -- Working on    pathonly()

 */


use lib_file::file_mngmnt::*;
use fltk::*;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::convert::From;

const PROGRAM_TITLE: &str = "File Management Functions";
const VERSION: &str = "0.1.5";

fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}

fn main() {

    let path = file_pathonly();

    println!("\n {:?} \n", path);
}

/*

pub fn file_fullpath() -> String {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();
    println!("\n {:?} \n", dialog.filename());

    let path = dialog.filename().to_str().unwrap().to_string();

    path
}

 */



pub fn file_pathonly() -> String {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

    let path = dialog.filename();
    let pathonly = path.parent().unwrap().to_str().unwrap().to_string();

    pathonly
}



pub fn file_nameonly() -> String {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

    let path = dialog.filename();

    let filename = path.file_name().expect("The path has no file available.");
    let filename_str = filename.to_str().expect("The path is not valid UTF-8");
    let filename_string: String = filename_str.to_string();

    filename_string
}