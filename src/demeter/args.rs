use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct DemeterArgs {
	#[clap(subcommand)]
	pub command: DemeterCommand
}

#[derive(Debug, Subcommand)]
pub enum DemeterCommand {
	/// Harvest files in the local filesystem
	Local(DemeterCommandLocal),
	/// Harvest files in SMB shares
	Smb(DemeterCommandSmb),
	/// Harvest files in S3 buckets
	S3(DemeterCommandS3)
}

#[derive(Debug, Args)]
pub struct DemeterCommandLocal {
	#[clap(short, long, required = false, help = "Regular expression for matching filenames")]
	pub regex: Option<String>,
	#[clap(short = 'R', long, required = false, help = "Read regular expressions from a file")]
	pub regex_file: Option<String>,
	#[clap(short, long, required = false, help = "Maximum recursion depth")]
	pub depth: Option<usize>,
	#[clap(short, long, required = false, help = "Maximum file size for copying")]
	pub size: Option<usize>,
	#[clap(
		short = 'o', long, required = false,
		help = "Save harvested files and directories in the [output] folder (omit to run in listing mode)"
	)]
	pub output: Option<String>,

	#[clap(required = true, help = "Target folder")]
	pub folder: String
}

#[derive(Debug, Args)]
pub struct DemeterCommandSmb {
	#[clap(short, long, required = true, help = "Username for authentication")]
	pub username: String,
	#[clap(short, long, required = true, help = "Password for authentication")]
	pub password: String,
	#[clap(short = 'D', long, required = true, help = "Workgroup/domain for authentication")]
	pub domain: String,
	#[clap(short = 'S', long, required = true, help = "Share name")]
	pub share: String,
	#[clap(short, long, required = false, help = "Regular expression for matching filenames")]
	pub regex: Option<String>,
	#[clap(short = 'R', long, required = false, help = "Read regular expressions from a file")]
	pub regex_file: Option<String>,
	#[clap(short, long, required = false, help = "Maximum recursion depth")]
	pub depth: Option<usize>,
	#[clap(short, long, required = false, help = "Maximum download file size")]
	pub size: Option<usize>,
	#[clap(
		short = 'o', long, required = false,
		help = "Save harvested files and directories in the [output] folder (omit to run in listing mode)"
	)]
	pub output: Option<String>,

	#[clap(required = true, help = "Target host: smb://PC01.contoso.local[:port]")]
	pub target: String
}

#[derive(Debug, Args)]
pub struct DemeterCommandS3 {
	#[clap(short, long, required = false, help = "Regular expression for matching object filenames")]
	pub regex: Option<String>,
	#[clap(short = 'R', long, required = false, help = "Read regular expressions from a file")]
	pub regex_file: Option<String>,
	#[clap(short, long, required = false, help = "Maximum download file size")]
	pub size: Option<usize>,
	#[clap(
		short = 'o', long, required = false,
		help = "Save harvested files and directories in the [output] folder (omit to run in listing mode)"
	)]
	pub output: Option<String>,

	#[clap(required = true, help = "Target bucket")]
	pub bucket: String
}