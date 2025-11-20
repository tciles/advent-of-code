use std::path::PathBuf;

pub trait Exercise<T> {
    /// Public
    fn new() -> Self;
    fn question_one(&self, base_dir: &PathBuf);
    fn question_two(&self, base_dir: &PathBuf);
}

pub(crate) trait Ops<T> {
    /// Private
    fn get_input_file_path(base_dir: &PathBuf) -> PathBuf;
    fn read_lines(file_path: &str) -> T;
}
