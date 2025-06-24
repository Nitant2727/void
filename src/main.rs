mod cli;
use std::process::exit;
mod errors;
use errors::Errcode;

use crate::errors::exit_with_retcode;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(Ok(()))
        }

        Err(e) => {
            log::error!("Error while parsing arguements : \n \t {}", e);
            exit(e.get_retcode());
        }
    }
}
