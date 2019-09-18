use std::env::args;
use image;
use image::{DynamicImage, GrayImage};
use RASCII::{structs, initial_image_processing};
use tuikit::term::{Term, TermHeight};
use tuikit::event::{Event, Key};
use std::{thread, time};
use std::sync::mpsc::channel;
use RASCII::structs::{Kernel, CharCell};
use std::sync::Arc;
use tuikit::attr::*;
use std::time::{Duration, Instant};
use tuikit::screen::Screen;
use tuikit::canvas::Canvas;

const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    let args: Vec<String> = args().collect();
    let thread_count: u32 = match args.get(2) {
        Some(num) => num.parse().unwrap(),
        None => 4
    };
    let kernel = structs::Kernel::new(9);
    assert!(args.len() as usize >= 2, "file path was not provided!");

    let (rgb, (width, height)) = initial_image_processing(&args[1], &kernel);

    // move to thread!
    let (tx, rx) = channel();
    let width = Arc::new(width);
    let height = Arc::new(height);
    let img = Arc::new(rgb);
    let mut handles = Vec::with_capacity(thread_count as usize);

    for num in 0..thread_count {
        let tx_c = tx.clone();
        let mut current_point = [0, num * kernel.kernel()];
        let kernel = Kernel::new(9);
        let width = Arc::clone(&width);
        let height = Arc::clone(&height);
        let img = Arc::clone(&img);
        handles.push(thread::spawn(move || {
            while current_point[1] < *height {
//                println!("thread number: {:?} | point proceesed {:?}\n w:{:?}, h:{:?}" ,num, current_point, width, height);

                let cell = kernel.to_char_cell(&current_point, &*img, &ASCII_CHARS);
//
                tx_c.send(cell).unwrap();

                current_point = if current_point[0] + kernel.kernel() < *width {
                    [current_point[0] + kernel.kernel(), current_point[1]]
                } else {
                    [0, current_point[1] + (kernel.kernel())]
                };
            }
        }));
    }
    drop(tx);
    let mut term = Term::new().unwrap();
    let _ = term.show_cursor(false);
//    println!("time for reciev!");
    let mut cells: Vec<CharCell> = Vec::new();

    for cell in rx {
        cells.push(cell);
//        for p_cell in &cells {
//            let _ = term.print_with_attr(p_cell.x, p_cell.y, &p_cell.ascii.to_string(),
//                                         Attr {
//                                             fg: p_cell.color,
//                                             bg: Color::BLACK,
//                                             ..Attr::default()
//                                         }).unwrap();
//        }
//        let _ = term.present();
        }

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev  {
            break;
        }
        for p_cell in &cells {
            let _ = term.print_with_attr(p_cell.x, p_cell.y, &p_cell.ascii.to_string(),
                                         Attr {
                                             fg: p_cell.color,
                                             bg: Color::BLACK,
                                             ..Attr::default()
                                         }).unwrap();
        }
        let _ = term.present();

    }
}
