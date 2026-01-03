use asciilator::AsciilatorConfig;
use asciilator::convert_to_ascii;
use image::open;
fn main() {
    let image = open("test.jpg").expect("Cannot find file");

    let config = AsciilatorConfig {
        width: 60,
        scale_factor: 0.55,
    };
    let result = convert_to_ascii(&image, &config);

    println!("{}", result);
}
