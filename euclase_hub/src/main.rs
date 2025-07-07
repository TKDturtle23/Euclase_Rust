use std::process::Command;
use euclase_vulkan::hello_from_lib;

fn main() {
    println!("App2 says: {}", hello_from_lib());

    let status = Command::new("cargo")
        .arg("run")
        .arg("--package")
        .arg("euclase")
        .status()
        .expect("Failed to launch app1");

    println!("App1 exited with: {:?}", status);
}
