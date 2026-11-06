use std::{env, ffi::{CString, c_char}};

use crate::{cli::{CLI, Command}, utils::compiler_outputs::CompilerOutputs};

unsafe extern "C" {
    fn __morgana_upload(
        target: *mut c_char,
        sake: *mut c_char,
        extensor: *mut c_char,
        baudrate: *mut c_char,
        device: *mut c_char,
    );
}


impl Command {
    pub fn upload(&self, params: &CLI) {
        let Ok(current_dir) = env::current_dir() else {
            CompilerOutputs::fatal("Can't read the current directory.");
            unreachable!();
        };

        let Command::Upload(extensor, device, baudrate) = &params.command else { unreachable!() };

        let target = current_dir.join("target").join(extensor);

        if !target.exists() || !target.is_dir() {
            println!("There is nothing to upload.");
            return;
        }

        let sake = current_dir.join("Sake");

        if !sake.exists() {
            CompilerOutputs::fatal_internal(
                "Can't reach Samar Eva's addapter file: That file is needed because into it has all compilation rules you are using for your peoject."
            );
        }

        let target   = CString::new(target.to_str().unwrap()).unwrap();
        let sake     = CString::new(sake.to_str().unwrap()).unwrap();
        let extensor = CString::new(extensor.as_str()).unwrap();
        let baudrate = CString::new(baudrate.as_str()).unwrap();
        let device   = CString::new(device.as_str()).unwrap();

        unsafe {
            __morgana_upload(
                target.as_ptr() as *mut c_char,
                sake.as_ptr() as *mut c_char,
                extensor.as_ptr() as *mut c_char,
                baudrate.as_ptr() as *mut c_char,
                device.as_ptr() as *mut c_char,
            );
        }
    }
}
