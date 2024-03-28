use bardecoder;
use bardecoder::prepare::BlockedMean;

use image;

fn main() {
    // let img = image::open("test_qrcodes/qrtest10.png").unwrap();

    // // Use default decoder builder
    // let mut db = bardecoder::default_builder();

    // // Use some different arguments in one of the default components
    // db.prepare(Box::new(BlockedMean::new(4, 9)));

    // // Build the actual decoder
    // let decoder = db.build();

    // let results = decoder.decode(&img);
    // for result in results {
    //     println!("data :{}", result.unwrap());
    // }
}

/*
 db.prepare(Box::new(BlockedMean::new(7, 9)));

qrtest1 -> yes
qrtest2 -> no
qrtest3 -> no
qrtest4 -> no
qrtest5 -> no
qrtest6 -> no
qrtest7 -> no

*/

/*
 db.prepare(Box::new(BlockedMean::new(10, 10)));
 
qrtest1 -> yes
qrtest2 -> no
qrtest3 -> no
qrtest4 -> no
qrtest5 -> no
qrtest6 -> no
qrtest7 -> no

*/