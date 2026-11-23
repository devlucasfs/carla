cargo build --release
cd carla-env
../target/release/rewrite build
../target/release/rewrite upload platform@esp32

cd ..
