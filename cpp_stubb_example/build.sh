#!/bin/bash
rm -rf build/*
cd build
cmake -DCMAKE_CXX_FLAGS=-m32 ..
make
mv cppmetastub.so ../../server/bin/cstrike/addons/cppmetastub
