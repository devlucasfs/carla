use crate::cli::{Build, BuildPattern};

pub fn get(b: &Build) -> (String, String) {
    if let Build::Pattern(BuildPattern { arch, os }) = b {
        return ( arch.clone(), os.clone() )
    }

    (std::env::consts::ARCH.into(), std::env::consts::OS.into())
}
