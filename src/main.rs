//#![allow(dead_code)]
//#![allow(unused)]


use lib_file::file_fltk::*;

/*
    PROGRAM_TITLE = "File-Management Function Library";
    VERSION = "0.2.0";
*/

fn main() {
    let usedir = "/home/jtreagan/programming/sssdflkjfslkjfd".to_string();

    let beginningdir = file_nameonly_fltr(&usedir, "*.lst");

    println!("\n {} \n", beginningdir);
}




