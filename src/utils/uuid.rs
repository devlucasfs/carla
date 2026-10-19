use std::{
    collections::HashSet,
    sync::{Mutex, OnceLock},
};

#[cfg(windows)]
fn random_bytes(buf: &mut [u8]) {
    use std::ffi::c_void;

    #[link(name = "bcrypt")]
    unsafe extern "system" {
        fn BCryptGenRandom(algorithm: *mut c_void, buffer: *mut u8, length: u32, flags: u32)
        -> i32;
    }

    const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x0000_0002;

    let status = unsafe {
        BCryptGenRandom(
            std::ptr::null_mut(),
            buf.as_mut_ptr(),
            buf.len() as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        )
    };

    assert!(status == 0, "BCryptGenRandom failed (NTSTATUS {status:#x})");
}

#[cfg(not(windows))]
fn random_bytes(buf: &mut [u8]) {
    use std::{fs::File, io::Read};
    File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(buf))
        .expect("failed to read /dev/urandom");
}

pub fn gen_hex(bits: usize) -> String {
    assert!(bits > 0, "gen_hex needs at least 1 bit");

    static ISSUED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    let mut issued = ISSUED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());

    let digits = (bits + 3) / 4;
    let mut bytes = vec![0u8; (digits + 1) / 2];
    for _ in 0..1_000_000 {
        random_bytes(&mut bytes);
        let mut out = String::with_capacity(digits);
        for i in 0..digits {
            let byte = bytes[i / 2];
            let mut n = if i % 2 == 0 { byte >> 4 } else { byte & 0x0f };

            if i == 0 && bits % 4 != 0 {
                n &= (1u8 << (bits % 4)) - 1;
            }

            out.push(char::from_digit(n as u32, 16).unwrap());
        }

        if issued.insert(out.clone()) {
            return out;
        }
    }

    panic!("gen_hex: espaço de {bits} bits esgotado");
}
