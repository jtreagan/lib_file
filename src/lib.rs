
//! # Functions for use in Managing File IO
//! The functions in the modules below were written to help
//! manage file paths, file names, and other file-based
//! operations.  I've used them in several different projects
//! which is why I've kept them together in a separate crate.
//! Their greatest weakness is poor error handling, so keep that
//! in mind if you choose to use them.  By the way, I need help getting
//! those weaknesses corrected, so if you feel like taking that on,
//! please check
//! out the issues tab in this crate's repository.
//!
//!
//!    * VERSION = "0.1.2";
//!    * AUTHOR = "John T. Reagan";
//!    * LICENSE = "MIT";
//!    * LICENSE_URL = "<https://opensource.org/licenses/MIT>";
//!    * COPYRIGHT = "Copyright (c) 2025, John T. Reagan";
//!    * REPOSITORY = "<https://github.com/jtreagan/lib_file>";

/// # Functions based on the fltk::dialog module.
///
pub mod file_fltk {
    // region todo's
    //todo: Passing usedir as an &String is clumsy.  Find a better way.

    //todo:  When running this program with the first call to FLTK.rs's
    //          file `dialog`, the dialog window opens on top of the primary
    //          window just fine.  However, after that any subsequent dialog
    //          windows open behind the primary window.  I need them to open
    //          on top.  You need to fix this in every function.  Here's what the AI said:
    //          ## Option 3: Use FLTK's Modal Windows (Best Long-term Solution)
    //          The proper fix is to modify your `lib_file` crate's dialog functions.
    //          If you can share or modify those functions, you should add parent window
    //           support.   Here's an example of what the fix would look like:
    //          ```rust
    // use fltk::window::Window;
    // pub fn file_fullpath_with_parent(path: &str, parent: Option<&Window>) -> String {
    //     let mut dialog = fltk::dialog::FileDialog::new(fltk::dialog::FileDialogType::BrowseFile);
    //     dialog.set_directory(path);
    //     // Set parent if provided
    //     if let Some(win) = parent {
    //         dialog.set_modal(true);
    //         // This ensures the dialog is associated with the parent window
    //     }
    //     dialog.show();
    //     dialog.filename().to_string_lossy().to_string()
    // }
    //          ```

    //todo: The native Linux file dialog browser does not allow the user to add
    //      a title to the save-file dialog window.  That feature has been
    //      removed from the function.
    //      You may want to use FLTK's file dialog browser instead.
    //      Is it worth it?  Experiment.
    //endregion

    use fltk::dialog;
    use std::path::Path;
    use crate::dir_mngmnt::*;

    /// Prompts the user with a file-save dialog, allowing them to select a file location and name.
    ///
    /// # Parameters
    ///
    /// - `sggstdpath`: A suggested directory path where the dialog should initially open.
    ///   This value is used as a starting point for the file browser, but users can change
    ///   the location if desired. If the directory is invalid or does not exist, the
    ///   function defaults to the user's home directory.
    /// - `sggstdname`: A suggested filename to prefill in the dialog box. Users can
    ///   change this as necessary.
    /// - `extnsnvec`: A vector of file extensions to use as selectable filters during the file-saving
    ///   process. The first item in the vector is assumed to be a label, and the second item is used
    ///   to define the default file extension.
    ///
    /// # Returns
    ///
    /// - Returns a `String` representing the full path selected by the user, including the chosen
    ///   file name. If the dialog is cancelled, this value may vary depending on implementation and
    ///   should be treated with caution.
    ///
    /// # Function Workflow
    ///
    /// 1. **Validate Input Directory Path**:
    ///    - Confirm that `sggstdpath` points to a valid directory.
    ///    - When invalid, the function defaults to the home directory.
    /// 2. **Initialize the File Browser**:
    ///    - A `NativeFileChooser` dialog box of type `BrowseSaveFile` is created and set to open
    ///      at the validated starting directory.
    /// 3. **Apply Default File Extension**:
    ///    - If `extnsnvec` has a valid second entry (default extension), it is appended to
    ///      `sggstdname` as the default suggested filename.
    /// 4. **Set File Filters**:
    ///    - Create a combined filter string from `extnsnvec` and apply it to the file browser.
    /// 5. **User Interaction**:
    ///    - Display the dialog to the user, allowing them to choose a file path and name.
    ///    - The user-selected file path is then returned as a `String`.
    ///
    /// # Assumptions
    ///
    /// - `extnsnvec` contains at least two elements and should contain an even number of elements.
    ///   The first entry is a label and
    ///   the second entry specifies the default file extension.
    /// - The user's environment supports a native file dialog (e.g., a GUI-based desktop).
    /// - The second element in `extnsnvec`, if present, will follow the typical pattern of
    ///   file extension filters (e.g., `"*.txt"`).
    ///
    /// # Example
    ///
    ///     fn main() {
    ///         let app = app::App::default();
    ///         let suggested_path = "/home/user/Documents";
    ///         let suggested_name = "new_file";
    ///         let extensions = vec!["Text Files", "*.txt", "*.csv"];
    ///
    ///         let result = file_browse_tosave(suggested_path, suggested_name, &extensions);
    ///         println!("Selected file path: {}", result);
    ///     }
    ///
    /// # Notes
    ///
    /// - The user can override both the suggested directory and file name.
    /// - Invoking the dialog may depend on system-dependent GUI frameworks.
    /// - Returns an empty string or unexpected value if the dialog is cancelled, so handle
    ///   that return value carefully.
    ///
    /// # Errors
    ///
    /// - Panics if the `set_directory` operation fails for the given directory path.
    /// - If `extnsnvec[1]` is absent or invalid, using an invalid index will cause a panic.
    ///
    /// # See Also:
    ///
    /// - `dialog::NativeFileChooser` for more details about the underlying dialog implementation.
    /// - `fltk_build_dialogfltr` for details on constructing the filter string.
    ///
    /// # Region Breakdown
    ///
    /// - The function is broken into regions for validating paths, setting up dialog filters,
    ///   processing suggested filenames, and showing the file browser to the user.
    pub fn file_browse_tosave(sggstdpath: &str, sggstdname: &str, extnsnvec: &Vec<&str>) -> String {

// region Note:
//   The passed string `sggstdpath` should be a suggested directory for saving a
//   file using the suggested `sggstdname` for a file name.  Note that both
//   variables are "suggestions" allowing for the user to change either.
// endregion

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

        let usename;

        let defaultextnsn = extnsnvec[1];  // Assumes the first entry is a label so chooses the second.
        if defaultextnsn != "" {
            let ext_to_append = defaultextnsn.strip_prefix("*.").unwrap_or(defaultextnsn);
            usename = format!("{}.{}", sggstdname, ext_to_append);
        } else {
            usename = sggstdname.to_string();
        }

        fchooser.set_preset_file(usename.as_str());

        // endregion

        // region Create the filter string & set the filter.

        let combined_filter = fltk_build_dialogfltr(&extnsnvec);
        fchooser.set_filter(&combined_filter);
        // endregion

        fchooser.show();

        let path = fchooser.filename().to_str().unwrap().to_string();
        path
    }

