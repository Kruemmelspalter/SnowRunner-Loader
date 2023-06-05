#![feature(once_cell_try)]
#![cfg(windows)]
#![windows_subsystem = "windows"]

use std::{error::Error, process::Command};

use dll_syringe::{process::OwnedProcess, Syringe};

use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_ALL_ACCESS};

fn main() -> Result<(), Box<dyn Error>> {
    assert!(cfg!(target_os = "windows"));

    let steamcmd = std::env::args().nth(1).expect("no command supplied");
    let dll = std::env::args().nth(2).expect("no dll supplied");

    let mut game_process = Command::new(steamcmd)
        .spawn()
        .expect("failed to spawn game process");
    let pid = game_process.id();

    let proc = OwnedProcess::from_pid(game_process.id()).expect("failed to create owned process");

    let syringe = Syringe::for_process(proc);

    let _module = syringe
        .inject(dll)
        .expect("failed to inject dll");

    ctrlc::set_handler(move || kill_proc(pid).expect("failed to kill process"))
        .expect("Failed to set CTRL+C handler");

    game_process
        .wait()
        .expect("failed to wait for game process to exit");

    Ok(())
}

fn kill_proc(pid: u32) -> Result<(), ()> {
    match Into::<bool>::into(unsafe {
        TerminateProcess(
            OpenProcess(PROCESS_ALL_ACCESS, true, pid).expect("failed to open process"),
            1,
        )
    }) {
        true => Ok(()),
        false => Err(()),
    }
}
