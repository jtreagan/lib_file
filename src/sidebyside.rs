use std::path::Path;

pub fn file_browse_tosave(sggstdpath: &str, sggstdname: &str, sggstdextnsn: &str, wintitle: &str) -> String {

    // region Check that the passed directory exists and `startpath` is ready.
    let track = dir_check_valid(&mut sggstdpath.to_string());  // Defaults to home directory on err.
    let startpath = Path::new(track.as_str());
    // endregion

    // region Call a dialog browser and set it to the passed directory.
    let mut fchooser = dialog::NativeFileChooser
    ::new(dialog::NativeFileChooserType
    ::BrowseSaveFile);
    fchooser.set_directory(&startpath).expect("Cannot set directory.");
    // endregion

    // region  Add the passed extension to the suggested file name.
    let ext_to_append = sggstdextnsn.strip_prefix("*.").unwrap_or(sggstdextnsn);
    let usename = format!("{}.{}", sggstdname, ext_to_append);
    fchooser.set_preset_file(usename.as_str());
    // endregion

    // region Create the filter string & set the filter.
    let useext;
    if !sggstdextnsn.starts_with("*.") {
        useext = format!("*.{}", sggstdextnsn);
    } else {
        useext = sggstdextnsn.to_string();
    }
    let combined_filter = format!("List Files\t{}\nAll Files\t*.*", useext);
    fchooser.set_filter(&combined_filter);
    // endregion

    // Set the title of the dialog browser.
    fchooser.set_title(wintitle);

    fchooser.show();

    let path = fchooser.filename().to_str().unwrap().to_string();

    path
}