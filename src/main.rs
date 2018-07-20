extern crate termion;
extern crate bmp;

use termion::{color, cursor, clear};
use std::thread;
use bmp::{Image, Pixel};



fn clear() {
    print!("{}", clear::All);
}

fn goto(x: u32, y: u32) {
    print!("{}", goto = cursor::Goto(x as u16, y as u16));
}

// todo print on full width


fn main() {
    let (width, height) = match termion::terminal_size() {
        Ok((width, height)) => (width / 2, height),
        Err(e) => {
            println!("Can not get terminal size - assuming min of 50x12");
            (50, 12)
        }
    };
    
    println!("width: {}\nheight: {}", width, height);

    // --------------
    let img = bmp::open("./in-img300.bmp").unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    let x_mult = img.get_width() / width as u32;
    let y_mult = img.get_height() / height as u32;

    let mut area: Vec<Vec<u32>> = vec![vec![1; width as usize]; height as usize];

    // thread::sleep_ms(400);
    for i in 0..width {
        for j in 0..height {
            println!("{:?}", (i, j));
            println!("{:?}", (i as u32 * x_mult, j as u32 * y_mult));
            area[j as usize][i as usize] = numberFromPixel(&img.get_pixel(i as u32 * x_mult, j as u32 * y_mult));
        }
    }
    render(& mut area);
}

fn render(area: & Vec<Vec<u32>>) {
    let x_offset = 1;
    let y_offset = 1;
    clear();
    goto(x_offset, y_offset);
    for (i, outer) in area.iter().enumerate() {
        for (j, inner) in outer.iter().enumerate() {
            print!("{}", get_graphic(inner));
        }
        goto(x_offset, 1 + y_offset + i as u32)
    }
    println!("");
}

fn get_graphic(num: &u32) -> String {
    match num {
        1 => String::from("\u{2591}\u{2591}"),
        2 => String::from("\u{2592}\u{2592}"),
        3 => String::from("\u{2593}\u{2593}"),
        4 => String::from("\u{2588}\u{2588}"),
        _ => String::from("  ")
    }
} 

fn numberFromPixel(px: &Pixel) -> u32 {
    (px.r as u32 + px.g as u32 + px.b as u32) / 154
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