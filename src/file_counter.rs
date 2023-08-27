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

pub fn find_files(directory: &Path, extensions: &[String], recursive: bool) -> Vec<PathBuf> {
    let mut matching_files: Vec<PathBuf> = Vec::new();

    let walker: walkdir::IntoIter = if recursive {
        WalkDir::new(directory).into_iter()
    } else {
        WalkDir::new(directory).max_depth(1).into_iter()
    };

    for entry in walker.filter_map(|e: Result<walkdir::DirEntry, walkdir::Error>| e.ok()) {
        if entry.file_type().is_file() {
            let path: &Path = entry.path();
            if has_valid_extension(path, extensions) {
                matching_files.push(path.to_path_buf());
            }
        }
    }

    matching_files
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
        let file_path: &Path = Path::new("test.txt");
        let extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        assert!(has_valid_extension(file_path, &extensions));
    }

    #[test]
    fn test_has_valid_extension_invalid() {
        let file_path: &Path = Path::new("test.png");
        let extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        assert!(!has_valid_extension(file_path, &extensions));
    }

    #[test]
    fn test_has_valid_extension_no_extension() {
        let file_path: &Path = Path::new("test");
        let extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        assert!(!has_valid_extension(file_path, &extensions));
    }

    #[test]
    fn test_find_files_recursive_no_files() {
        let directory: PathBuf = PathBuf::from("test_data/empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, true);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_recursive_some_files() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, true);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_recursive_subdirectory() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["png".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, true);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_recursive_multiple_extensions() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, true);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_find_files_non_recursive_no_files() {
        let directory: PathBuf = PathBuf::from("test_data/empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, false);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_non_recursive_some_files() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, false);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_find_files_non_recursive_subdirectory() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["png".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, false);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_find_files_non_recursive_multiple_extensions() {
        let directory: PathBuf = PathBuf::from("test_data/non_empty_directory");
        let extensions: Vec<String> = vec!["txt".to_string(), "jpg".to_string()];
        let result: Vec<PathBuf> = find_files(&directory, &extensions, false);
        assert_eq!(result.len(), 2);
    }

}