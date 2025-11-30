/*
    PROGRAM_TITLE = "File-Management Function Library";
    VERSION = "0.2.0";
*/
use fltk::app;
use lib_file::file_fltk::*;


fn main() {
   let _app = app::App::default(); // must live while dialogs are used


    //let usedir = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/banks";
    let usedir = "/home/jtreagan/programming/mine/empty";

    let extsnvec = vec!["Lists", "*.lst", "Variables", "*.vrbl", "Banks", "*.bnk", "Text", "*.txt", "All Files", "*.*"];

    let path = file_browse_tosave(usedir, "NAME_OF_FILE_TO_SAVE", &extsnvec);

   // app.run().unwrap();

    println!("\n {} \n", path);
}




