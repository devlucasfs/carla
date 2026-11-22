cargo build --release
cd carla-env
../target/release/rewrite build --extra-xtensa_lx7 esp32
../target/release/rewrite upload xtensa_lx7 --mcu esp32

cd ..
