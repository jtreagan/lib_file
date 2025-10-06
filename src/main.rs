//#![allow(dead_code)]
//#![allow(unused)]


use lib_file::file_fltk::*;

/*
    PROGRAM_TITLE = "File-Management Function Library";
    VERSION = "0.2.0";
*/

fn main() {
    let usedir = "/home/jtreagan/nothing".to_string();

    let beginningdir = file_fullpath(&usedir);

    println!("\n {} \n", beginningdir);
}




