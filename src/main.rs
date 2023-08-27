mod file_counter;
use crate::file_counter::*;

// Add the necessary imports
use std::path::PathBuf;
use clap::StructOpt;

// Define the command-line options using structopt
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    directory: PathBuf,

    #[structopt(short = 'e', long = "extensions")]
    extensions: Vec<String>,

    #[structopt(short = 'r', long = "recursive")]
    recursive: bool,
}

fn main() {
    // Default list of common image and video extensions
    let default_extensions: Vec<String> = [
        "jpg", "jpeg", "png", "gif", "bmp", // Image formats
        "mp4", "avi", "mkv", "mov", "wmv" // Video formats
    ]
    .iter()
    .map(|&ext| ext.to_string())
    .collect();

    // Parse command-line arguments
    let args: Cli = Cli::from_args();

    // Combine user-provided extensions with default extensions
    let user_extensions: Vec<String> = args.extensions.to_vec();
    let all_extensions: Vec<String> = combine_extensions(user_extensions, default_extensions);

    // Call the function to find files
    let matching_files: Vec<PathBuf> = find_files(&args.directory, &all_extensions, args.recursive);

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
        let user_extensions: Vec<String> = vec!["png".to_string(), "gif".to_string()];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["txt", "jpg", "png", "gif"]);
    }

    #[test]
    fn test_combine_extensions_only_default_extensions() {
        let user_extensions: Vec<String> = vec![];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_with_duplicates() {
        let user_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let default_extensions: Vec<String> = vec!["jpg".to_string(), "png".to_string()];
        let result: Vec<String> = combine_extensions(user_extensions, default_extensions);
        assert_eq!(result, vec!["jpg", "png", "txt", "jpg"]);
    }
    
    #[test]
    fn test_combine_extensions() {
        let user_extensions = vec!["txt".to_string(), "csv".to_string()];
        let default_extensions = vec!["jpg".to_string(), "png".to_string()];

        let combined_extensions = combine_extensions(user_extensions.clone(), default_extensions.clone());

        assert_eq!(combined_extensions.len(), user_extensions.len() + default_extensions.len());
        assert!(combined_extensions.contains(&user_extensions[0]));
        assert!(combined_extensions.contains(&user_extensions[1]));
        assert!(combined_extensions.contains(&default_extensions[0]));
        assert!(combined_extensions.contains(&default_extensions[1]));
    }

}
