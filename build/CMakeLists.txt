cmake_minimum_required(VERSION 3.16)
project(MatrixNet)

set(CMAKE_CXX_STANDARD 17)

# Optional: integrate Rust build
add_custom_target(
    matrixnet_rust ALL
    COMMAND cargo build --manifest-path ${CMAKE_SOURCE_DIR}/build/Cargo.toml
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

# Future: add C++ renderer modules here
# add_subdirectory(renderer_cpp)

message(STATUS "MatrixNet build system initialized")
