#!/bin/bash

cd compiler

# flag -fsanitize=address
clang++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp \
    -o ../build/carla \
    -I. \
    -L./libs/aarch64-linux \
    -leva \
    -Wl,-rpath,'$ORIGIN'

cp libs/aarch64-linux/libeva.so ../build/

cd ..