    /*
    /// Opens a native file save dialog for the user to select a location
    /// and name for saving a file, with pre-suggested parameters and a file extension filter.
    ///
    /// # Parameters
    /// - `sggstdpath`: A reference to a string that specifies the suggested directory to open in the file dialog.
    ///                 If the directory does not exist, it defaults to the user's home directory.
    /// - `sggstdname`: A reference to a string that specifies the suggested default file name.
    /// - `sggstdextnsn`: A reference to a string that defines the file extension or filter type for the save dialog.
    ///                   This may include the `*.` prefix (e.g., `*.txt`) but is not required.
    /// - `wintitle`: A reference to a string that specifies the title of the file save dialog window.
    ///
    /// # Returns
    /// Returns a `String` that represents the full path of the file chosen or saved by the user.
    ///
    /// # Behavior
    /// - Ensures the suggested directory (`sggstdpath`) exists before launching the file dialog.
    /// - Uses the suggested directory and file name as defaults in the save dialog but allows the user to modify them.
    /// - Automatically appends the provided file extension (`sggstdextnsn`) to the suggested file name if it is not already included.
    /// - Sets a filter in the file dialog to display only files matching the provided extension and also adds an
    ///   "All Files" filter option for displaying all files.
    /// - Displays the dialog with the title provided in `wintitle`.  That title can also serve as a
    ///     prompt to help guide the user.
    ///
    /// # Notes
    /// - The `sggstdpath` and `sggstdname` parameters are suggestions only; the user can change both during the browsing process.
    /// - If an invalid directory is specified in `sggstdpath`, it defaults to the user's home directory.
    ///
    /// # Example
    ///     fn main() {
    ///         let usedir = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/lists";
    ///         let path = file_browse_tosave(usedir, "fakefilename", "txt");
    ///         println!("\n {} \n", path);
    ///     }
    ///
    /// # Dependencies
    /// - This function utilizes `dialog::NativeFileChooser` for handling the native file save dialog.
    /// - The function requires utilities like `dir_check_valid` to validate the suggested directory.
    pub fn file_browse_tosave_old(sggstdpath: &str, sggstdname: &str, sggstdextnsn: &str) -> String {

        // region Note:
        //          The passed string `sggstdpath` should be a suggested directory for saving a
        //          file using the suggested `sggstdname` for a file name.  Note that both
        //          variables are "suggestions" allowing for the user to change either.
        // endregion
        
        // region Check that the passed directory exists and `startpath` is ready.
        let track = dir_check_valid(&mut sggstdpath.to_string());  // Defaults to home directory on err.
        let startpath = Path::new(track.as_str());
        // endregion

        println!("FBW1");

        // region Call a dialog browser and set it to the passed directory.
        let mut fchooser = dialog::NativeFileChooser
                                ::new(dialog::NativeFileChooserType
                                ::BrowseSaveFile);
        fchooser.set_directory(&startpath).expect("Cannot set directory.");
        // endregion

        println!("FBW2");

        // region  Add the passed extension to the suggested file name.
        let usename;
        if sggstdextnsn != "" {
            let ext_to_append = sggstdextnsn.strip_prefix("*.").unwrap_or(sggstdextnsn);
            usename = format!("{}.{}", sggstdname, ext_to_append);
        }  else {
            usename = sggstdname.to_string();
        }
        fchooser.set_preset_file(usename.as_str());
        // endregion

        println!("FBW3");

        // region Create the filter string & set the filter.

        // todo: Pass the extension suggestion in a vector that can include multiple extensions.
        //      Include an extension title in that vector.  Would a hashmap be a better choice?

        let combined_filter;
        if sggstdextnsn == "" {
            combined_filter = "All Files\t*.*".to_string();


        }  else {
            let useext;
            if !sggstdextnsn.starts_with("*.") {
                useext = format!("*.{}", sggstdextnsn);
            } else {
                useext = sggstdextnsn.to_string();
            }
            combined_filter = format!("Type   \t{}\nAll Files   \t*.*", useext);
        }

        fchooser.set_filter(&combined_filter);
        // endregion

        println!("FBW4");

        fchooser.show();

        println!("FBW5");

        let path = fchooser.filename().to_str().unwrap().to_string();

        println!("FBW6");

        path
    }
    */  // Deleteme later.

