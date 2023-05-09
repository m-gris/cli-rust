use std::fs; 
use assert_cmd::Command;
use predicates::prelude::*;

// Create Type Alias
// the Ok part will only ever hold the unit type 
// the Err part can hold anything implementing the std::error::Error trait. 
// this error will live inside a smart pointer called a Box
type TestResult = Result < () , Box < dyn std::error::Error> >;

#[test]
fn dies_no_args() -> TestResult { 

    // nota: below  the ? is used to unpack an Ok value or propagate an Err
    let mut cmd = Command::cargo_bin("echor")?; 

    // run the program with NO ARGS
    // and assert that it fails and print a usage statement to STDERR
    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("USAGE"));
    Ok(())

}

fn run(args: &[&str], expected_file: &str)-> TestResult { 
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult { 
    run(&["Hello there"], "tests/expected/hello1.txt")
}








#[test]
fn hello2() -> TestResult { 
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult { 
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}



#[test]
fn hello2_no_new_line() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
