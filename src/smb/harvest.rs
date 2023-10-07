use std::path::PathBuf;
use pavao::{SmbClient, SmbDirent, SmbDirentType, SmbOpenOptions, SmbError};

use std::io;
use std::fs::{File, create_dir_all};

use regex::Regex;
use colored::Colorize;

// Local
use crate::demeter::style::*;
use crate::demeter::utils::*;


fn entity_uri(entity: &SmbDirent, path: &str) -> String {
	let mut p = PathBuf::from(path);

	p.push(PathBuf::from(entity.name()));
	p.as_path().to_string_lossy().to_string()
}

pub fn harvest(client: &SmbClient, folder: &str, regexes: &Vec<Regex>, root: Option<&String>, cur_depth: usize, max_depth: Option<usize>, max_size: Option<usize>) -> Result<usize, SmbError> {
	let mut count: usize = 0;
	let mut folder_printed: bool = false;

	for entity in client.list_dir(folder)?.into_iter() {
		let entity_path = PathBuf::from(entity_uri(&entity, folder));
		let entity_file_str = entity.name();
		let entity_path_str = entity_path.to_str().unwrap();

		if entity.get_type() == SmbDirentType::Dir {
			if max_depth.is_some() && cur_depth >= max_depth.unwrap() {
				continue;
			}

			match harvest(client, entity_path_str, regexes, root, cur_depth + 1, max_depth, max_size) {
				Ok(c) => {
					count += c;
				}

				Err(e) => {
					print_folder_failed(
						entity_path_str, Some(&format!("Failed to open: {}", e))
					);
				}
			}

			continue;
		}

		if entity.get_type() != SmbDirentType::File {
			continue;
		}

		let mut matched = false;

		for regex in regexes {
			if regex.is_match(&entity_file_str) {
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
			print_file(&entity_file_str, None);
			continue;
		}

		let entity_stat = client.stat(&entity_path_str).unwrap();

		if max_size.is_some() && entity_stat.size as usize > max_size.unwrap() {
			print_file_failed(
				&entity_file_str,
				Some(&format!(
						"skipping download: file size {} > {}",
						entity_stat.size.to_string().bright_cyan(),
						max_size.unwrap().to_string().bright_cyan()
				))
			);

			continue;
		}

		let mut copy_path = build_copy_path(
			&root.unwrap().to_string(), &entity_path, cur_depth
		);

		match create_dir_all(&copy_path) {
			Ok(_) => {},
			Err(e) => {
				print_file_failed(
					entity_path_str, 
					Some(&format!("failed to create {}: {}", copy_path, e)
				));
				continue;
			}
		}

		copy_path.push_str(entity_file_str);

		let mut reader = client.open_with(
			entity_path_str,
			SmbOpenOptions::default().read(true),
		).unwrap();


		match File::create(&copy_path) {
			Ok(f) => {
				let mut writer = f;

				match io::copy(&mut reader, &mut writer) {
					Ok(_) => {
						print_file(entity_file_str, None);
					}
					Err(e) => {
						print_file_failed(
							entity_file_str,
							Some(&format!("download failed remotely: {}", e))
						);
					}
				}
			}

			Err(e) => {
				print_file_failed(
					entity_file_str,
					Some(&format!("download failed locally: {}", e))
				);
			}
		}
	}

	Ok(count)
}