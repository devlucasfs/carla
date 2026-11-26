use crate::utils::compiler_outputs::CompilerOutputs;
#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};

use std::{env, ffi::{CString, c_char}};

unsafe extern "C" {
    fn __morgana_upload(
        target: *mut c_char,
        sake: *mut c_char,
        extensor: *mut c_char,
        baudrate: *mut c_char,
        device: *mut c_char,
        mcu: *mut c_char,
    );
}

#[cfg(any(target_os = "linux", target_os = "android"))]
const DEFAULT_DEVICE: &str = "/dev/ttyUSB0";

#[cfg(target_os = "windows")]
const DEFAULT_DEVICE: &str = "COM3";

#[cfg(target_os = "macos")]
const DEFAULT_DEVICE: &str = "/dev/tty.usbserial";

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
const DEFAULT_DEVICE: &str = "/dev/cuaU0";

#[cfg(target_os = "openbsd")]
const DEFAULT_DEVICE: &str = "/dev/cua00";

#[cfg(target_os = "netbsd")]
const DEFAULT_DEVICE: &str = "/dev/dtyU0";

#[cfg(target_os = "illumos")]
const DEFAULT_DEVICE: &str = "/dev/term/a";

#[cfg(target_os = "haiku")]
const DEFAULT_DEVICE: &str = "/dev/ports/usb";

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "windows",
    target_os = "macos",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "illumos",
    target_os = "haiku",
)))]
const DEFAULT_DEVICE: &str = "/dev/ttyUSB0";

const DEFAULT_BAUDRATE: &str = "115200";

command_fn!(upload(args) {
    o_extensor: String = "target",
    o_baudrate: String = "baudrate",
    o_device:   String = "device",
    o_mcu:      String = "mcu",
} => {
    let baudrate = o_baudrate.unwrap_or_else(|| DEFAULT_BAUDRATE.to_string());
    let device   = o_device.unwrap_or_else(|| DEFAULT_DEVICE.to_string());
    let extensor = o_extensor.unwrap_or_else(|| {
        CompilerOutputs::fatal("Upload failed! Extensor flag was not found.");
        unreachable!()
    });

    let mcu = o_mcu.unwrap_or_else(|| {
        CompilerOutputs::fatal("Upload failed! MCU flag was not found.");
        unreachable!()
    });

    let Ok(current_dir) = env::current_dir() else {
        CompilerOutputs::fatal("Can't read the current directory.");
        unreachable!();
    };

    let target = current_dir.join("target").join(&extensor);

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
    let mcu      = CString::new(mcu.as_str()).unwrap();

    unsafe {
        __morgana_upload(
            target.as_ptr() as *mut c_char,
            sake.as_ptr() as *mut c_char,
            extensor.as_ptr() as *mut c_char,
            baudrate.as_ptr() as *mut c_char,
            device.as_ptr() as *mut c_char,
            mcu.as_ptr() as *mut c_char,
        );
    }
});

pub const UPLOAD: Command = command! {
    name: "upload",
    description: "Flash your program to your MCU",
    since: FIRST_VERSION_EVER,
    example: "carla upload -t xtensa_lx7 -m esp32",
    flags: {
        Evaluated("baudrate", "b") => "MCU baudrate",
        Evaluated("mcu", "m") => "Board name",
        Evaluated("device", "d") => "Upload device",
        Evaluated("target", "t") => "Target archtecture"
    },
    function: upload,
};
