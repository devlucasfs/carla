cargo build --release
cd carla-env
../target/release/rewrite build -useram

# echo
# echo
# echo "Uploading via Carla (./test.sh)"
# echo
# echo

# ../target/debug/rewrite upload avr_atmega328p
cd ..
