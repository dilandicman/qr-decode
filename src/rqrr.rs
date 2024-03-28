use image; // need latest package verion 0.25
use rqrr;

// this is the best use this

/*
    TODO: Check how to the qr code payload is strcutured
    TODO: After getting the structure extract the passkey set from the qr
    TODO: Save the passkey set to the sm in a secured manner
*/

fn main() {
    let img = image::open("test_qrcodes/qrtest9.jpg");
    let image = img.unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(image);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // Decode the grid
    let content = grids[0].decode();

    println!("Decoded data: {:?}", content);

}

/*
    Qr testing
    - qrtest1.jpg -> yes
    - qrtest2.jpg -> yes
    - qrtest3.jpg -> yes
    - qrtest4.jpg -> no
    - qrtest5.jpg -> no
    - qrtest6.jpg -> yes
    - qrtest7.jpg -> no
    - qrtest8.jpg -> yes

*/