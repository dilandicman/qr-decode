use image::io::Reader as ImageReader;

fn main() {
    println!("Reading qr code --- \n");

    // open the image from disk
    let img = ImageReader::open("test_qrcodes/qrtest10.png").unwrap();

    // convert to gray scale
    let img_gray = img.decode().unwrap().to_luma8();

    // show the grey scale image
    img_gray.save("test_qrcodes/greyscale/qrtest8.jpg").unwrap();

    // create a decoder
    let mut decoder = quircs::Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);

    

    for code in codes {
        let code = code.expect("failed to extract qr code");
        let decoded = code.decode().expect("failed to decode qr code");
        println!("Decoded data length: {}", &decoded.payload.len());
        let payload_list = &decoded.payload;
        for i in 0..payload_list.len() {
            println!("Decoded data: {}", &payload_list[i]);
        }
        println!("qrcode: {}", std::str::from_utf8(&decoded.payload).unwrap());
    }


    /*
        test output
        - qrtest1.jpg -> yes
        - qrtest2.jpg -> yes
        - qrtest3.jpg -> yes
        - qrtest4.jpg -> no
        - qrtest5.jpg -> no
        - qrtest6.jpg -> no
        - qrtest7.jpg -> no
        - qrtest8.jpg -> no
     */

}
