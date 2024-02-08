use image::{Rgb, RgbImage};

pub fn dither(img: &mut RgbImage,quant_per_color: u8) {
    for y in 0..(img.height()) {
        for x in 0..(img.width()) {
            let pix = img.get_pixel_mut_checked(x, y).expect("outside image error");
            let new = get_closest_color(&pix, quant_per_color);
            let err = Rgb([
                pix.0[0] - new.0[0],
                pix.0[1] - new.0[1],
                pix.0[2] - new.0[2]
            ]);
            *pix = new;
            match img.get_pixel_mut_checked(x + 1, y) {
                Some(p) => { 
                    add_two_rgb(p, &err, 7, 16)
                }
                None => (),
            };
            if x != 0{
                match img.get_pixel_mut_checked(x - 1, y + 1) {
                    Some(p) => { 
                        add_two_rgb(p, &err, 3, 16)
                    }
                    None => (),
                };
            }
            match img.get_pixel_mut_checked(x, y + 1) {
                Some(p) => { 
                    add_two_rgb(p, &err, 5, 16)
                }
                None => (),
            };
            match img.get_pixel_mut_checked(x + 1, y + 1) {
                Some(p) => { 
                    add_two_rgb(p, &err, 1, 16)
                }
                None => (),
            };

        }
    }
}

fn get_closest_color(pixel: &Rgb<u8>, quant: u8) -> Rgb<u8> {
    let r = (((pixel.0[0] as u16) * (quant as u16) / 255) * 255 / quant as u16) as u8;
    let g = (((pixel.0[1] as u16) * (quant as u16) / 255) * 255 / quant as u16) as u8;
    let b = (((pixel.0[2] as u16) * (quant as u16) / 255) * 255 / quant as u16) as u8;
    Rgb([r,g,b])
}

fn add_to_rgb(pixel: &mut u8, other_pixel: &u8, numerator: &i32, denominator: &i32) {
    let store = *pixel as i32 + (*other_pixel as i32 * *numerator / *denominator);
    if store < 0 {
        *pixel = 0;
    }else if store > 255 {
        *pixel = 255;
    }else {
        *pixel = store as u8;
    }
}

fn add_two_rgb(pixel: &mut Rgb<u8>, other_pixel: &Rgb<u8>, numerator: i32, denominator: i32) {
    add_to_rgb(&mut pixel.0[0], &other_pixel.0[0], &numerator, &denominator);
    add_to_rgb(&mut pixel.0[1], &other_pixel.0[1], &numerator, &denominator);
    add_to_rgb(&mut pixel.0[2], &other_pixel.0[2], &numerator, &denominator);
}
