mod cli;
use std::process::exit;
mod errors;
use errors::Errcode;
mod child;
mod config;
mod container;
use crate::errors::exit_with_retcode;
#[macro_use]
extern crate scan_fmt;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        }

        Err(e) => {
            log::error!("Error while parsing arguements : \n \t {}", e);
            exit(e.get_retcode());
        }
    }
}
