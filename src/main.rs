mod file_counter;
use crate::file_counter::*;

// Add the necessary imports
use std::path::PathBuf;
use clap::Parser;

const MEDIA_EXTENSIONS: [&str; 10] = [
    "jpg", "jpeg", "png", "gif", "bmp", // Image formats
    "mp4", "avi", "mkv", "mov", "wmv" // Video formats
];

// Define the command-line options using structopt
#[derive(Parser, Debug)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    directory: PathBuf,

    #[arg(short = 'e')]
    extensions: Vec<String>,

    #[arg(short = 'r')]
    recursive: bool,
}

fn main() {
    // Default list of common image and video extensions
    let default_extensions: Vec<String> = MEDIA_EXTENSIONS.to_vec().iter()
    .map(|&ext| ext.to_string())
    .collect();

    // Parse command-line arguments
    let args = Cli::parse();
    //println!("{args:?}");

    // Combine user-provided extensions with default extensions
    let user_extensions: Vec<String> = args.extensions.to_vec();
    let all_extensions: Vec<String> = combine_extensions(&user_extensions, &default_extensions);

    // Call the function to find files
    let matching_files: Vec<PathBuf> = find_files(&args.directory, &all_extensions, args.recursive);

    let total_files: usize = matching_files.len();

    println!("Total files found: {}", total_files);
}

fn combine_extensions(user_extensions: &[String], default_extensions: &[String]) -> Vec<String> {
    let mut all_extensions: Vec<String> = default_extensions.to_owned();
    all_extensions.extend(user_extensions.to_owned());
    all_extensions
}

// Unit tests for the functions
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }

    #[test]
    fn test_combine_extensions_empty() {
        let user_extensions: Vec<String> = vec![];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(&user_extensions, &default_extensions);
        assert_eq!(result, vec!["txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_with_user_extensions() {
        let user_extensions: Vec<String> = vec!["png".to_string(), "gif".to_string()];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(&user_extensions, &default_extensions);
        assert_eq!(result, vec!["txt", "jpg", "png", "gif"]);
    }

    #[test]
    fn test_combine_extensions_only_default_extensions() {
        let user_extensions: Vec<String> = vec![];
        let default_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<String> = combine_extensions(&user_extensions, &default_extensions);
        assert_eq!(result, vec!["txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions_with_duplicates() {
        let user_extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let default_extensions: Vec<String> = vec!["jpg".to_string(), "png".to_string()];
        let result: Vec<String> = combine_extensions(&user_extensions, &default_extensions);
        assert_eq!(result, vec!["jpg", "png", "txt", "jpg"]);
    }

    #[test]
    fn test_combine_extensions() {
        let user_extensions = vec!["txt".to_string(), "csv".to_string()];
        let default_extensions = vec!["jpg".to_string(), "png".to_string()];

        let combined_extensions = combine_extensions(&user_extensions, &default_extensions);

        assert_eq!(combined_extensions.len(), user_extensions.len() + default_extensions.len());
        assert!(combined_extensions.contains(&user_extensions[0]));
        assert!(combined_extensions.contains(&user_extensions[1]));
        assert!(combined_extensions.contains(&default_extensions[0]));
        assert!(combined_extensions.contains(&default_extensions[1]));
    }

}
