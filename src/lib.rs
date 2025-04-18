/*
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                    Functions for use in Managing File IO

                                To Do List

    -- Write examples for functions that don't already have one.
    -- Note that examples should have some meat to them.  assert!() macros
        are fine, but if someone is interested in using one of these
        functions they will need an example that actually does something.
*/

// region Credit Constants
const _VERSION: &str = "0.0.5";
const _AUTHOR: &str = "John T. Reagan";
const _LICENSE: &str = "MIT";
const _LICENSE_URL: &str = "https://opensource.org/licenses/MIT";
const _COPYRIGHT: &str = "Copyright (c) 2025, John T. Reagan";
const _REPOSITORY: &str = "https://github.com/jtreagan/lib_file";
// endregion

pub mod file_fltk {

/*
                                TO DO

    -- The file browser is forcing the choice of whatever
            folder is at the end of the branch.  It won't let
            the user choose some folder in the middle of the
            branch.  Fix it.

    -- Need to deal with the result returned by
            dialog.set_directory(&strtpath);
*/

    use fltk::dialog;
    use std::path::Path;

    pub fn file_browse_save(usedir: &String) -> String {

        // region Convert the text of the starting directory into a PATH & check exists.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }
        // endregion

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser
                                ::new(dialog::NativeFileChooserType
                                ::BrowseSaveFile);

        dialog.set_preset_file(usedir); // Set the suggested file name

        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename().to_str().unwrap().to_string();
        path
    }

    /*
                    Sample Usage --   file_browse_save_fltr()

    let path = file_browse_save_fltr("Text Files   \t*.txt\nVariable Files   \t*.vrbl\nAll Files");
    let path = file_browse_save_fltr("*.*");    // To show all files.
 */  // Sample Usage --   file_browse_save_fltr()
    pub fn file_browse_save_fltr(usedir: &String, extension: &str) -> String{
        // Note that the `extension` value must have format  "*.xxxxx"

        // region Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }
        // endregion

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser
                                ::new(dialog::NativeFileChooserType
                                ::BrowseSaveFile);

        dialog.set_preset_file(usedir); // Set the suggested file name

        dialog.set_filter(extension);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename().to_str().unwrap().to_string();
        path
    }


    pub fn file_fullpath(usedir: &String) -> String {

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                        ::NativeFileChooserType::BrowseFile);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename().to_str().unwrap().to_string();

        path
    }

    pub fn file_pathonly(usedir: &String) -> String {
        // Convert the RefCell contents to a String.
        //let rc_contents: String = usedir.borrow().clone();

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                        ::NativeFileChooserType::BrowseFile);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename();
        let pathonly = path.parent().unwrap().to_str().unwrap().to_string();
        pathonly
    }

    pub fn file_nameonly(usedir: &String) -> String {
        // Convert the RefCell contents to a String.
        //let rc_contents: String = usedir.borrow().clone();

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                        ::NativeFileChooserType::BrowseFile);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename();

        let filename = path.file_name().expect("The path has no file available.");
        let filename_str = filename.to_str().expect("The path is not valid UTF-8");
        let filename_string: String = filename_str.to_string();

        filename_string
    }


    pub fn file_fullpath_fltr(usedir: &String, extension: &str) -> String {
        // Note that the `extension` value must have format  `*.xxxxx`.

        // Convert the RefCell contents to a String.
        //let rc_contents: String = usedir.borrow().clone();

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                        ::NativeFileChooserType::BrowseFile);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }
        dialog.set_filter(extension);

        dialog.show();

        let path = dialog.filename().to_str().unwrap().to_string();
        path
    }

    pub fn file_pathonly_fltr(usedir: &String, extension: &str) -> String {
        // Note that the `extension` value must have format  `*.xxxxx`.

        // Convert the RefCell contents to a String.
        //let rc_contents: String = usedir.borrow().clone();

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter(extension);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename();
        let pathonly = path.parent().unwrap().to_str().unwrap().to_string();
        pathonly
    }

    pub fn file_nameonly_fltr(usedir: &String, extension: &str) -> String {
        // Note that the `extension` value must have format  `*.xxxxx`.

        // Convert the RefCell contents to a String.
        //let rc_contents: String = usedir.borrow().clone();

        // Convert the text of the starting directory into a PATH.
        let strtpath = Path::new(usedir.as_str());
        if !strtpath.exists() {
            eprintln!("The path {} does not exist!", strtpath.display());
        }

        // Set the dialog browser to the default directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter(extension);
        let setrslt = dialog.set_directory(&strtpath);
        if let Err(e) = setrslt {
            eprintln!("Failed to set starting directory to:  {}", e);
        }

        dialog.show();

        let path = dialog.filename();

        let filename = path.file_name().expect("The path has no file available.");
        let filename_str = filename.to_str().expect("The path is not valid UTF-8");
        let filename_string: String = filename_str.to_string();

        filename_string
    }


}  // End file_fltk module.


