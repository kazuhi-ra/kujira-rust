fn main() {
    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 90, 90]);

    let length = 120;
    let draw = |x, y| {
        let block_length = length / 10;
        let (xi, yi) = (x / block_length, y / block_length);

        return match (xi % 2, yi % 2) {
            (0, 0) => white,
            (1, 0) => red,
            (0, 1) => red,
            (1, 1) => white,
            (_, _) => panic!("error"),
        };
    };

    let img = image::ImageBuffer::from_fn(length, length, draw);

    img.save("sample.png").unwrap();
}
