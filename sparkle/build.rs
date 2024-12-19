use std::env;

fn main() {
    // Link against PyTorch C++ libraries
    println!("cargo:rustc-link-search=native=C:\\vcpkg\\installed\\x64-windows\\lib");
    println!("cargo:rustc-link-lib=dylib=torch");
    println!("cargo:rustc-link-lib=dylib=c10");
    
    // Link against OpenCV
    println!("cargo:rustc-link-lib=dylib=opencv_world480");
    
    // Set environment variables
    println!("cargo:rerun-if-env-changed=TORCH_CUDA_VERSION");
    println!("cargo:rerun-if-env-changed=TORCH_CUDA_ARCH_LIST");
} 