    /// Opens a native directory chooser dialog, sets its starting path and window title,
    /// and returns the selected directory path as a `String`.
    ///
    /// # Arguments
    ///
    /// * `sggstdpath` - A mutable string slice representing the initial directory path.
    ///   If the provided path is invalid, it defaults to the user's home directory.
    /// * `wintitle` - A string slice representing the title of the dialog window.
    ///
    /// # Returns
    ///
    /// This function returns the selected directory path as a `String`.
    /// If no selection is made or an error occurs during directory selection,
    /// it may result in unexpected behavior.
    ///
    /// # Panics
    ///
    /// This function will panic if the directory passed to `fchooser.set_directory()`
    /// is invalid and cannot be set.
    ///
    /// # Examples
    ///
    ///     fn main() {
    ///         let usedir = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/lists";
    ///         let path = file_pathonly(usedir, "TITLE TITLE TITLE");
    ///         println!("\n {} \n", path);
    ///     }
    ///
    /// # Notes
    ///
    /// This function relies on `dialog::NativeFileChooser` for platform-native file chooser dialogs
    /// and uses the helper function `dir_normalize_path` to handle and sanitize the input directory path.
    pub fn file_pathonly(mut sggstdpath: &str, wintitle: &str) -> String {

        // region Check that the passed directory exists and `startpath` is ready.
        let track = dir_normalize_path(&mut sggstdpath);  // Defaults to home directory on err.
        let startpath = Path::new(track.as_str());
        // endregion

        // region Call a dialog browser, set it to the passed directory & set the title.

        let mut fchooser = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
        fchooser.set_directory(&startpath).expect("Cannot set directory.");
        fchooser.set_title(wintitle);
        fchooser.show();

        // endregion

        let pathonly = fchooser.filename().to_str().unwrap().to_string();
        pathonly
    }

    /// A function that presents a native file chooser dialog to the user, allowing them to browse
    /// for a file starting from a specified directory, and returns the selected file's name only.
    ///
    /// This function takes two inputs: a mutable reference to a string representing the starting directory
    /// and a string for the window's title. It normalizes the directory path, initializes the native
    /// file chooser dialog, sets its initial directory to the given path, and allows the user to select a file.
    /// The name of the selected file (with no accompanying path information) is then extracted and returned as a `String`.
    ///
    /// # Arguments
    /// * `sggstdpath` - A mutable reference to a string slice representing the starting
    ///   path. If invalid or unavailable, it defaults to the user's home directory.
    /// * `wintitle` - The title of the window for the file chooser dialog.
    ///
    /// # Returns
    /// A `String` containing the name of the selected file, which is extracted from the full path provided
    /// by the file chooser. If the user does not select a file or the operation fails, the function may panic.
    ///
    /// # Panics
    /// This function will panic in the following scenarios:
    /// 1. If the set directory is invalid or cannot be set (this is handled through the dialog API).
    /// 2. If the returned file path does not contain a valid file name (e.g., if the path is empty).
    /// 3. If the extracted filename is not valid UTF-8.
    ///
    /// # Example
    ///     fn main() {
    ///         let usedir = "/some/existing/directory";
    ///         let path = file_nameonly(usedir, "TITLE TITLE TITLE");
    ///         println!("\n {} \n", path);
    ///     }
    ///
    /// Note: This function uses `dialog::NativeFileChooser` for presenting the file dialog
    /// and `std::path::Path` for dealing with directory paths.
    pub fn file_nameonly(mut sggstdpath: &str, wintitle: &str) -> String {

        // region Check that the passed directory exists and `startpath` is ready.
        let track = dir_normalize_path(&mut sggstdpath);  // Defaults to home directory on err.
        let startpath = Path::new(track.as_str());
        // endregion

        // region Call a dialog browser, set it to the passed directory & set the title.

        let mut fchooser = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        fchooser.set_directory(&startpath).expect("Cannot set directory.");
        fchooser.set_title(wintitle);
        fchooser.show();

        // endregion

        // region Extract the file name from the path.

        let path = fchooser.filename();
        let filename = path.file_name().expect("The path has no file available.");
        let filename_str = filename.to_str().expect("The path is not valid UTF-8");
        let filename_string: String = filename_str.to_string();

        // endregion

        filename_string
    }

