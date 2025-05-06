use std::process::Command;
use buildargs::{buildArgs};
use javapath::{getJavaPath};

fn LaunchGame() {
    let userNickname = String::new()
    let gameArgs = String::new()
    let javaPath = String::from(getJavaPath());
    let buildedArgs = String::from(buildArgs(userNickname, gameArgs));

    Command::new(javaPath)
    .arg(buildedArgs)
    .status()
    .expect("Failed to execute the game");
}