pub mod file_mngmnt {

    /*
        Note the following:
            1. I wrote these functions early-on while I was still learning Rust
                and the code quality reflects that.

            2. While the previous module -- file_fltk -- is dependent on the
                FLTK-RS crate, the functions in this module rely on the Rust
                standard crate.
    */

    use lib_jt::{input_utilities::*, misc::*};
    use std::io::{BufRead, BufReader, Read, Write};
    use std::{fmt::Debug, fs, fs::File, io, path::Path, str::FromStr};


    /*

        fn main() {
        let file_path = "/home/jtreagan/programming/rust/mine/cards/front_files_list.txt"; // Replace with your file path
        let vec = file_read_csv_to_vector(file_path);

        println!("\n {:?} \n", vec); // Print the resulting vector
}

    */  // Example for file_read_csv_to_vector()
    pub fn file_read_csv_to_vector(file_path: &str) -> Vec<String> {  // Comma delimited
        // Read the file into a string
        let content = fs::read_to_string(file_path).expect("Failed to read the file");

        // Split the content by commas and collect into a vector
        content.split(',')
            .map(|s| s.trim().to_string()) // Trim whitespace off each element
            .collect()
    }

    /*  ******  Example for  file_read_to_string()   *****
    fn main() {
        let filename = "/home/jtreagan/programming/rust/mine/tr_rbld1/David_config.yaml";

        match file_read_to_string(filename) {
            Ok(contents) => {
                println!("\n The file contents is:  \n{} \n", contents);
            }
            Err(err) => {
                eprintln!("\n Error reading the file: {} \n", err);
            }
        }
    }

 */  // Example for  file_read_to_string()
    pub fn file_read_to_string(fname: &str) -> io::Result<String> {
        // TODO: This needs better error handling.

        // Attempt to open the file
        let mut file = File::open(fname)?;

        // Prepare a String to store the file's contents
        let mut contents = String::new();

        // Read the file's contents into the string
        file.read_to_string(&mut contents)?;

        // Return the contents
        Ok(contents)
    }

    pub fn file_path_to_fname(pathstr: &String) -> String {
        let usepath = Path::new(pathstr);

        match usepath.file_name() {
            Some(filename_osstr) => {
                match filename_osstr.to_str() {
                    Some(filename_str) => filename_str.to_string(),
                    None => "Error: Could not convert filename to UTF-8 string.".to_string(),
                }
            }
            None => "Error: could not get filename".to_string()
        }
    }

    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for file_get_dir_list() ******

    fn main() {
        let dirpath = "../qbnk_list";
        let file_names = file_get_dir_list(dirpath);

        println!("\n In main() the list of files is \n {:?}", file_names);
    }
  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */  // ******* Example for file_get_dir_list() ******
    pub fn file_get_dir_list(path: &str) -> Vec<String> {
        let dir_entries = fs::read_dir(path).unwrap();

        let file_names: Vec<String> = dir_entries
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().unwrap().is_file())
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect();