    /// Generates a dialog to choose a file, starting from a specified directory,
    /// and returns the full path of the selected file.
    ///
    /// # Arguments
    /// - `sggstdpath`: A mutable reference to a string slice representing the starting directory path.
    ///   If the path is invalid or does not exist, it defaults to the home directory.
    /// - `wintitle`: A string slice that sets the window title of the file chooser dialog.
    ///
    /// # Returns
    /// - A `String` containing the full path of the selected file.
    ///
    /// # Behavior
    /// 1. Validates and normalizes the given directory path (`sggstdpath`) to ensure it exists.
    ///    If validation fails, defaults the starting path to the home directory.
    /// 2. Displays a file chooser dialog starting from the provided (or adjusted) directory
    ///    and assigns the given `wintitle` as the window's title.
    /// 3. Retrieves the full path of the selected file and returns it as a string.
    ///
    /// # Panics
    /// - If there is a problem setting the directory for the file chooser dialog, the function will
    ///   panic with the message `"Cannot set directory."`.
    ///
    /// # Example
    ///     fn main() {
    ///         let usedir = "/home/jtreagan/programming/mine/qbnk_rb7/src/qbnk_data/lists";
    ///         let path = file_fullpath(usedir, "TITLE TITLE TITLE");
    ///         println!("\n {} \n", path);
    ///         }
    ///
    /// # Notes
    /// - The function uses the `dialog` crate to create a native file chooser dialog.
    /// - The provided `sggstdpath` is validated and normalized using `dir_normalize_path`.
    /// - The resulting file path is converted to a `String` and returned.
    pub fn file_fullpath(mut sggstdpath: &str, wintitle: &str) -> String {

        // region Check that the passed directory exists and `startpath` is ready.
        let track = dir_normalize_path(&mut sggstdpath);  // Defaults to home directory on err.

        let startpath = Path::new(track.as_str());
        // endregion

        // region Call a dialog browser, set it to the passed directory & set the title.

        let mut fchooser = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        fchooser.set_directory(&startpath).expect("Cannot set directory.");
        fchooser.set_title(wintitle);
        fchooser.show();

        // endregion

        // Extract the path from the file chooser.
        let path = fchooser.filename().to_str().unwrap().to_string();
        path
    }


    // todo: If you add extension and file name fields to the above functions,
    //          you can likely eliminate the need for the following functions.
    
    /// Browse to a desired directory, filter the files by the passed extension.
    /// The returned string includes both the path and the file name.
    ///
    /// Note that the `extension` value must have format  `*.xxxxx`.
    /// Note that a file must be highlighted before the dialog will close.
    pub fn file_fullpath_fltr(mut usedir: &String, extension: &str) -> String {

        // Make sure the passed directory exists and `startpath` is ready.
        let track = dir_check_valid(&mut usedir);  // Defaults to home directory on err.
        let startpath = Path::new(track.as_str());

        // Start dialog browser and set to the correct directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                                        ::NativeFileChooserType
                                        ::BrowseFile);
        dialog.set_directory(&startpath).expect("Directory does not exist.");
        dialog.set_filter(extension);

        dialog.show();

        let path = dialog.filename().to_str().unwrap().to_string();
        path
    }

    /// Browse to a desired directory, filter the files by the passed extension.
    /// The returned string includes only the file name.
    pub fn file_nameonly_fltr(mut usedir: &String, extension: &str) -> String {
        // Note that the `extension` value must have format  `*.xxxxx`.

        // Make sure the passed directory exists and `startpath` is ready.
        let track = dir_check_valid(&mut usedir);  // Defaults to home directory on err.
        let startpath = Path::new(track.as_str());


        // Set the dialog browser to the correct directory.
        let mut dialog = dialog::NativeFileChooser::new(dialog
                                                    ::NativeFileChooserType
                                                    ::BrowseFile);
        dialog.set_directory(&startpath).expect("Directory does not exist.");
        dialog.set_filter(extension);

        dialog.show();

        let path = dialog.filename();

        let filename = path.file_name().expect("The path has no file available.");
        let filename_str = filename.to_str().expect("The path is not valid UTF-8");
        let filename_string: String = filename_str.to_string();

        filename_string
    }

    /// Constructs a filter string for use in FLTK dialog functions based on pairs of labels and extensions.
    ///
    /// The `fltk_build_dialogfltr` function takes a vector of string slices (`&Vec<&str>`) containing alternating
    /// labels and extensions, and produces a properly formatted filter string for use in FLTK file dialogs.
    ///
    /// # Parameters
    ///
    /// * `labelextns`: A vector of string slices containing alternating file type labels and their corresponding
    ///   file extension patterns. Each label should be paired with its extension. Extensions can include wildcards
    ///   (e.g. `*.txt`) or curly-braced patterns (e.g. `{*.jpg,*.png}`).
    ///
    /// # Returns
    ///
    /// Returns a formatted string with the label and extension/filter pattern pairs, each pair on a new line. The
    /// format for each line is:
    ///
    /// ```text
    /// label<TAB>extension_pattern
    /// ```
    ///
    /// - Example:
    ///
    /// ```text
    /// Text Files	*.txt
    /// Images	{*.jpg,*.png}
    /// ```
    ///
    /// # Panics
    ///
    /// This function does not panic, but will silently skip input strings that do not form valid label-extension pairs
    /// (e.g., if the vector has an odd number of elements).
    ///
    /// # Example
    ///
    ///     fn main() {
    ///         let label_extensions = vec![
    ///             "Text Files", "*.txt",
    ///             "Images", "{*.jpg,*.png}",
    ///             "Rust Files", "*.rs"
    ///     ];
    ///
    ///         let dialog_filter = fltk_build_dialogfltr(&label_extensions);
    ///         println!("{}", dialog_filter);
    /// }
    ///
    /// Example output:
    /// ```text
    /// Text Files	*.txt
    /// Images	{*.jpg,*.png}
    /// Rust Files	*.rs
    /// ```
    ///
    /// # Notes
    ///
    /// - If an extension doesn't start with `*.` or doesn't contain `{`, the function automatically prepends
    ///   `*.` to the provided extension.
    /// - The function ignores invalid input pairs without causing a runtime error.
    ///
    pub fn fltk_build_dialogfltr(labelextns: &Vec<&str>) -> String {
        let mut parts = Vec::new();

        for pair in labelextns.chunks(2) {
            match pair {  // I used a match here instead of `if let` because I don't like `if let`.
                [label, extension] => {  // Ensure pattern starts with "*."
                    let usextnsn = if !extension.starts_with("*.") && !extension.contains('{') {
                        format!("*.{}", extension) // temporary string if needed
                    }
                    else {
                        extension.to_string()
                    };
                    parts.push(format!("{}\t{}", label, usextnsn));
                }
                _ => {},
            }
        }
        parts.join("\n")
    }


}  // End of file_fltk module.

