use java_runtimes::detector;

fn getJavaPath() {
    return detector::detect_java_in_environments();
}

fn chooseJavaPath(pos: usize) {
    return String::from(getJavaPath().get(pos).unwrap());
}
