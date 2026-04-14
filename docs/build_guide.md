Build Guide
Requirements
Rust (latest stable)
CMake
Vulkan/OpenGL drivers
Steps
Clone repository
Install dependencies
Build core: cargo build --release
Build client: cmake -B build && cmake --build build
Run

./build/run.sh

Notes
Ensure GPU drivers are installed
VR requires OpenXR runtime