/// # Functions dealing with directories.
///
pub mod dir_mngmnt {
    use std::{env, fs, io, path::Path};
    use lib_utils::input_utilities::input_string_prompt;

    /// Retrieves the default home directory path of the current user based on the operating system.
    ///
    /// # Returns
    /// * A `String` containing the path to the home directory.
    ///   - On Windows: Retrieves the value of the `USERPROFILE` environment
    ///         variable. If unavailable, defaults to `"C:\"`.
    ///   - On macOS: Retrieves the value of the `HOME` environment variable.
    ///         If unavailable, defaults to `"/Users"`.
    ///   - On Linux and other Unix-like systems: Retrieves the value of
    ///         the `HOME` environment variable. If unavailable,
    ///         defaults to `"/home"`.
    ///
    /// # Platform Support
    /// - **Windows**: Uses the `USERPROFILE` environment variable.
    /// - **macOS**: Uses the `HOME` environment variable.
    /// - **Linux/Unix**: Uses the `HOME` environment variable.
    ///
    /// # Example:
    ///     fn main() {
    ///         let homedirectory = file_get_home_directory();
    ///         println!("\n {} \n", homedirectory);
    ///     }
    ///
    /// # Note
    /// This function does not verify the existence of the retrieved path, it only returns the configured or default home directory.
    pub fn dir_get_home() -> String {
        if cfg!(windows) {
            env::var("USERPROFILE").unwrap_or_else(|_| "C:\\".to_string())
        } else if cfg!(target_os = "macos") {
            env::var("HOME").unwrap_or_else(|_| "/Users".to_string())
        } else {
            // Linux and other Unix-like systems
            env::var("HOME").unwrap_or_else(|_| "/home".to_string())
        }
    }

    // region TODO'S
    // todo: Could `dir_check_valid` and `dir_normalize_path` be combined into one function?
    //       The only difference is that `dir_check_valid` returns the original path if it exists,
    //       while `dir_normalize_path` returns the parent directory if the path is a file.
    // todo: Could/should they be turned into methods?
    // endregion

    /// Validates the given directory path and ensures it is ready for use.
    /// If the provided directory exists, its path is returned. Otherwise,
    /// the function falls back to the user's home directory.
    ///
    /// # Arguments
    ///
    /// * `usedir` - A reference to a `String` containing the directory path to be checked.
    ///
    /// # Returns
    ///
    /// A `String` containing the valid directory path. If the provided path does not
    /// exist, it returns the user's home directory path.
    ///
    /// # Behavior
    ///
    /// - Verifies whether the directory specified by `usedir` exists.
    /// - If the directory exists, the function returns the original directory path.
    /// - If the directory does not exist:
    ///   - An error message is printed to `stderr` indicating the invalid path.
    ///   - The function defaults to the user's home directory, as obtained
    ///     by the `dir_get_home` function, and returns that instead.
    ///
    /// # Example
    ///     fn main() {
    ///         let path = String::from("/nonexistent/path");
    ///         let valid_path = dir_check_valid(&path);
    ///         println!("Validated path: {}", valid_path); // Prints the home directory path
    ///     }                                               if `/nonexistent/path` doesn't exist.
    ///
    /// # Notes
    ///
    /// The function assumes the existence of a helper function `dir_get_home`,
    /// which retrieves the user's home directory as a `String`. Ensure this
    /// dependency is properly implemented.
    ///
    pub fn dir_check_valid(dirstring: &str) -> String {

        // Make sure the directory exists and `trail` is ready for use.
        let trail = Path::new(dirstring);
        if trail.exists() {
            dirstring.to_string().clone()
        } else {   // If the directory doesn't exist, default to the home directory.
            eprintln!("The path {} does not exist!", dirstring);
            let track: String = dir_get_home();
            track
        }
    }

