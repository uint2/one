cmake_minimum_required (VERSION 3.12)
set(CMAKE_EXPORT_COMPILE_COMMANDS YES)
# cmake_policy(SET CMP0135 NEW)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Build in Debug mode by default
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug)
endif()
set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

project(gitnu VERSION 0.0.0)

add_library(libgitnu
  src/lib.cpp
)

add_executable(git-nu
  src/main.cpp
  src/git_cmd.cpp
)

add_executable(gitnu_tests
  src/tests.cpp
)

include_directories(mini include)
target_link_libraries(git-nu PUBLIC libgitnu)
target_link_libraries(gitnu_tests PUBLIC libgitnu)
