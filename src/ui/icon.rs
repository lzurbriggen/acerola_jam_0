use macroquad::{miniquad::conf::Icon, prelude::*};

fn load_img(bytes: &'static [u8]) -> Image {
    return Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();
}

fn populate_array(img: Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
            array[index] = *value;
            index += 1;
        }
    }
}

pub fn set() -> Icon {
    let mut array_small: [u8; 1024] = [0; 1024];
    let mut array_medium: [u8; 4096] = [0; 4096];
    let mut array_big: [u8; 16384] = [0; 16384];

    populate_array(
        load_img(include_bytes!("../../assets/ui/icon_16.png")),
        &mut array_small,
    );
    populate_array(
        load_img(include_bytes!("../../assets/ui/icon_32.png")),
        &mut array_medium,
    );
    populate_array(
        load_img(include_bytes!("../../assets/ui/icon_64.png")),
        &mut array_big,
    );

    Icon {
        small: array_small,
        medium: array_medium,
        big: array_big,
    }
}
