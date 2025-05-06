use java_runtimes::detector;

fn getJavaPath() {
    let javaRuntimes = detector::detect_java_in_environments();
    return javaRuntimes
}

fn chooseJavaPath(pos: usize) {
    let javaPath = String::from(getJavaPath().get(pos).unwrap());
    return javaPath
}
