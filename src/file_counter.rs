use walkdir::WalkDir;
use std::path::{Path, PathBuf};

fn has_valid_extension(file_path: &Path, extensions: &[String]) -> bool {
    if let Some(ext) = file_path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        extensions.iter().any(|e| e == &ext_str)
    } else {
        false
    }
}

pub fn find_files_recursive(directory: &Path, extensions: &[String]) -> Vec<PathBuf> {
    let mut matching_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(directory) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && has_valid_extension(&path, extensions) {
                matching_files.push(path.to_path_buf());
            }
        }
    }

    matching_files
}

// Function to find files non-recursively
pub fn find_files_non_recursive(directory: &PathBuf, extensions: &[String]) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    for entry in directory.read_dir().unwrap() {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_file() {
                if let Some(extension) = entry.path().extension().and_then(|e| e.to_str()) {
                    if extensions.is_empty() || extensions.contains(&extension.to_string()) {
                        file_paths.push(entry.path());
                    }
                }
            }
        }
    }

    file_paths
}

// Unit tests for the functions
#[cfg(test)]
mod tests {
    use super::*;

    // Test data directory structure:
    // - test_data/
    //     - empty_directory/
    //     - non_empty_directory/
    //         - file1.txt
    //         - file2.jpg
    //         - subdirectory/
    //             - file3.png
    //             - file4.mkv


    #[test]
    fn test_has_valid_extension_valid() {
        let file_path = Path::new("test.txt");
        let extensions = vec!["txt".to_string(), "jpg".to_string()];
        assert!(has_valid_extension(&file_path, &extensions));
    }

    #[test]
    fn test_has_valid_extension_invalid() {
        let file_path = Path::new("test.png");
        let extensions = vec!["txt".to_string(), "jpg".to_string()];
        assert!(!has_valid_extension(&file_path, &extensions));
    }

    #[test]
    fn test_has_valid_extension_no_extension() {
        let file_path = Path::new("test");
        let extensions = vec!["txt".to_string(), "jpg".to_string()];
        assert!(!has_valid_extension(&file_path, &extensions));
    }

    #[test]
    fn test_find_files_recursive_no_files() {
        let directory = PathBuf::from("test_data/empty_directory");
        let extensions = vec!["txt".to_string()];
        let result = find_files_recursive(&directory, &extensions);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_recursive_some_files() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["txt".to_string()];
        let result = find_files_recursive(&directory, &extensions);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_recursive_subdirectory() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["png".to_string()];
        let result = find_files_recursive(&directory, &extensions);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_recursive_multiple_extensions() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["txt".to_string(), "jpg".to_string()];
        let result = find_files_recursive(&directory, &extensions);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_files_non_recursive_no_files() {
        let directory = PathBuf::from("test_data/empty_directory");
        let extensions = vec!["txt".to_string()];
        let result = find_files_non_recursive(&directory, &extensions);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_non_recursive_some_files() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["txt".to_string()];
        let result = find_files_non_recursive(&directory, &extensions);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_non_recursive_subdirectory() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["png".to_string()];
        let result = find_files_non_recursive(&directory, &extensions);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_non_recursive_multiple_extensions() {
        let directory = PathBuf::from("test_data/non_empty_directory");
        let extensions = vec!["txt".to_string(), "jpg".to_string()];
        let result = find_files_non_recursive(&directory, &extensions);
        assert_eq!(result.len(), 2);
    }

    // Add more test cases as needed
}