#!/bin/bash

cd compiler

# flag -fsanitize=address
g++ -std=c++17 -g -O0 -fPIC -fpermissive -fexceptions main.cpp \
    -o ../build/carla \
    -I. \
    -L./libs/x86_64-linux \
    -leva \
    -Wl,-rpath,'$ORIGIN'

cp libs/x86_64-linux/libeva.so ../build/

cd ..
