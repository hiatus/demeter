use colored::Colorize;


pub fn print_success(s: &str) {
	println!("{}{}{} {}", "[".bright_black(), "+".bright_cyan(), "]".bright_black(), s);
}

pub fn print_fail(s: &str) {
	println!("{}{}{} {}", "[".bright_black(), "-".bright_red(), "]".bright_black(), s);
}

pub fn print_error(s: &str) {
	println!("{}{}{} {}", "[".bright_black(), "!".bright_red(), "]".bright_black(), s);
}

pub fn print_folder(path: &str, comment: Option<&str>) {
	let mut s: String = String::from(
		format!("{} {}", "=>".white(), path.white().bold())
	);

	if let Some(c) = comment {
		s.push_str(&format!(" ({})", c));
	}

	println!("{}", &s);
}


pub fn print_file(filename: &str, comment: Option<&str>) {
	let mut s: String = String::from(
		format!("   {}", filename.bright_cyan())
	);

	if let Some(c) = comment {
		s.push_str(&format!(" ({})", c));
	}

	println!("{}", &s);
}

pub fn print_folder_failed(path: &str, comment: Option<&str>) {
	let mut s: String = String::from(
		format!("{} {}", "=>".bright_red(), path.white().bold())
	);

	if let Some(c) = comment {
		s.push_str(&format!(" ({})", c));
	}

	println!("{}", &s);
}

pub fn print_file_failed(filename: &str, comment: Option<&str>) {
	let mut s: String = String::from(
		format!("   {}", filename.bright_red())
	);

	if let Some(c) = comment {
		s.push_str(&format!(" ({})", c));
	}

	println!("{}", &s);
}