use day_07::get_calibration_results;



#[test]
fn should_get_calibration_result() {
    assert_eq!(get_calibration_results("tests/resources/puzzle.txt"), 3749)
}