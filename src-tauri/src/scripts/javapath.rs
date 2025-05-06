use java_runtimes::detector;

fn getJavaPath() {
    let runtimes = detector::detect_java_in_environments();
    return runtimes
}

fn chooseJavaPath(pos: usize) {
    let javaPath = String::from(getJavaPath().get(pos).unwrap());
    return javaPath
}