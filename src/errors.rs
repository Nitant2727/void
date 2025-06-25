use std::fmt;
use std::process::exit;

#[derive(Debug)]
pub enum Errcode {
    ContainerError(u8),
    NotSupported(u8),
    ArguementInvalid(&'static str),
    SocketError(u8),
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArguementInvalid(element) => write!(f, "Arguement Invalid : {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exiting without any errors, returning 0");
            exit(0);
        }

        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
