use std::process::exit;

use clap::Parser;

// Local
mod demeter;
use demeter::args::*;

mod local;
mod smb;
mod s3;


fn main() {
    let ret: i32;
    let cli_args = DemeterArgs::parse();

    match cli_args.command {
        DemeterCommand::Local(args) => {
            ret = local::main(args);
        },

        DemeterCommand::Smb(args) => {
            ret = smb::main(args);
        },

        DemeterCommand::S3(args) => {
            ret = s3::main(args);
        }
    }

    exit(ret);
}