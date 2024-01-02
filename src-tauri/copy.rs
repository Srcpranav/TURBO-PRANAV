use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{command, State};

#[derive(Clone)]
struct AppState {
    copied_path: Option<PathBuf>,
}

#[command]
fn execute_command(app: State<Arc<Mutex<AppState>>>, command: String) {
    match command.as_str() {
        "copy" => copy_command(app.clone()),
        "paste" => paste_command(app.clone()),
        "cut" => cut_command(app.clone()),
        "rename" => rename_command(app.clone()),
        "newFile" => new_file_command(app.clone()),
        "newFolder" => new_folder_command(app.clone()),
        _ => println!("Unknown command: {}", command),
    }
}

#[command]
fn copy_command(app: State<Arc<Mutex<AppState>>>) {
    // Set the copied_path in the app state
    app.lock().unwrap().copied_path = Some(PathBuf::from("path/to/source/file_or_folder"));
}

#[command]
fn paste_command(app: State<Arc<Mutex<AppState>>>) {
    if let Some(copied_path) = &app.lock().unwrap().copied_path {
        // Assuming the destination path is hard-coded for demonstration purposes
        let destination_path = Path::new("path/to/destination");
        let destination = destination_path.join(copied_path.file_name().unwrap());

        // Check if the destination already exists
        if !destination.exists() {
            // Perform the paste logic by copying the file/folder
            if copied_path.is_file() {
                fs::copy(copied_path, &destination).expect("Copy failed");
            } else {
                fs::create_dir_all(&destination).expect("Create directory failed");
                // You may need to copy the contents of the folder recursively
                // You can use a crate like `fs_extra` for this purpose
            }
            println!("Pasting: {:?}", destination);
        } else {
            println!("Destination already exists");
        }
    } else {
        println!("Nothing to paste");
    }
}

#[command]
fn cut_command(app: State<Arc<Mutex<AppState>>>) {
    // Perform the cut logic
    // Assuming the path is hard-coded for demonstration purposes
    app.lock().unwrap().copied_path = Some(PathBuf::from("path/to/cut/file_or_folder"));
    if let Some(copied_path) = &app.lock().unwrap().copied_path {
        // Remove the original file or folder after cutting
        fs::remove_dir_all(copied_path).expect("Remove failed");
    }
}

#[command]
fn rename_command(app: State<Arc<Mutex<AppState>>>) {
    // Perform the rename logic
    // Assuming the path is hard-coded for demonstration purposes
    if let Some(copied_path) = &app.lock().unwrap().copied_path {
        let new_name = "new_name"; // You should replace this with the user-provided new name
        let renamed_path = copied_path.with_file_name(new_name);
        fs::rename(copied_path, &renamed_path).expect("Rename failed");
        println!("Renaming to: {:?}", renamed_path);
    }
}

#[command]
fn new_file_command(app: State<Arc<Mutex<AppState>>>) {
    // Perform the logic to create a new file
    // Assuming the path is hard-coded for demonstration purposes
    let new_file_path = Path::new("path/to/new/file.txt");
    fs::File::create(new_file_path).expect("Create file failed");
    println!("Creating a new file: {:?}", new_file_path);
}

#[command]
fn new_folder_command(app: State<Arc<Mutex<AppState>>>) {
    // Perform the logic to create a new folder
    // Assuming the path is hard-coded for demonstration purposes
    let new_folder_path = Path::new("path/to/new/folder");
    fs::create_dir_all(new_folder_path).expect("Create directory failed");
    println!("Creating a new folder: {:?}", new_folder_path);
}

fn main() {
    let app_state = Arc::new(Mutex::new(AppState { copied_path: None }));

    let app = tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            execute_command,
            copy_command,
            paste_command,
            cut_command,
            rename_command,
            new_file_command,
            new_folder_command,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(
        |_app_handle| Ok(()),
        |webview, _event| {
            // ... (existing code for setting up the menu)
            Ok(())
        },
    );
}
