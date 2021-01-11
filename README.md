# tibco-ems-sender-rs
Simple Windows Filesender for Tibco EMS, coded in Rust 

This repo needs tibco EMS libraries to build and to execute.  
Tibco EMS is protected under copywrite law and not included.  
Please download the Tibco EMS community client from the official website or use your own.

# Requirements
* Windows (at least for this guide)
* Tibco EMS client installation

## Build
* Clone repo and navigate into it
* Update the build.rs with your own values (see below)
* run ` cargo build `
* a new folder called target will have appeared /target/debug will host your exe

## Use
* go into target folder where your exe is located
* copy tibems.dll from %tibco_home%/bin into the same folder
* copy files from /samples into the same folder and config
* run exe from shell or doubleclick

## Environment Variables
| Optional? | Name | Value [one of] |
|---|---|---|
| required | ems_url | tcp://url:port |  
| required | ems_user | user |
| required | ems_password | userpw |
| required | ems_input_dest_name | mytopic |
| required | ems_input_dest_type | [topic, queue] |
| optional | RUST_LOG_STYLE | [always, never, auto] |
| optional | RUST_LOG | [error, warn, info, debug, trace] |
| optional | RUST_BACKTRACE | [1, full]

## example build.rs (for "cargo run")
To make your build easier, use a build.rs file in base folder
Paste in and config the following:

```rust
fn main () {
    // on windows
    // tibems.dll from 8.5/bin need to be copied into target/debug
    // to make cargo run work on windows
    println!("cargo:warning=You are using a build script! Values are prefilled");
    
    // build requirements
    println!("cargo:rustc-link-search=native=C:\\tibco\\ems\\8.5\\lib");

    // debug
    println!("cargo:rustc-env=RUST_LOG=debug");
    println!("cargo:rustc-env=RUST_LOG_STYLE=always");
    println!("cargo:rustc-env=RUST_BACKTRACE=1");

    // test config (FT/loadbalancer mode for ems_url not yet implemented in underlying rust-ems-lib)
    println!("cargo:rustc-env=ems_url=tcp://localhost:7222");
    println!("cargo:rustc-env=ems_user=testuser");
    println!("cargo:rustc-env=ems_password=testpassword");
    println!("cargo:rustc-env=ems_input_dest_name=mytopic");
    // either queue or topic
    println!("cargo:rustc-env=ems_input_dest_type=topic");
  }
```
