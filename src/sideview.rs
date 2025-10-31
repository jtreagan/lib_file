use std::path::Path;

pub fn file_fullpath(mut usedir: &String) -> String {

    // Make sure the passed directory exists and `startpath` is ready.
    let track = dir_check_valid(&mut usedir);
    let startpath = Path::new(track.as_str());

    // Set the dialog browser to the correct directory.
    let mut dialog = dialog::NativeFileChooser::new(dialog
                                                ::NativeFileChooserType
                                                ::BrowseFile);
    dialog.set_directory(&startpath).expect("Directory does not exist.");

    dialog.show();

    let path = dialog.filename().to_str().unwrap().to_string();
    path
}