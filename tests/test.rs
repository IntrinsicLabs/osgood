#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn integration() {
        let child = Command::new("npm")
            .arg("install")
            .current_dir("tests/integration/server")
            .status()
            .unwrap();
        assert!(child.success());

        let child = Command::new("node")
            .arg("--no-warnings")
            .arg("tests/integration/test")
            .status()
            .unwrap();
        assert!(child.success());
    }

    #[test]
    fn format() {
        let child = Command::new("cargo")
            .args(&["fmt", "--all", "--", "--check"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .unwrap();
        assert!(child.success());
    }
}
