use regex::Regex;
use pavao::{SmbClient, SmbCredentials, SmbOptions};

// Local
mod harvest;

use crate::demeter::{args::DemeterCommandSmb, style::*, utils::*};


pub fn main(args: DemeterCommandSmb) -> i32 {
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

	let client: SmbClient = SmbClient::new(
		SmbCredentials::default()
			.server(args.target).workgroup(args.domain).share(args.share)
			.username(args.username).password(args.password),
		SmbOptions::default()
			.case_sensitive(true).one_share_per_server(true),
	).unwrap();

	let mut file_count = 0;

	match harvest::harvest(&client, "/", &regexes, args.output.as_ref(), 0, args.depth, args.size) {
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