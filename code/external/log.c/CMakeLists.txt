cmake_minimum_required(VERSION 3.14)

project(rxi-log C)

set(CMAKE_C_STANDARD 99)

add_library(rxi-log src/log.c)
target_include_directories(rxi-log PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src")
