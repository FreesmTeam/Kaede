use std::process::Command;
use buildargs::{buildArgs};
use javapath::{chooseJavaPath, getJavaPath};

fn LaunchGame() {
    let mut pos: usize;
    let userNickname = String::new();
    let gameArgs = String::new();
    let javaPath = String::new(chooseJavaPath(pos));
    let buildedArgs = String::from(buildArgs(userNickname, gameArgs));

    Command::new(javaPath)
    .arg(buildedArgs)
    .status()
    .expect("Failed to execute the game");
}
