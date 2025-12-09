cmake_minimum_required(VERSION 4.1)
project(day8)

set(CMAKE_CXX_STANDARD 26)

add_executable(day8 main.cpp)

configure_file(test-input.txt test-input.txt COPYONLY)
configure_file(input.txt input.txt COPYONLY)
