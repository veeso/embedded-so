cmake_minimum_required(VERSION 3.10)

project(libfoo C)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

include_directories(include)

set(SOURCES src/libfoo.c)

add_library(foo_shared SHARED ${SOURCES})
set_target_properties(foo_shared PROPERTIES OUTPUT_NAME "foo")
target_include_directories(foo_shared PUBLIC include)

add_library(foo_static STATIC ${SOURCES})
set_target_properties(foo_static PROPERTIES OUTPUT_NAME "foo")
target_include_directories(foo_static PUBLIC include)
