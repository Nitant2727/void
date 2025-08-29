use std::fmt;
use std::process::exit;

use crate::container::MINIMAL_KERNEL_VERSION;

#[derive(Debug)]
pub enum Errcode{
    ArgumentInvalid(&'static str),
    NotSupported(u8),
    SocketError(u8),
    ContainerError(u8),
    ChildProcessError(u8),
    MountsError(u8),
    NamespacesError(u8),
    HostnameError(u8),
    CapabilitiesError(u8),
    SyscallsError(u8),
    ResourcesError(u8),
    RngError
}

impl Errcode{
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}


#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self{

            Errcode::NotSupported(errtype) => {
                match errtype {
                    0 => write!(f, "Minimal kernel version required: {}", MINIMAL_KERNEL_VERSION),
                    1 => write!(f, "Only x86_64 architecture is supported"),
                    _ => write!(f, "{:?} (unknown id)", self),
                }
            },

            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),

            _ => write!(f, "{:?}", self) // For any variant not previously covered
        }
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        },

        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
