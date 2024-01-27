use regex::Regex;
use tokio::runtime::Runtime;
use aws_sdk_s3::Client;
use aws_config::meta::region::RegionProviderChain;


// Local
mod harvest;

use crate::demeter::{args::DemeterCommandS3, style::*, utils::*};


pub fn main(args: DemeterCommandS3) -> i32 {
	let mut regexes: Vec<Regex> = Vec::<Regex>::new();

	if let Some(regex_str) = args.regex {
		match Regex::new(&regex_str) {
			Ok(regex) => {
				regexes.push(regex)
			}

			Err(_) => {
				print_error(&format!("Bad regex: {}", regex_str));
				return 1;
			}
		};
	}

	if let Some(regex_file) = args.regex_file {
		let regex_lines: Vec<String>;

		match read_file_lines(&regex_file) {
			Ok(lines) => {
				regex_lines = lines;
			},

			Err(e) => {
				print_error(&format!("Failed to read regex file: {}", &e.to_string()));
				return 1;
			}
		}

		for regex_str in regex_lines {
			match Regex::new(&regex_str) {
				Ok(regex) => {
					regexes.push(regex)
				}
	
				Err(_) => {
					print_error(&format!("Bad regex in file: '{}'", regex_str));
					return 1;
				}
			};
		}
	}

	if regexes.len() == 0 {
		print_error(&format!("No regular expressions to match"));
		return 1;
	}

	let rt = Runtime::new().unwrap();

	let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
	let config = rt.block_on(aws_config::from_env().region(region_provider).load());
	let client = Client::new(&config);

	let mut file_count = 0;

	match rt.block_on(harvest::harvest(&client, args.bucket.as_ref(), &regexes, args.output.as_ref(), args.size)) {
		Ok(count) => { file_count = count; },
		Err(e) => {
			print_error(&e.to_string())
		}
	}

	if file_count > 0 {
		println!();
		print_success(&format!("{} files matched", file_count));
	}
	else {
		print_fail("No files matched");
	}

	0
}