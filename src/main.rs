use day_07::{get_calibration_results, get_calibration_results_with_concatenation};

fn main() {

    let calibration_result = get_calibration_results("src/resources/puzzle.txt");
    //6083020304036
    println!("Calibration result: {calibration_result}");

    let calibration_result = get_calibration_results_with_concatenation("src/resources/puzzle.txt");
    //59002246504791
    println!("Calibration result: {calibration_result}");
}
