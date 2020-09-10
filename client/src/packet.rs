use crate::alphabet::Alphabet;
use io::Write;
use std::{io, net::TcpStream};

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;
struct Pixel<'t> {
    x: u32,
    y: u32,
    color: &'t [u8],
}

// const SCREEN_DIM_MAX_LEN: usize = 4;
const MIN_COMMAND_SIZE: usize = 14; // "PX X Y CCCCCC\n"
const MAX_COMMAND_SIZE: usize = 20; // "PX XXXX YYYY CCCCCC\n"

pub struct Packet<A>
where
    A: Alphabet,
{
    data: Vec<u8>,
    data_i: usize,
    xoffset: u32,
    yoffset: u32,
    scale: u32,
    noise_start: usize,
    noise_end: usize,
    alphabet: A,
    //buf: [u8; SCREEN_DIM_MAX_LEN],
}

impl<A> Packet<A>
where
    A: Alphabet,
{
    pub fn new(scale: u32) -> Packet<A> {
        Packet {
            data: vec![0; MAX_COMMAND_SIZE * SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize],
            data_i: 0,
            xoffset: 0,
            yoffset: 0,
            scale,
            noise_start: 0,
            noise_end: 0,
            alphabet: Alphabet::new(),
            //buf: [0 as u8; SCREEN_DIM_MAX_LEN],
        }
    }

    fn write_slice_and_update_index(&mut self, slice: &[u8], i: &mut usize) {
        let len = slice.len();
        self.data[*i..*i + len].copy_from_slice(slice);
        *i += len;
    }

    fn write_color_and_update_index(&mut self, rgb: u32, i: &mut usize) {
        for shift in 0..6 {
            let mut val = ((rgb >> (shift * 4)) & 0xf) + 48;
            if val > 57 {
                val += 7;
            }
            self.data[*i + shift] = val as u8;
        }
        *i += 6;
    }

    fn write_num_and_update_index(&mut self, num: usize, i: &mut usize) {
        let thousands = (num / 1000) as u8;
        let hundreds = ((num % 1000) / 100) as u8;
        let tens = ((num % 100) / 10) as u8;
        let ones = (num % 10) as u8;
        if thousands != 0 {
            self.data[*i] = thousands + 48;
            self.data[*i + 1] = hundreds + 48;
            self.data[*i + 2] = tens + 48;
            *i += 3;
        } else if hundreds != 0 {
            self.data[*i] = hundreds + 48;
            self.data[*i + 1] = tens + 48;
            *i += 2;
        } else if tens != 0 {
            self.data[*i] = tens + 48;
            *i += 1;
        }
        self.data[*i] = ones + 48;
        *i += 1;
    }

    fn get_rand_color(&mut self) -> u32 {
        return rand::random::<u32>();
    }

    fn get_next_command(&mut self, start: &mut usize) {
        // From min to max command index (looking for end of command)
        for j in MIN_COMMAND_SIZE - 1..MAX_COMMAND_SIZE - 1 {
            if self.data[*start + j] == '\n' as u8 {
                *start += j + 1;
                return;
            }
        }
    }

    fn recalculate_pixel_noise(&mut self, i: &mut usize) {
        self.get_next_command(i);
        *i -= 7;
        let rgb = self.get_rand_color();
        self.write_color_and_update_index(rgb, i);
        *i += 1;
    }

    pub fn recalculate_noise(&mut self) {
        let mut i = self.noise_start;
        while i < self.noise_end {
            self.recalculate_pixel_noise(&mut i);
        }
    }

    pub fn add_area(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32, color: u32) {
        let mut i = self.data_i;
        for x in start_x..end_x {
            for y in start_y..end_y {
                self.add_pixel(x, y, color, &mut i);
            }
        }
        self.data_i = i;
    }

    pub fn add_noise(&mut self, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
        let mut i = self.noise_end;
        for x in start_x..end_x {
            for y in start_y..end_y {
                let rgb = self.get_rand_color();
                self.add_pixel(x, y, rgb, &mut i);
            }
        }

        self.data_i += i - self.noise_end;
        self.noise_end = i;
    }

    pub fn extend_noise(&mut self, pixels: u32) {
        let mut i = self.noise_end;
        for j in 0..pixels {
            self.get_next_command(&mut i);
        }
        if i <= self.data_i {
            self.noise_end = i;
        } else {
            panic!("Trying to extend noise beyond commands")
        }
    }

    fn add_pixel_no_overwrite(&mut self, x: u32, y: u32, rgb: u32) {
        let mut i = self.data_i;
        self.add_pixel(x, y, rgb, &mut i);
        self.data_i = i;
    }

    //#[inline(never)]
    fn add_pixel(&mut self, x: u32, y: u32, rgb: u32, i: &mut usize) {
        self.write_slice_and_update_index(b"PX ", i);
        self.write_num_and_update_index(x as usize, i);
        self.write_slice_and_update_index(b" ", i);
        self.write_num_and_update_index(y as usize, i);
        self.write_slice_and_update_index(b" ", i);
        self.write_color_and_update_index(rgb, i);
        self.write_slice_and_update_index(b"\n", i);
    }

    pub fn add_letter(&mut self, c: char) {
        let scale_y: u32 = self.scale;
        let scale_x: u32 = self.scale;
        let width: u32 = 8;
        let height: u32 = 8;
        let mut data_i = self.data_i;
        //let array = char_to_pixel_positions(c).expect(&format!("Invalid char: {}", c));
        for y in 0..height {
            //print!("{}\n", termion::style::Reset);
            for x in 0..width {
                let i = x + y * width;
                //let array_contains_i = array.binary_search(&i).is_ok();
                let array_contains_i = self.alphabet.is_pixel_in_char(c, i as u8);
                for zy in 0..scale_y {
                    for zx in 0..scale_x {
                        let x = zx + self.xoffset + x * scale_x;
                        let y = zy + self.yoffset + y * scale_y;
                        let mut rgb = 0x000000;
                        if !array_contains_i {
                            rgb = self.get_rand_color();
                        }
                        //println!("Color: {:x}", rgb);
                        self.add_pixel(x, y, rgb, &mut data_i);
                    }
                }
            }
            //print!("{}\n", termion::style::Reset)
        }
        self.data_i = data_i;
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
#[cfg(test)]
mod tests {
    use crate::alphabet_bitarr::BitMapAlphabet;

    use super::*;

    #[test]
    fn test_non_random() {
        let mut packet_to_build = Packet::<BitMapAlphabet>::new(8);
        packet_to_build.add_string("▓▓▓▓▓▓▓▓▓▓▓");

        let mut sum = 0u64;
        for x in packet_to_build.data.iter() {
            sum += *x as u64;
        }
        assert_eq!(sum, 36315648)
    }
}
