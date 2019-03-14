use std::process::Command;
use std::env;

fn main() {
    let lua_dir = format!("{}/src/lua", env::current_dir().unwrap().display());

    // compile luacall library: OS dependant
    if cfg!(target_os = "linux") {
        // makefile is using a special env variable
        Command::new("make")
                .env("LUA_DIR", lua_dir)
                .args(&["-f", "src/lua/luacall_linux.mak"])
                .status()
                .expect("failed to make!");
    }
   

    println!(r"cargo:rustc-link-lib=static=lua");
    println!(r"cargo:rustc-link-lib=static=luacall");    
    println!(r"cargo:rustc-link-search=/home/m330421/projects/rbfrust/src/lua");
    println!(r"cargo:rustc-link-search=/usr/local/lib");    
}