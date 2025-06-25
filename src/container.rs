use crate::cli::Args;
use crate::config::ContainerOpts;
use crate::errors::Errcode;
use nix::sys::utsname::uname;
use nix::unistd::close;
use std::os::unix::io::RawFd;

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = uname();
    log::debug!("Linux Release : {}", host.release());

    if let Ok(version) = scan_fmt!(host.release(), "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::NotSupported(0));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(Errcode::NotSupported(1));
    }

    Ok(())
}

pub struct Container {
    config: ContainerOpts,
    sockets: (RawFd, RawFd),
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Container { config, sockets })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        log::debug!("Cleaning container");
        if let Err(e) = close(self.socket.0) {
            log::error!("Unable to write socket: {:?}", e);
            return Err(Errcode::SocketError(3));
        }
        if let Err(e) = close(self.socket.1) {
            log::error!("Unable to close to read socket : {:?}", e);
            return Err(Errcode::SocketError(4));
        }
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error while creating container : {:?}", e);
        return Err(e);
    }
    log::debug!("Finished, cleaning and exit");
    container.clean_exit()
}
