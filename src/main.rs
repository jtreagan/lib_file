

use lib_file::file_fltk::*;

/*
    PROGRAM_TITLE = "File-Management Function Library";
    VERSION = "0.2.0";
*/

fn main() {
    let usedir = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/lists";

    let path = file_browse_tosave(usedir, "fakefilename", "txt");

    println!("\n {} \n", path);
}




