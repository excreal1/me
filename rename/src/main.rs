use std::fs;
use std::env;
use std::path::Path;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        return;
    }

    // Get the directory path from the arguments
    let dir_path = &args[1];
    let path = Path::new(dir_path);

    // Check if the provided path is a directory
    if !path.is_dir() {
        eprintln!("The provided path is not a directory");
        return;
    }

    // Patterns to remove from filenames
    let patterns = [
        " (NKIRI COM)",
        " (NKIRI.COM)",
        " NKIRI.COM",
        " NKIRI",
        " COM",
        " AMZN",
        " WEBRip",
        " COM_2",
        " DOWNLOADED FROM",
        " WEBRip DOWNLOADED FROM NKIRI COM",
    ];

    // Iterate over all entries in the directory
    for entry in fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let old_name = entry.file_name().into_string().expect("Failed to convert OsString to String");

        // Check if the file name contains "NKIRI"
        if old_name.contains("NKIRI") {
            let mut new_name = old_name.clone();
            // Remove specified keywords in the file name
            for pattern in &patterns {
                new_name = new_name.replace(pattern, "");
            }

            // Replace periods with spaces except the one before the file extension
            if let Some(pos) = new_name.rfind('.') {
                let (name_part, ext_part) = new_name.split_at(pos);
                new_name = format!("{}{}", name_part.replace(".", " "), ext_part);
            }

            // Get the old and new file paths
            let old_path = entry.path();
            let new_path = old_path.with_file_name(&new_name);
            // Rename the file
            if let Err(e) = fs::rename(&old_path, &new_path) {
                eprintln!("Failed to rename file {}: {}", old_name, e);
            } else {
                println!("File renamed to: {}", new_name);
            }
        }
    }
}