#![allow(dead_code)]
#![allow(unused)]


/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Notes  ~~~~~~~~~~~~~~~~~~~~~~~~~~

    -- You will eventually want to include choosing the directory
        you want to pull the files from.

 */

/*
    ~~~~~~~~~~~~~~~~~~~~~~~  Goals  ~~~~~~~~~~~~~~~~~~~~~~~~~~

    -- Take a vector of file names

        -- Eliminate unwanted extensions


 */


use lib_file::file_mngmnt::*;

const PROGRAM_TITLE: &str = "File Management Functions";
const VERSION: &str = "0.1.5";

use std::convert::From;


fn splash() {
    println!("~~~~~~~~~~~~~~  {}  ~~~~~~~~~~~~~~~~", PROGRAM_TITLE);
    println!("                  VERSION   {}\n\n", VERSION);
}


fn main() {
    let path = ("/home/camascounty/programming/rust/mine/file_lib").to_string();
    let dirpath = file_choose_new_fname("vrbl", &path);
    println!("\nIn main() the new path & fname is:  {}", dirpath);

    println!("\n All is okay!!  :>) \n");
}

