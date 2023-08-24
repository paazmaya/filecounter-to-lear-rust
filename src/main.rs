mod file_counter;
use crate::file_counter::*;


// Add the necessary imports
use std::path::PathBuf;
use structopt::StructOpt;

// Define the command-line options using structopt
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    directory: PathBuf,

    #[structopt(short = "e", long = "extensions")]
    extensions: Vec<String>,

    #[structopt(short = "r", long = "recursive")]
    recursive: bool,
}

fn main() {
    // Default list of common image and video extensions
    let default_extensions: Vec<String> = vec![
        "jpg", "jpeg", "png", "gif", "bmp", // Image formats
        "mp4", "avi", "mkv", "mov", "wmv", // Video formats
    ]
    .iter()
    .map(|&ext| ext.to_string())
    .collect();

    // Parse command-line arguments
    let args: Cli = Cli::from_args();

    // Combine user-provided extensions with default extensions
    let user_extensions: Vec<String> = args.extensions.iter().cloned().collect::<Vec<_>>();
    let all_extensions: Vec<String> = combine_extensions(user_extensions, default_extensions);

    // Call the function to find files
    let matching_files: Vec<PathBuf> = if args.recursive {
        find_files_recursive(&args.directory, &all_extensions)
    } else {
        find_files_non_recursive(&args.directory, &all_extensions)
    };

    let total_files: usize = matching_files.len();

    println!("Total files found: {}", total_files);
}


fn combine_extensions(user_extensions: Vec<String>, default_extensions: Vec<String>) -> Vec<String> {
    let mut all_extensions: Vec<String> = default_extensions;
    all_extensions.extend(user_extensions);
    all_extensions
}

// Unit tests for the functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_extensions_empty() {
        let user_extensions: Vec<String> = vec![];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_with_user_extensions() {
        let user_extensions = vec!["png".to_string(), "gif".to_string()];
        let default_extensions = vec!["txt".to_string(), "jpg".to_string()];
        let result = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["png", "gif", "txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_only_default_extensions() {
        let user_extensions = vec![];
        let default_extensions = vec!["txt".to_string(), "jpg".to_string()];
        let result = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_with_duplicates() {
        let user_extensions = vec!["txt".to_string(), "jpg".to_string()];
        let default_extensions = vec!["jpg".to_string(), "png".to_string()];
        let result = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["txt", "jpg", "png"]);
    }

    // Add more test cases as needed
}
