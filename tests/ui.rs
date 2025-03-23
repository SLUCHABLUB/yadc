use trybuild::TestCases;

#[test]
fn ui() {
    let cases = TestCases::new();
    cases.pass("tests/ui/debug.rs")
}