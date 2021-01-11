# tibco-ems-sender-rs
Simple Filesender for Tibco EMS coded in Rust

This repo needs tibco EMS libraries to build and to execute.
Tibco EMS is protected under copywrite law and not included.
Please download the Tibco EMS community client from the official website or use your own.

# Requirements
* Tibco EMS installation

## Build
* Clone repo and dir into it
* Update the build.rs with your own values (see below)
* run ` cargo build `

## Use
* go into target folder where your exe is located
* copy tibems.dll from %tibco_home%/lib into the same folder
* copy content from samples folder into the same folder
* config samples
* run exe from shell or doubleclick

## Environment Variables
---|---|---
required | ems_url | tcp://url:port
required | ems_user | user
required | ems_password | userpw
required | ems_input_dest_name | mytopic
required | ems_input_dest_type | [topic, queue]
optional | RUST_LOG_STYLE | [always, never, auto]
optional | RUST_LOG | [error, warn, info, debug, trace]

## example build.rs (for "cargo run")
To make your building easier, use a build.rs file in base folder
Paste the following in the following and config to your needs.

```rust
fn main () {
    // on windows
    // tibems.dll and from 8.5/bin need to be copied into target/debug
    // to make cargo run work on windows
    println!("cargo:warning=You are using a build script! Values are prefilled");
    
    // build requirements
    println!("cargo:rustc-link-search=native=C:\\tibco\\ems\\8.5\\lib");

    // debug
    println!("cargo:rustc-env=RUST_LOG=debug");
    println!("cargo:rustc-env=RUST_LOG_STYLE=always");
    println!("cargo:rustc-env=RUST_BACKTRACE=1");

    // test config (FT mode not yet implemented in underlying rust ems lib)
    println!("cargo:rustc-env=ems_url=tcp://localhost:7222");
    println!("cargo:rustc-env=ems_user=testuser");
    println!("cargo:rustc-env=ems_password=testpassword");
    println!("cargo:rustc-env=ems_input_dest_name=mytopic");
    // either queue or topic
    println!("cargo:rustc-env=ems_input_dest_type=topic");
  }
```