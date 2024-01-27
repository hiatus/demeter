use std::path::PathBuf;
use aws_sdk_s3::Client;

use std::io::Write;
use std::fs::{File, create_dir_all};

use regex::Regex;
use colored::Colorize;

// Local
use crate::demeter::style::*;
use crate::demeter::utils::*;


pub async fn harvest(client: &Client, bucket: &str, regexes: &Vec<Regex>, root: Option<&String>, max_size: Option<usize>) -> Result<usize, aws_sdk_s3::Error> {
	let mut count: usize = 0;
	let mut folder_printed = String::from("\\//");

	let response = client.list_objects_v2().bucket(bucket).send().await?;

	for object in response.contents() {
		let object_path = PathBuf::from(object.key().unwrap());

		let object_path_str = object_path.to_str().unwrap();
		let object_dirname_str = object_path.parent().unwrap().to_str().unwrap();
		let object_basename_str = object_path.file_name().unwrap().to_str().unwrap();

		let mut matched = false;

		for regex in regexes {
			if regex.is_match(&object_basename_str) {
				matched = true;
				break;
			}
		}

		if ! matched {
			continue;
		}

		if object_dirname_str.ne(folder_printed.as_str()) {
			print_folder(object_dirname_str, None);
			folder_printed = object_dirname_str.to_owned();
		}

		count += 1;

		if ! root.is_some() {
			print_file(&object_basename_str, None);
			continue;
		}

		if max_size.is_some() && object.size().unwrap() as usize > max_size.unwrap() {
			print_file_failed(
				&object_basename_str,
				Some(&format!(
					"skipping download: file size {} > {}",
					object.size.unwrap(), max_size.unwrap()
				).bright_cyan())
			);

			continue;
		}

		let mut copy_path = build_copy_path(
			&root.unwrap().to_string(), &object_path, object_path.components().count() - 1
		);

		match create_dir_all(&copy_path) {
			Ok(_) => {},
			Err(e) => {
				print_file_failed(
					object_path_str, 
					Some(&format!("failed to create {}: {}", copy_path, e)
				));
				continue;
			}
		}

		copy_path.push_str(object_basename_str);

		let mut object_output;

		match client.get_object()
				.bucket(bucket).key(object.key().unwrap()).send().await {
			Ok(o) => {
				object_output = o
			},
			Err(e) => {
				print_file_failed(
					object_basename_str,
					Some(&format!("download failed remotely: {}", e))
				);

				continue;
			}
		}

		match File::create(&copy_path) {
			Ok(f) => {
				let mut writer = f;

				while let Some(bytes) = object_output.body.try_next().await.unwrap() {
					writer.write_all(&bytes).unwrap();
				}

				print_file(object_basename_str, None);
			}

			Err(e) => {
				print_file_failed(
					object_basename_str,
					Some(&format!("download failed locally: {}", e))
				);
			}
		}
	}

	Ok(count)
}