use std::path::PathBuf;

// Returns None if the input is not valid UTF-8.
pub fn path_buf_to_str(input: &PathBuf) -> Option<&str> {
  input.as_path().to_str()
}