    /// Normalizes a given directory string to resolve its path based on the context.
    ///
    /// This function processes the input string and modifies it according to the following cases:
    ///
    /// ### Case 1: Input Path is a Valid Directory
    /// - If the given string corresponds to an existing directory, the function returns the directory path
    ///   as is.
    ///
    /// ### Case 2: Input Path is a Valid File
    /// - If the input string points to an existing file, the function returns the parent directory of that file.
    ///
    /// ### Case 3: Input Path is Likely a Directory with an Inaccurate Ending
    /// - If the input string might represent a directory but includes an inaccurate file name, the function
    ///   attempts to resolve and return the parent directory.
    ///
    /// ### Case 4: Cannot Resolve the Input Path
    /// - If none of the above conditions are met (e.g., the path is invalid), an error message will be printed
    ///   to `stderr`, and it will return the user's home directory as a fallback.
    ///
    /// #### Parameters
    /// - `dirstring`: A string slice holding the directory or file path to normalize.
    ///
    /// #### Returns
    /// - A `String` containing the normalized path, based on the provided input's resolution.
    ///
    /// #### Panics
    /// - The function uses `unwrap()` in scenarios where it is guaranteed that the operation is safe
    ///   (e.g., when fetching the parent of a file path).
    ///
    /// #### Side Effects
    /// - Prints an error message to `stderr` if the input cannot be resolved to a valid path or file.
    ///
    /// #### Example
    ///     fn main() {
    ///         let newstring = dir_normalize_path("/home/jtreagan/programming/mine/cards");
    ///         println!("\n The normalized path is: {} \n", newstring);
    ///     }
    ///
    /// This example would print the home directory path if the given path `/some/invalid/file.txt`
    /// cannot be resolved to a legitimate file or directory path.
    ///
    /// #### Notes
    /// - The function relies on another method `dir_get_home` to fetch the user's home directory when
    ///   the input path is invalid.
    pub fn dir_normalize_path(dirstring: &str) -> String {
        let usepath = Path::new(dirstring);

        // region Case 1: The path does not need modification.
        if usepath.is_dir() {
            return usepath.to_string_lossy().into_owned();
        }
        // endregion

        // region Case 2: The path points to an existing file.
        if usepath.is_file() {
            let newpath = usepath.parent();
            if newpath.is_some() {
                return newpath.unwrap().to_string_lossy().into_owned();
                // The unwrap() is safe because the path is a file.
            }
        }
        // endregion

        // region Case 3: The string could be a directory with an inaccurate file name on the end.
        let newpath = usepath.parent().unwrap();
        if newpath.is_dir() {
            return newpath.to_string_lossy().into_owned();
        }
        // endregion

        // region Case 4: The path cannot be resolved so default to the home directory.
        eprintln!("\n Error!  The passed string,   '{}'   is not a path or file.", dirstring);
        dir_get_home()
        // endregion
    }



    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // More recently written functions are above.
    // Older, terminal-based functions that may need modification are below.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    /// Check the validity of a directory path and correct it if necessary.
    ///
    /// Example:
    ///
    ///     fn main() {
    ///         let mut dirpath: String = "kjhkjhjkh/home/camascounty/programming/rust/mine/file_lib".to_string();
    ///
    ///         let dirchecked = dir_checkexist_fix(&dirpath);
    ///         if dirchecked.0 == false {
    ///             println!("\n The path \n      {} \n was not usable and was not corrected. \n", dirpath);
    ///         } else {
    ///            println!("\n The correct path is:  {}", dirchecked.1);
    ///            println!("\n All is okay!!  :>) \n");
    ///         }
    ///     }
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

    /// Check to see if a directory is empty.
    ///
    /// Example:
    ///
    ///      fn main() {
    ///         let directory = "/home/camascounty/programming/rust/mine/empty";
    ///         match dir_check_empty(directory) {
    ///         Ok(is_empty) => {
    ///             if is_empty {
    ///                 println!("\n The path  {}  is empty.", directory);
    ///             } else {
    ///                 println!("\n The path  {}  is not empty.", directory);
    ///             }
    ///         }
    ///         Err(err) => {
    ///             println!("Error when checking if directory is empty: {}", err);
    ///         }
    ///     }
    ///     }
    ///
    pub fn dir_check_empty(dirpath: &str) -> io::Result<bool> {
        let mut entries = fs::read_dir(dirpath)?;
        let first_entry = entries.next();
        Ok(first_entry.is_none())
    }

    /// Check a user-entered path for validity.
    ///
    pub fn dir_get_path() -> (bool, String) {
        let dirpath = input_string_prompt(
            "\n Please enter a path for the directory in which you wish to save this file.  \n\
              (Do not include the file name):   ");

        let dirok = dir_checkexist_fix(&dirpath);
        if dirok.0 == false {
            println!("\n The path \n   {} \n was not usable and was not corrected. \n", dirpath);
            panic!("Invalid and uncorrected path entered.");
            // todo: Maybe eventually return an error that the main program can use to
            // redirect the user's activity.
        }
        (true, dirpath)
    }

    /// Same as `dir_get_path` except that one can pass whatever prompt
    /// you like to the function.
    pub fn dir_get_path_prompt(prompt: &str) -> (bool, String) {
        let dirpath = input_string_prompt( prompt);
        let dirok = dir_checkexist_fix(&dirpath);
        if dirok.0 == false {
            println!("\n The path \n   {} \n was not usable and was not corrected. \n", dirpath);
            panic!("Invalid and uncorrected path entered.");
            // todo: Maybe eventually return an error that the main program can use to
            // redirect user's activity.
        }
        (true, dirpath)
    }




} // End of dir_mngmnt module.

