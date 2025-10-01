//#![allow(dead_code)]
//#![allow(unused)]


use lib_file::file_fltk::*;

/*
    PROGRAM_TITLE = "File-Management Function Library";
    VERSION = "0.2.0";
*/

fn main() {
    let usedir = "/home/jtreagan/programming/mine/lib_file".to_string();

    let homedirectory = file_pathonly(&usedir);

    println!("\n {} \n", homedirectory);
}




