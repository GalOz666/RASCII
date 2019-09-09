use std::env::args;
use image;
use image::{DynamicImage, GrayImage};
use RASCII::{structs, initial_image_processing};
use tuikit::term::{Term, TermHeight};
use tuikit::event::{Event, Key};
use std::{thread, time};
use std::time::Duration;
use std::sync::mpsc::channel;
use RASCII::structs::Kernel;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    let thread_count: u32 = 10;
    // get path fron env
    let args: Vec<String> = args().collect();
    // create Dynamic image
    let kernel = structs::Kernel::new(9);
    assert_eq!(args.len(), 2 as usize, "file path was not provided!");
    let img: DynamicImage  =  image::open(&args[1]).unwrap();


    let cell = kernel.to_char_cell(&[0,0], &img, &ASCII_CHARS);
//    println!("{:?}, {:?}, {:?}, {:?},", cell.x, cell.y, cell.color, cell.ascii);
    let (rgb, grey, (width, height)) = initial_image_processing(&args[1], kernel);
    let term = Term::new().unwrap();



    let term = Term::with_height(TermHeight::Percent(100)).unwrap();
    term.show_cursor(false);
    // move to thread!
    let (tx, rx) = channel();
    for num in 1..thread_count+1 {
        let tx_c = tx.clone();
        let mut current_point = [num, 0 as u32];
        let kernel = Kernel::new(9);
        let width = Arc::new(width);
        let height = Arc::new(height);
        let img = Arc::new(&img);
        thread::spawn(move|| {
            loop {
                if current_point[1] > *height {
                    break
                }
                tx_c.send(kernel.to_char_cell(&current_point, *img, &ASCII_CHARS)).unwrap();
                current_point = if current_point[0]+kernel.kernel() > *width {
                    [current_point[0], current_point[1]+kernel.kernel()]
                } else {
                    [current_point[0]+kernel.kernel()*num, 0]
                };
            }

        });
    }

    for cell in rx.recv() {
        term.print_with_attr(cell.x, cell.y, &cell.ascii.to_string(), cell.color);

    }

    while let Ok(ev) = term.poll_event() {
    if let Event::Key(Key::Char('q')) = ev {
        break;
    }
        term.present();
}
 // write to screen at the correct sequence and\or save to file
}