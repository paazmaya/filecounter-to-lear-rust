use assert_cmd::Command;

#[test]
fn test_cli_basic() {
    let mut cmd = Command::cargo_bin("filecounter").expect("Failed to build filecounter binary");

    cmd.arg("-r");
    cmd.arg("test_data");

    cmd.assert()
        .success()
        .stdout("Total files found: 3\n");
}

#[test]
fn test_cli_also_txt() {
    let mut cmd = Command::cargo_bin("filecounter").expect("Failed to build filecounter binary");

    cmd.arg("-r");
    cmd.arg("-e");
    cmd.arg("txt");
    cmd.arg("test_data");

    cmd.assert()
        .success()
        .stdout("Total files found: 4\n");
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("filecounter").expect("Failed to build filecounter binary");

    cmd.arg("-V");

    cmd.assert()
        .success()
        .stdout("filecounter 0.1.0\n");
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("filecounter").expect("Failed to build filecounter binary");

    cmd.arg("-h");

    cmd.assert()
        .success()
        .to_string()
        .contains("Command line tool to count files");
}