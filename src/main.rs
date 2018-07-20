extern crate termion;
extern crate bmp;

use termion::{color, cursor, clear};
use std::thread;
use std::env;
use bmp::{Image, Pixel};



fn clear() {
    print!("{}", clear::All);
}

fn goto(x: u32, y: u32) {
    print!("{}", goto = cursor::Goto(x as u16, y as u16));
}

fn set_color_bw() {
    print!("{}{}", color::Fg(color::White), color::Bg(color::Black));
}

fn reset_color() {
    print!("{}{}", color::Fg(color::Reset), color::Bg(color::Reset));
}
// todo black bg / white fg
// add colors
// add more unicode characters 
// add more file supports 

fn main() {
    let (ter_width, ter_height) = match termion::terminal_size() {
        Ok((ter_width, ter_height)) => (ter_width / 2, ter_height),
        Err(e) => {
            println!("Can not get terminal size - assuming min of 50x12");
            (50, 12)
        }
    };
    println!("terminal-ter_width: {}\nterminal-ter_height: {}", ter_width, ter_height);

    // --------------
    let args: Vec<String> = env::args().collect();
    println!("Arguments {:?}", args);
    let img = bmp::open(&args[1]).unwrap_or_else(|e| {
    // let img = bmp::open(env::args().collect().1).unwrap_or_else(|e| {
        panic!("Failed to open: {}", e);
    });

    let x_mult = img.get_width() as f32 / ter_width as f32;
    let y_mult = img.get_height() as f32 / ter_height as f32;

    let mult = if x_mult >= y_mult { x_mult } else { y_mult };

    let render_width = ( img.get_width() as f32 / mult ) as u32;
    let render_height = ( img.get_height() as f32 / mult ) as u32;

    println!("render width {} render height {}", render_width, render_height);

    let mut area: Vec<Vec<u32>> = vec![vec![1; render_width as usize]; render_height as usize];

    for i in 0..render_width {
        for j in 0..render_height {
            area[j as usize][i as usize] = numberFromPixel(&img.get_pixel((i as f32 * mult) as u32, (j as f32 * mult) as u32));
        }
    }
    render(& mut area);
}

fn render(area: & Vec<Vec<u32>>) {
    let x_offset = 1;
    let y_offset = 1;
    clear();
    set_color_bw();
    goto(x_offset, y_offset);
    for (i, outer) in area.iter().enumerate() {
        for (j, inner) in outer.iter().enumerate() {
            print!("{}", get_graphic(inner));
        }
        goto(x_offset, 1 + y_offset + i as u32)
    }
    reset_color();
    // println!("");
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