/// # Terminal-based file i/o functions.
///
pub mod file_mngmnt {

//! ### Note the following:
//! 1) I wrote these functions early-on while I was still
//!     learning Rust and the code quality reflects that.
//! 2) While the previous module -- file_fltk -- is
//!     dependent on the FLTK-RS crate, the functions in
//!     this module rely on the Rust standard crates along with
//!     some functions from `lib_utils`, another of my personal
//!     crates.  These functions are all terminal-based.

    use lib_utils::{input_utilities::*, utilities::*};
    use std::io::{BufRead, BufReader, Read, Write};
    use std::{fmt::Debug, fs, fs::File, io, path::Path, str::FromStr};
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::dir_mngmnt::{dir_check_empty, dir_checkexist_fix};

    /// Checks the file extension of a given filename.
    ///
    /// This function takes a string slice representing a filename and determines if it has a valid
    /// (non-empty) file extension. It splits the string from the rightmost `.` character
    /// and checks both the base name and the extension to ensure neither is empty.
    ///
    /// # Arguments
    ///
    /// * `filename` - A string slice representing the filename to be checked.
    ///
    /// # Returns
    ///
    /// * `(bool, &str)` - A tuple where:
    ///   - The first element is a boolean indicating whether a valid extension was found (`true` if valid).
    ///   - The second element is the file extension as a string slice, or an empty string if no valid extension exists.
    ///
    /// # Examples
    ///
    ///     fn main() {
    ///         let test_cases = vec![
    ///          "file.txt",
    ///         "document.pdf",
    ///         "archive.tar.gz",
    ///         "noextension",
    ///         ".hidden",
    ///         "file.",
    ///         "",
    ///         "path/to/file.rs",
    ///     ];
    ///
    ///     for filename in test_cases {
    ///         let (has_ext, ext) = check_extension(filename);
    ///         println!("{:20} -> has_ext: {}, ext: '{}'", filename, has_ext, ext);
    ///         }
    ///     }
    ///
    pub fn file_check_extension(filename: &str) -> (bool, &str) {
        match filename.rsplit_once('.') {
            Some((base, ext)) if !base.is_empty() && !ext.is_empty() => {
                (true, ext)
            }
            _ => (false, "")
        }
    }


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// More recently written functions are above.  Older functions that may need modification are below.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    /// Read a file to a String and print that String to the terminal.
    ///
    pub fn file_read_print_to_term(fname: String) {
        let mut file = File::open(fname.as_str()).expect("Can't open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Oops!  Cant read file...");

        println!("{}", contents);
    }

    /// Read a file to a String with the file name passed
    /// to the function as a RefCell.
    pub fn file_read_file_to_string_refcell(fname: &Rc<RefCell<String>>) -> String {
        let usefname = fname.borrow().clone();

        let mut file = File::open(usefname.as_str()).expect("Can't open file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Oops!  Cant read file...");
        contents
    }

    /// Read a comma delimited file and collect its contents into a vector.
    ///
    /// Example:
    ///
    ///     fn main() {
    ///     // Replace the path below with your own file path to
    ///     // a *.csv (comma separated values) file.
    ///
    ///         let file_path = "/home/somebody/somewhere/rusty/nails.csv";
    ///         let vec = file_read_csv_to_vector(file_path);
    ///
    ///         println!("\n {:?} \n", vec); // Print the resulting vector
    ///     }
    pub fn file_read_csv_to_vector(file_path: &str) -> Vec<String> {  // Comma delimited
        // Read the file into a string
        let content = fs::read_to_string(file_path).expect("Failed to read the file");

        // Split the content by commas and collect into a vector
        content.split(',')
            .map(|s| s.trim().to_string()) // Trim whitespace off each element
            .collect()
    }

    /// Read a file's contents into a String and return
    /// it as a String.
    ///
    /// Example:
    ///
    ///     fn main() {
    ///     let filename = "/home/jtreagan/programming/rust/mine/tr_rbld1/David_config.yaml";
    ///
    ///     match file_read_to_string(filename) {
    ///         Ok(contents) => {
    ///            println!("\n The file contents is:  \n{} \n", contents);
    ///        }
    ///        Err(err) => {
    ///            eprintln!("\n Error reading the file: {} \n", err);
    ///        }
    ///     }
    ///     }
    ///
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

    /// Pull the file name off of the end of a path and return it.
    ///
    pub fn file_path_to_fname(pathstr: &String) -> String {
        // todo: This doesn't look right.  Check it out.
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

    /// Read a folder directory and collect the filenames into a vector.
    /// Then return the vector.
    ///
    ///             ******* Example for file_get_dir_list() ******
    ///
    ///     fn main() {
    ///         let dirpath = "../qbnk_list";
    ///         let file_names = file_get_dir_list(dirpath);
    ///
    ///         println!("\n In main() the list of files is \n {:?}", file_names);
    ///     }
    pub fn file_get_dir_list(path: &str) -> Vec<String> {
        let dir_entries = fs::read_dir(path).unwrap();

        let file_names: Vec<String> = dir_entries
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().unwrap().is_file())
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect();

        file_names
    }

