fn main() {
    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 90, 90]);

    // let w = 64;
    let draw = |x, y| match (x % 2, y % 2) {
        (0, 0) => white,
        (1, 0) => red,
        (0, 1) => red,
        (1, 1) => white,
        (_, _) => panic!("error"),
    };

    let img = image::ImageBuffer::from_fn(10, 10, draw);

    match img.save("sample.png") {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    }
}
