use crate::errors::Errcode;
use crate::{config::ContainerOpts, container::Container};
use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::any::Any;

const STACK_SIZE: usize = 1024 * 1024;

pub fn child(config: ContainerOpts) -> isize {
    log::info!(
        "Starting the container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    0
}

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    
    // Strategy 1: Try with user namespace first (works in WSL2)
    let mut user_flags = CloneFlags::empty();
    user_flags.insert(CloneFlags::CLONE_NEWUSER);
    user_flags.insert(CloneFlags::CLONE_NEWPID);
    user_flags.insert(CloneFlags::CLONE_NEWUTS);
    
    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        user_flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => {
            log::info!("Container created with user namespace isolation (WSL2 compatible)");
            return Ok(pid);
        }
        Err(e) => {
            log::debug!("User namespace failed: {:?}", e);
        }
    }
    
    // Strategy 2: Try full namespaces (for real Linux)
    let mut full_flags = CloneFlags::empty();
    full_flags.insert(CloneFlags::CLONE_NEWNS);
    full_flags.insert(CloneFlags::CLONE_NEWCGROUP);
    full_flags.insert(CloneFlags::CLONE_NEWPID);
    full_flags.insert(CloneFlags::CLONE_NEWIPC);
    full_flags.insert(CloneFlags::CLONE_NEWNET);
    full_flags.insert(CloneFlags::CLONE_NEWUTS);

    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        full_flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => {
            log::info!("Container created with full namespace isolation");
            Ok(pid)
        }
        Err(_) => {
            // Strategy 3: Fallback without namespaces
            log::warn!("All namespace attempts failed, running without isolation...");
            match clone(
                Box::new(|| child(config.clone())),
                &mut tmp_stack,
                CloneFlags::empty(),
                Some(Signal::SIGCHLD as i32),
            ) {
                Ok(pid) => {
                    log::warn!("Container running WITHOUT namespace isolation!");
                    Ok(pid)
                }
                Err(_) => Err(Errcode::ChildrenProcessError(0)),
            }
        }
    }
}
