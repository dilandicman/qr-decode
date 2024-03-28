use rxing;

fn main() {
    let file_name = "test_qrcodes/qrtest8.png";

    let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decodes");

    for result in results {
        println!("{} -> {}", result.getBarcodeFormat(), result.getText())
    }
}

/*
Files supported

- 
*/