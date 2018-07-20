extern crate termion;
extern crate bmp;

use termion::{color, cursor, clear};
use std::thread;
use bmp::{Image, Pixel};



fn clear() {
    print!("{}", clear::All);
}

fn goto(x: u16, y: u16) {
    print!("{}", goto = cursor::Goto(x, y));
}

// todo print on full width


fn main() {
    // --------------
    let img = bmp::open("./in-img300.bmp").unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    let x_mult = img.get_width() / 10;
    let y_mult = img.get_height() / 10;

    let mut area: [[u16; 10]; 10] = [[1; 10]; 10];

    // render(& mut area);
    thread::sleep_ms(400);
    for i in 0..10 {
        for j in 0..10 {
            // if i % 2 == j % 2 { area[i][j] = 0 };
            area[j][i] = numberFromPixel(&img.get_pixel(i as u32 * x_mult, j as u32 * y_mult));
        }
    }
    render(& mut area);
}

fn render(area: & [[u16; 10]; 10]) {
    let x_offset = 5;
    let y_offset = 3;
    clear();
    goto(x_offset, y_offset);
    for (i, outer) in area.iter().enumerate() {
        for (j, inner) in outer.iter().enumerate() {
            print!("{}", get_graphic(inner));
        }
        goto(x_offset, 1 + y_offset + i as u16)
    }


    // println!("{:?}", area);
}

fn get_graphic(num: &u16) -> String {
    match num {
        1 => String::from("\u{2591}\u{2591}"),
        2 => String::from("\u{2592}\u{2592}"),
        3 => String::from("\u{2593}\u{2593}"),
        4 => String::from("\u{2588}\u{2588}"),
        _ => String::from("  ")
    }
} 

fn numberFromPixel(px: &Pixel) -> u16 {
    (px.r as u16 + px.g as u16 + px.b as u16) / 154
}

    // let mut img = Image::new(10, 10);
    
    // let pixel = img.get_pixel(0, 0);

    // for i in 0..10 {
    //     for j in 0..10 {
    //         if i % 2 == j % 2 { img.set_pixel(i, j, Pixel::new(255, 255, 255)); };
    //     }
    // }

    // save
    // let _ = img.save("./img.bmp");