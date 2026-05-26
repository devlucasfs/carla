#!/bin/bash

cd compiler

# flag -fsanitize=address
clang++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp \
    -o ../build/carla \
    -I. \
    -L./libs/aarch64-android \
    -leva \
    -Wl,-rpath,'$ORIGIN'

cp libs/aarch64-android/libeva.so ../build/

cd ..
