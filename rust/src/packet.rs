use crate::alphabet_fast::Alphabet;
use io::Write;
use numtoa::NumToA;
use std::{io, net::TcpStream};

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;
struct Pixel<'t> {
    x: u32,
    y: u32,
    color: &'t [u8],
}

const SCREEN_DIM_MAX_LEN: usize = 4;
const COMMAND_SIZE: usize = 19; // "PX XXXX YYY CCCCCC\n"

pub struct Packet {
    data: Vec<u8>,
    data_i: usize,
    xoffset: u32,
    yoffset: u32,
    scale: u32,
    alphabet: Alphabet,
    //buf: [u8; SCREEN_DIM_MAX_LEN],
}

impl Packet {
    pub fn new(scale: u32) -> Packet {
        Packet {
            data: vec![0; COMMAND_SIZE * SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize],
            data_i: 0,
            xoffset: 0,
            yoffset: 0,
            scale,
            alphabet: Alphabet::new(),
            //buf: [0 as u8; SCREEN_DIM_MAX_LEN],
        }
    }

    fn write_and_update_index(&mut self, slice: &[u8], i: &mut usize) {
        let len = slice.len();
        self.data[*i..*i + len].copy_from_slice(slice);
        *i += len;
    }

    fn write_num_and_update_index(&mut self, num: usize, i: &mut usize) {
        // 1234 / 1000 = 1
        // 1234 % 1000 = 234 / 100 = 2
        let th = (num / 1000) as u8;
        let h = ((num % 1000) / 100) as u8;
        let te = ((num % 100) / 10) as u8;
        let o = (num % 10) as u8;
        if th != 0 {
            self.data[*i] = th + 48;
            self.data[*i + 1] = h + 48;
            self.data[*i + 2] = te + 48;
            *i += 3;
        } else if h != 0 {
            self.data[*i] = h + 48;
            self.data[*i + 1] = te + 48;
            *i += 2;
        } else if te != 0 {
            self.data[*i] = te + 48;
            *i += 1;
        }
        self.data[*i] = o + 48;
        *i += 1;
    }

    //#[inline(never)]
    fn add_pixel(&mut self, pixel: Pixel) {
        let mut i = self.data_i;
        self.write_and_update_index(b"PX ", &mut i);
        // let tmp = pixel.x.numtoa(10, &mut self.data[i..]);
        // i += tmp.len();
        self.write_num_and_update_index(pixel.x as usize, &mut i);
        self.write_and_update_index(b" ", &mut i);
        // let tmp = pixel.y.numtoa(10, &mut self.data[i..]);
        // i += tmp.len();
        self.write_num_and_update_index(pixel.y as usize, &mut i);
        self.write_and_update_index(b" ", &mut i);
        self.write_and_update_index(pixel.color, &mut i);
        self.write_and_update_index(b"\n", &mut i);
        self.data_i = i;
    }

    pub fn add_letter(&mut self, c: char) {
        let scale_y: u32 = self.scale;
        let scale_x: u32 = self.scale;
        let width: u32 = 8;
        let height: u32 = 8;
        //let array = char_to_pixel_positions(c).expect(&format!("Invalid char: {}", c));
        for y in 0..height {
            //print!("{}\n", termion::style::Reset);
            for x in 0..width {
                let i = x + y * width;
                //let array_contains_i = array.binary_search(&i).is_ok();
                let array_contains_i = self.alphabet.is_pixel_in_char(c, i as u8);
                for zy in 0..scale_y {
                    for zx in 0..scale_x {
                        let color: &mut [u8] = &mut ['0' as u8; 6];
                        let x = zx + self.xoffset + x * scale_x;
                        let y = zy + self.yoffset + y * scale_y;
                        if !array_contains_i {
                            let rand_num = rand::random::<u32>();
                            (rand_num & 0xff).numtoa(16, &mut color[0..2]);
                            ((rand_num >> 8) & 0xff).numtoa(16, &mut color[2..4]);
                            ((rand_num >> 8 * 2) & 0xff).numtoa(16, &mut color[4..6]);
                        }
                        let pixel = Pixel { x, y, color };
                        self.add_pixel(pixel)
                        //print!(" ");
                    }
                }
            }
            //print!("{}\n", termion::style::Reset)
        }
        self.xoffset += width * self.scale;
        if self.xoffset + width * self.scale > SCREEN_WIDTH {
            self.xoffset = 0;
            self.yoffset += height * self.scale;
        }
    }

    pub fn add_string(&mut self, s: &str) {
        for c in s.chars() {
            self.add_letter(c);
        }
    }

    pub fn write(&mut self, stream: &mut TcpStream) -> io::Result<usize> {
        // println!(
        //     "{:?}",
        //     String::from_utf8(self.data[0..self.data_i].to_vec())
        // );
        stream.write(&self.data[0..self.data_i])
    }

    pub fn reset(&mut self) {
        self.xoffset = 0;
        self.yoffset = 0;
        self.data_i = 0;
    }
}
