use day_07::{get_calibration_results, get_calibration_results_with_concatenation};



#[test]
fn should_get_calibration_result() {
    assert_eq!(get_calibration_results("tests/resources/puzzle.txt"), 3749)
}

#[test]
fn should_get_calibration_result_with_concatenation() {
    assert_eq!(get_calibration_results_with_concatenation("tests/resources/puzzle.txt"), 11387)
}