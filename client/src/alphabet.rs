pub trait Alphabet {
    fn new() -> Self;
    fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool;
}
