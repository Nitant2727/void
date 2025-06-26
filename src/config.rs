use crate::errors::Errcode;
use crate::ipc::generate_socketpair;

use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;
#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,

    pub fd: RawFd,
    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;
        log::debug!("Sockets created successfully: parent_fd={}, child_fd={}", sockets.0, sockets.1);

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].clone();

        Ok((
            ContainerOpts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
            },
            sockets,
        ))
    }
}
