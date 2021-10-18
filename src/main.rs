fn main() {
    // Image

    const IAMGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    // Render

    // Image width height
    println!("P3 {} {}", IAMGE_WIDTH, IMAGE_HEIGHT);
    // 255 for max color
    println!("{}", 255);
    // RGB triplets
    const WIDTH_FACTOR: f64 = 1.0 / (IAMGE_WIDTH as f64 - 1.0);
    const HEIGHT_FACTOR: f64 = 1.0 / (IMAGE_HEIGHT as f64 - 1.0);
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IAMGE_WIDTH {
            let r = i as f64 * WIDTH_FACTOR;
            let g = j as f64 * HEIGHT_FACTOR;
            let b = 0.25;

            let r = (255.999 * r) as u8;
            let g = (255.999 * g) as u8;
            let b = (255.999 * b) as u8;

            println!("{} {} {}", r, g, b);
        }
    }
}