        file_names
    }

    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                ******* Example for file_namemenu() ******

    fn main() {
        let dirpath = "../qbnk_list";
        let file_names = file_get_list(dirpath);
        let chosen: String;

        chosen = file_namemenu(&file_names);

        println!("\n The chosen menu item is:   {}", chosen);
    }

  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */  // ******* Example for file_namemenu() ******
    pub fn file_namemenu(fnames: &Vec<String>) -> String {
        let choice = activity_menu(&fnames, "\n Please choose which file you want to use \n");
        let chosen = &fnames[choice - 1];
        chosen.to_string()
    }

    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for file_extract_extensions() ******

    fn main() {
        let mountains = vec!["Soldier.mtn".to_string(), "Deer.low".to_string(),
                 "Buttercup.mtn".to_string(), "Borah.hgher".to_string(),
                 "Newman.low".to_string(), "Dollarhide.mtn".to_string()];

        let extns = file_extract_extensions(&mountains);

        println!("\n In main() the list of extensions is \n {:?}", extns);
    }
  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    pub fn file_extract_extensions(filelist: &Vec<String>) -> Vec<String> {
        let mut extensions: Vec<String> = Vec::new();

        for item in filelist {
            extensions.push(item.split('.').last().unwrap().to_string());
        }
        extensions
    }



    pub fn file_remove_extension(filename: &str) -> String {
        let path = Path::new(filename);
        match path.file_stem() {
            Some(stem) => stem.to_string_lossy().into_owned(),
            None => String::from(filename),
        }
    }


    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for file_sort_by_ext() ******

    fn main() {
        let dirpath = "../qbnk_list";
        let mut file_names = file_get_list(dirpath);

        println!("\n Before:   {:?}", file_names);

        file_sort_by_ext(&mut file_names);

        println!("\n After:   {:?}", file_names);
    }

  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    pub fn file_sort_by_ext(vctr: &mut Vec<String>) {
        vctr.sort_by(|a, b| {
            let ext_a = a.split('.').last().unwrap();
            let ext_b = b.split('.').last().unwrap();
            ext_a.cmp(ext_b)
        });
    }


    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for file_del_unwanted_names() ******

      -- Delete unwanted names (by extension) from the file-names vector.

    fn main() {
        let dirpath = "../qbnk_list";
        let mut file_names = file_get_list(dirpath);

        println!("\n Before:   {:?}", file_names);

        file_sort_by_ext(&mut file_names);
        file_del_unwanted_names(&mut file_names, "lst");

        println!("\n After:   {:?}", file_names);
    }

  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    pub fn file_del_unwanted_names(vctr: &mut Vec<String>, keeper: &str) {
        vctr.retain(|item| (item.split('.').last().unwrap()) == keeper);
    }

    pub fn file_read_to_vec<T: FromStr>(fname: &str, vctr: &mut Vec<T>)
        where <T as FromStr>::Err: Debug {
        let file = File::open(fname).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let num = line.parse().unwrap();
            vctr.push(num);
        }
    }

    pub fn file_save_vec<T: std::fmt::Display>(fname: &str, vector: &[T]) ->
                                                          std::io::Result<()> {
        let mut file = File::create(fname)?;
        for num in vector {
            let num_str = num.to_string();
            file.write_all(num_str.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }


    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        ******* Example for file_choose_from_existing() ******

  -- Choose a file name from the list of files that already exist in the
        given directory.

    fn main() {
        let extension = "lst";  // Be sure to check using a non-existing extension.
        let existing_fname = file_choose_from_existing(&extension);
        if existing_fname == "".to_string() {
            return
        } else {
            println!("\n You chose the name   {}   \n", existing_fname);
        }
    }


~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    pub fn file_choose_from_existing(extsn: &str) -> (String, String) {



        let dirpath = input_string_prompt("Please enter the path for the directory where this file has been saved:  ");
        let dirok = dir_checkexist_fix(&dirpath);
        if dirok.0 == false {
            println!("\n The path \n   {} \n was not usable and was not corrected. \n", dirpath);
            panic!("Invalid and uncorrected path entered.");
            // Maybe eventually return a result that the main program can use to
            // redirect user's activity.
        }

        let is_empty = dir_check_empty(&dirpath).unwrap();
        if is_empty {
            println!("\n That directory is empty.");
            return (dirpath, "".to_string());
        }
        let mut file_names = file_get_dir_list(dirpath.as_str());

        file_del_unwanted_names(&mut file_names, extsn);
        if file_names.len() == 0 {
            println!("\n There are no *.{} files in this directory.", extsn);
            return (dirpath, "".to_string());
        }

        let chosen = file_namemenu(&file_names);
        (dirpath, chosen)
    }


    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for file_choose_new_fname() ******

                -- Enter a new file name to use for saving.
                -- Adds extension to the fname and then adds it
                    to the file path.

        fn main() {
            let dirpath = file_choose_new_fname("lst");
            println!("\nIn main() the new path & fname is:  {}", dirpath);

            println!("\n All is okay!!  :>) \n");
        }

  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ */

    pub fn file_choose_new_fname(extnsn: &str, dirpath: &String) -> String {
        let mut fname: String;
        let mut usepath: String;
        loop {
            usepath = dirpath.clone();
            fname = input_string_prompt("\n Please enter a name for your new file:  ");
            if fname.split('.').last().unwrap() != extnsn {
                fname = fname + "." + extnsn;
            }
            usepath = usepath + "/" + fname.as_str();
            let fullpath = Path::new(&usepath);
            let exists = fullpath.try_exists()
                .expect("Error when checking the existence of the file");
            if exists {
                println!("\n That file   {}   already exists.", &fname);
                let choice = input_bool_prompt("\n Do you want to overwrite the file? ");
                if choice { break; }
                else { continue; }
            }
            break;
        }
        usepath
    }

    pub fn file_getfname_addextsn(extnsn: &str) -> String {
        let mut fname = input_string_prompt("\n Please enter a name for your new file:  ");
        if fname.split('.').last().unwrap() != extnsn {
            fname = fname + "." + extnsn;
        }
        fname

    }

    pub fn file_addextsn(extnsn: &str, fname: &String) -> String {
        let mut usename = fname.clone();
        if usename.split('.').last().unwrap() != extnsn {
            usename = usename + "." + extnsn;
        }
        usename
    }

    pub fn file_chkfname( fname: &String, dirpath: &String) -> String {
// This is not yet ready.  What are you returning?


        let mut usepath: String;
        loop {
            usepath = dirpath.clone();
            usepath = usepath + "/" + fname.as_str();
            let fullpath = Path::new(&usepath);
            let exists = fullpath.try_exists()
                .expect("Error when checking the existence of the file");
            if exists {
                println!("\n That file   {}   already exists.", &fname);
                let choice = input_bool_prompt("\n Do you want to overwrite the file? ");
                if choice {
                    break;
                }
                else { continue; }
            }
            break;
        }
        usepath
    }



    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            ******* Example for dir_checkexist_fix() ******

         -- Check that a directory path actually exists and correct it if it is wrong.

    fn main() {
        let mut dirpath: String = "kjhkjhjkh/home/camascounty/programming/rust/mine/file_lib".to_string();

        let dirchecked = dir_checkexist_fix(&dirpath);
        if dirchecked.0 == false {
            println!("\n The path \n      {} \n was not usable and was not corrected. \n", dirpath);
        } else {
            println!("\n The correct path is:  {}", dirchecked.1);
            println!("\n All is okay!!  :>) \n");
        }
    }

 */

    pub fn dir_checkexist_fix(dirpath: &String) -> (bool, String) {
        let mut fullpath = Path::new(dirpath.as_str());
        let mut newpath: String = dirpath.to_string().clone();

        loop {
            let exists = fullpath.try_exists()
                .expect("Error when checking the existence of the directory");

            if exists {
                return (true, newpath);
            } else {
                println!("\n The directory \n      {} \n does not exist and may not be used.", newpath);
                newpath = input_string_prompt(
                    "\n Please enter a corrected path for the directory in which you wish to save this file.  \n\
                         (Do not include the file name):   ");  // Eventually add ability to edit the existing string.
                if newpath == "" {
                    return (false, "".to_string());
                } else {
                    fullpath = Path::new(newpath.as_str());
                }
            }
        }
    }

    /* ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        ******* Example for dir_check_empty() ******

     -- Check whether a directory path actually contains files or folders.

fn main() {
    let directory = "/home/camascounty/programming/rust/mine/empty";
    match dir_check_empty(directory) {
        Ok(is_empty) => {
            if is_empty {
                println!("\n The path  {}  is empty.", directory);
            } else {
                println!("\n The path  {}  is not empty.", directory);
            }
        }
        Err(err) => {
            println!("Error when checking if directory is empty: {}", err);
        }
    }
}

*/

    pub fn dir_check_empty(dirpath: &str) -> std::io::Result<bool> {
        let mut entries = fs::read_dir(dirpath)?;
        let first_entry = entries.next();
        Ok(first_entry.is_none())
    }

    pub fn dir_get_path() -> (bool, String) {
        let dirpath = input_string_prompt(
            "\n Please enter a path for the directory in which you wish to save this file.  \n\
              (Do not include the file name):   ");

        let dirok = dir_checkexist_fix(&dirpath);
        if dirok.0 == false {
            println!("\n The path \n   {} \n was not usable and was not corrected. \n", dirpath);
            panic!("Invalid and uncorrected path entered.");
            // Maybe eventually return an error that the main program can use to
            // redirect user's activity.
        }
        (true, dirpath)
    }

    pub fn dir_get_path_prompt(prompt: &str) -> (bool, String) {
        let dirpath = input_string_prompt( prompt);
        let dirok = dir_checkexist_fix(&dirpath);
        if dirok.0 == false {
            println!("\n The path \n   {} \n was not usable and was not corrected. \n", dirpath);
            panic!("Invalid and uncorrected path entered.");
            // Maybe eventually return an error that the main program can use to
            // redirect user's activity.
        }
        (true, dirpath)
    }



} // End file_mngmnt module.