    /// Create a menu from a vector of file names.  Returns the item chosen
    /// by the user as a String.
    ///
    /// Example:
    ///
    ///     fn main() {
    ///         let dirpath = "../qbnk_list";
    ///         let file_names = file_get_list(dirpath);
    ///         let chosen: String;
    ///
    ///         chosen = file_namemenu(&file_names);
    ///
    ///         println!("\n The chosen menu item is:   {}", chosen);
    ///     }
    ///
    pub fn file_namemenu(fnames: &Vec<String>) -> String {
        let choice = util_activity_menu(&fnames, "\n Please choose which file you want to use \n");
        let chosen = &fnames[choice - 1];
        chosen.to_string()
    }

    /// Given a list of file names, this functioncollects all extensions
    /// into a vector
    /// and returns the vector.
    ///
    /// Example:
    ///
    ///          fn main() {
    ///            let mountains = vec!["Soldier.mtn".to_string(),
    ///                             "Deer.low".to_string(),
    ///                             "Buttercup.mtn".to_string(),
    ///                             "Borah.hgher".to_string(),
    ///                             "Newman.low".to_string(),
    ///                             "Dollarhide.mtn".to_string()];
    ///
    ///            let extns = file_extract_extensions(&mountains);
    ///
    ///            println!("\n In main() the list of extensions is \n {:?}", extns);
    ///         }
    ///
    pub fn file_extract_extensions(filelist: &Vec<String>) -> Vec<String> {
        let mut extensions: Vec<String> = Vec::new();

        for item in filelist {
            extensions.push(item.split('.').last().unwrap().to_string());
        }
        extensions
    }

    /// Removes the extension from a file name.  Returns the modified file name.
    ///
    pub fn file_remove_extension(filename: &str) -> String {
        let path = Path::new(filename);
        match path.file_stem() {
            Some(stem) => stem.to_string_lossy().into_owned(),
            None => String::from(filename),
        }
    }

    /// Sorts a vector of file names by their extensions.
    ///
    ///     fn main() {
    ///         let dirpath = "../qbnk_list";
    ///         let mut file_names = file_get_list(dirpath);
    ///
    ///         println!("\n Before:   {:?}", file_names);
    ///
    ///         file_sort_by_ext(&mut file_names);
    ///
    ///         println!("\n After:   {:?}", file_names);
    ///     }
    ///
    pub fn file_sort_by_ext(vctr: &mut Vec<String>) {
        vctr.sort_by(|a, b| {
            let ext_a = a.split('.').last().unwrap();
            let ext_b = b.split('.').last().unwrap();
            ext_a.cmp(ext_b)
        });
    }

    /// Deletes all elements from the given vector that do not have an
    /// extension that matches the passed extension.
    ///
    ///
    ///     fn main() {
    ///        let dirpath = "../qbnk_list";
    ///        let mut file_names = file_get_list(dirpath);
    ///
    ///        println!("\n Before:   {:?}", file_names);
    ///
    ///       file_sort_by_ext(&mut file_names);
    ///       file_del_unwanted_names(&mut file_names, "lst");
    ///
    ///       println!("\n After:   {:?}", file_names);
    ///     }
    ///
    pub fn file_del_unwanted_names(vctr: &mut Vec<String>, keeper_ext: &str) {
        vctr.retain(|item| (item.split('.').last().unwrap()) == keeper_ext);
    }

    /// Read the elements from a given file, storing them in a passed vector.
    ///
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

    /// Saves a vector to a file.
    ///
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

    /// Lets the user input a path string, checks that path for validity, then
    /// lets the user choose a file to work with.  Returns both
    /// the path and the chosen file name.
    /// Example:
    ///
    ///     fn main() {
    ///         let extension = "lst";  // Be sure to check using a non-existing extension.
    ///         let existing_fname = file_choose_from_existing(&extension);
    ///         if existing_fname == "".to_string() {
    ///             return
    ///         } else {
    ///             println!("\n You chose the name   {}   \n", existing_fname);
    ///         }
    ///     }
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

    /// Choose a name for your file from existing files in a given directory.
    ///
    /// Choose a file name to use for saving.
    /// The function adds an extension to the file name and then
    /// appends it to the path.
    ///
    /// Example:
    ///
    ///         fn main() {
    ///             let dirpath = file_choose_new_fname("lst");
    ///             println!("\n In main() the new path & fname is:  {}", dirpath);
    ///
    ///             println!("\n All is okay!!  :>) \n");
    ///         }
    pub fn file_choose_new_fname(extnsn: &str, dirpath: &String) -> String {
        let mut fname: String;
        let mut usepath: String;
        loop {
            usepath = dirpath.clone();
            fname = input_string_prompt("\n Please enter a name for your new file:  ");
            if fname.split('.').last().unwrap() != extnsn {
                fname = fname + "." + extnsn;
            }
            usepath = usepath + "/" + fname.as_str();  // Try using format!() here.
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

    /// Input a file name and append an extension to it.
    ///
    pub fn file_getfname_addextsn(extnsn: &str) -> String {
        let mut fname = input_string_prompt("\n Please enter a name for your new file:  ");
        if fname.split('.').last().unwrap() != extnsn {
            fname = fname + "." + extnsn;
        }
        fname

    }

    /// Add an extension to a file name.
    ///
    pub fn file_addextsn(extnsn: &str, fname: &String) -> String {
        let mut usename = fname.clone();
        if usename.split('.').last().unwrap() != extnsn {
            usename = usename + "." + extnsn;
        }
        usename
    }

    /// This function is not yet finished.  Don't use it.
    ///
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





} // End of file_mngmnt module

