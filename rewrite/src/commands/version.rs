use crate::cli::{CLI, Command};

const CARLA_VERSION: &'static str = "26.1";
const CARLA_PS: &'static str = "Rust rewrite";

impl Command {
    pub fn version(&self, _: &CLI) {
        println!("carla version {CARLA_VERSION} ({CARLA_PS})");
    }
}
