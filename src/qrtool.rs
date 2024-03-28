use std::process::Command;
use std::io::Write;

fn main() {
    // Assuming you have a Micro QR Code image in a file named "micro_qr.png"
    let image_path = "test_qrcodes/microqr8.png";

    // Build the qrtool command
    let output = Command::new("qrtool")
                         .arg("decode")
                         .arg(image_path)
                         .output()
                         .expect("Failed to execute 'qrtool' process");

    // Check if the command was successful
    if output.status.success() {
        let decoded_data = String::from_utf8_lossy(&output.stdout);
        println!("Decoded data: {}", decoded_data);
    } else {
        println!("qrtool failed. Error: {:?}", output.stderr);
    }
}

// use qrtool::QrCode;
// use std::path::Path;

// fn main() {
//     let image_path = Path::new("path/to/your/image.png");
//     let qr_code = QrCode::new();
//     let result = qr_code.decode(image_path);

//     match result {
//         Ok(data) => println!("QR Code data: {}", data),
//         Err(e) => println!("Error decoding QR Code: {}", e),
//     }
// }
