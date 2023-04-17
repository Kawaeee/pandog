use std::path::PathBuf;
use std::process::Command;


pub fn convert_file(
    input_file_path: &PathBuf,
    input_format: &str,
    output_file_path: &PathBuf,
    output_format: &str,
) -> Result<PathBuf, std::io::Error> {
    let output = Command::new("pandoc")
        .args(&[
            input_file_path.to_str().unwrap(),
            "-f",
            input_format,
            "-t",
            output_format,
            "-o",
            output_file_path.to_str().unwrap(),
        ])
        .output()?;
        
    if output.status.success() {
        Ok(output_file_path.clone())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(std::io::Error::new(std::io::ErrorKind::Other, stderr))
    }
}
