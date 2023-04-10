use std::process::Command;

#[macro_export]
macro_rules! success_tests {
    ($($name:ident: $expected:literal,)*) => {
        $(
        #[test]
        fn $name() {
            $crate::infra::run_success_test(stringify!($name), $expected);
        }
        )*
    }
}
#[macro_export]
macro_rules! error_tests {
    ($($name:ident: $expected:literal,)*) => {
        $(
        #[test]
        fn $name() {
            $crate::infra::run_error_test(stringify!($name), $expected);
        }
        )*
    }
}

fn compile(name: &str) -> Result<(), String> {
    let prog_name = format!("tests/{name}.snek");
    let asm_name = format!("tests/{name}.s");
    let exe_name = format!("tests/{name}.run");

    // Compile the compiler
    let status = Command::new("cargo")
        .arg("build")
        .status()
        .expect("Could not run Cargo");
    assert!(status.success(), "Compiling the compiler failed");

    // Run the compiler
    let output = Command::new("target/debug/boa")
        .arg(&prog_name)
        .arg(&asm_name)
        .output()
        .expect("Could not run the compiler");
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap());
    }

    // Assemble and link
    let status = Command::new("make")
        .arg(&exe_name)
        .status()
        .expect("Could not run make");
    assert!(status.success(), "Linking failed");

    Ok(())
}

pub(crate) fn run_success_test(name: &str, expected: &str) {
    compile(name).expect("There was a compiler error");

    let exe_name = format!("tests/{name}.run");
    let output = Command::new(&exe_name).output().unwrap();
    assert!(
        output.status.success(),
        "Running the compiled program had an error: {}",
        std::str::from_utf8(&output.stderr).unwrap(),
    );
    let actual_output = String::from_utf8(output.stdout).unwrap();
    let actual_output = actual_output.trim();
    let expected_output = expected.trim();
    if expected_output != actual_output {
        eprintln!(
            "Output differed!\n{}",
            prettydiff::diff_lines(actual_output, expected_output)
        );
        panic!("Test failed");
    }
}

pub(crate) fn run_error_test(name: &str, expected: &str) {
    let Err(actual_err) = compile(name) else {
        panic!("Expected a failure, but compilation succeeded");
    };
    assert!(
        actual_err.contains(expected.trim()),
        "The reported error message does not match",
    );
}
