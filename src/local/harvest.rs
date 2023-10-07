use std::io::Error;
use std::fs::{self, create_dir_all};

use regex::Regex;
use colored::Colorize;

// Local
use crate::demeter::style::*;
use crate::demeter::utils::*;


pub fn harvest(folder: &str, regexes: &Vec<Regex>, root: Option<&String>, cur_depth: usize, max_depth: Option<usize>, max_size: Option<usize>) -> Result<usize, Error> {
	let mut count: usize = 0;
	let mut folder_printed: bool = false;

	for e in fs::read_dir(folder)? {
		let entry = e?;
		let filetype = entry.file_type()?;

		let entry_path = entry.path();
		let entry_filename = entry.file_name();

		let entry_path_str = entry_path.to_str().unwrap();
		let entry_file_str = entry_filename.to_str().unwrap();

		if filetype.is_dir() {
			if max_depth.is_some() && cur_depth >= max_depth.unwrap() {
				continue;
			}

			match harvest(entry_path_str, regexes, root, cur_depth + 1, max_depth, max_size) {
				Ok(c) => {
					count += c;
				}
				Err(e) => {
					print_folder_failed(
						entry_path_str, Some(&format!("Failed to open: {}", e))
					);
				}
			}

			continue;
		}

		if ! filetype.is_file() {
			continue;
		}

		let mut matched = false;

		for regex in regexes {
			if regex.is_match(entry_file_str) {
				matched = true;
				break;
			}
		}

		if ! matched {
			continue;
		}

		if ! folder_printed {
			print_folder(folder, None);
			folder_printed = true;
		}

		count += 1;

		if ! root.is_some() {
			print_file(entry_file_str, None);
			continue;
		}

		let entry_size: usize = entry.metadata().unwrap().len() as usize;

		if max_size.is_some() && entry_size > max_size.unwrap() {
			print_file_failed(
				entry_file_str,
				Some(&format!(
					"skipping copy: file size {} > {}",
					entry_size.to_string().bright_cyan(),
					max_size.unwrap().to_string().bright_cyan()
				))
			);

			continue;
		}

		let mut copy_path = build_copy_path(
			&root.unwrap().to_string(), &entry_path, cur_depth
		);

		match create_dir_all(&copy_path) {
			Ok(_) => {},
			Err(e) => {
				print_file_failed(
					entry_path_str, 
					Some(&format!("failed to create {}: {}", copy_path, e)
				));
				continue;
			}
		}

		copy_path.push_str(entry_file_str);

		match fs::copy(entry_path_str, copy_path) {
			Ok(_) => {
				print_file(entry_file_str, None);
			},
			Err(e) => {
				print_file_failed(entry_file_str, Some(&format!("copy failed: {}", e)));
			}
		}
	}

	Ok(count)
}