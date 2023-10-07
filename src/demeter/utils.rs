use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};


pub fn read_file_lines(path: &str) -> Result<Vec<String>, io::Error> {
	let mut lines = Vec::<String>::new();
	let file = File::open(path)?;

	for line in io::BufReader::new(file).lines() {
		lines.push(line?);
	}

	Ok(lines)
}

pub fn folder_exists(path: &str) -> bool {
	let folder_path = Path::new(path);
	folder_path.exists() && folder_path.is_dir()
}

pub fn build_copy_path(path_root_str: &str, path_entry: &PathBuf, cur_depth: usize) -> String {
	let parent_path = path_entry.parent().unwrap();
	let partial_path = path_entry.parent().unwrap()
		.iter().skip(parent_path.components().count() - cur_depth).collect::<PathBuf>();

	let partial_path_str = partial_path.to_str().unwrap();
	let mut copy_path = path_root_str.to_string();

	copy_path.push(MAIN_SEPARATOR);

	if partial_path_str.len() > 0 {
		copy_path.push_str(partial_path_str);
		copy_path.push(MAIN_SEPARATOR);
	}

	copy_path
}