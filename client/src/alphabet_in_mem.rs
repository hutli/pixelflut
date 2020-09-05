pub struct Alphabet {
    buf: [bool; 64],
}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet { buf: [false; 64] }
    }

    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool {
        match c {
            ' ' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            '"' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '#' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, true, true, true, true, true, true, true, true,
                    false, true, true, false, false, true, true, false, true, true, true, true,
                    true, true, true, true, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '$' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    true, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, true, true, false, false, true, true, true, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '%' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    false, false, false, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '&' => {
                self.buf = [
                    false, false, true, true, true, false, false, false, false, true, true, false,
                    true, true, false, false, false, true, true, false, true, true, false, false,
                    false, false, true, true, true, false, true, true, false, true, true, false,
                    true, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, false, true, true, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '\'' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            '(' => {
                self.buf = [
                    false, false, false, false, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            ')' => {
                self.buf = [
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '*' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, true, true,
                    false, false, true, true, false, false, false, true, true, true, true, false,
                    false, true, true, true, true, true, true, true, true, false, false, true,
                    true, true, true, false, false, false, true, true, false, false, true, true,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '+' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, true, true, true, true, true, true, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            ',' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, true, false, false, false, false,
                ]
            }
            '-' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, true, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            '.' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, false, false, false, false,
                ]
            }
            '/' => {
                self.buf = [
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, true, true, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '0' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, true, true, true, false,
                    false, true, true, true, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '1' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '2' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, true, true, true, false, false, false, false, true, true,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '3' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '4' => {
                self.buf = [
                    false, false, false, false, true, true, true, false, false, false, false, true,
                    true, true, true, false, false, false, true, true, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, true,
                    true, true, true, true, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '5' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, false, false, false, false, true, true, true, true, true, false, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '6' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '7' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '8' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            '9' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, true, false, false, false, false, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            ':' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            ';' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, true, true, false, false, false, false,
                ]
            }
            '<' => {
                self.buf = [
                    false, false, false, false, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '=' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, true, true, true,
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, true, true, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '>' => {
                self.buf = [
                    false, true, true, false, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, true, true, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '?' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '@' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, true, true, true, false,
                    false, true, true, false, true, true, true, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, true, true,
                    false, false, true, true, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'A' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'B' => {
                self.buf = [
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'C' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'D' => {
                self.buf = [
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'E' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, true, true, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'F' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, true, true, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            'G' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, true, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'H' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'I' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'J' => {
                self.buf = [
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'K' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, true, true, false, false,
                    false, true, true, true, true, false, false, false, false, true, true, false,
                    true, true, false, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'L' => {
                self.buf = [
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'M' => {
                self.buf = [
                    false, true, true, false, false, false, true, true, false, true, true, true,
                    false, true, true, true, false, true, true, true, true, true, true, true,
                    false, true, true, false, true, false, true, true, false, true, true, false,
                    false, false, true, true, false, true, true, false, false, false, true, true,
                    false, true, true, false, false, false, true, true, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'N' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, true,
                    false, true, true, false, false, true, true, true, true, true, true, false,
                    false, true, true, false, true, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'O' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'P' => {
                self.buf = [
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'Q' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, false, true, true, true, true, false, false,
                    false, false, false, false, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'R' => {
                self.buf = [
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'S' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'T' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            'U' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'V' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, false, true, true, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'W' => {
                self.buf = [
                    false, true, true, false, false, false, true, true, false, true, true, false,
                    false, false, true, true, false, true, true, false, false, false, true, true,
                    false, true, true, false, true, false, true, true, false, true, true, true,
                    true, true, true, true, false, true, true, true, false, true, true, true,
                    false, true, true, false, false, false, true, true, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'X' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, false, true, true, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    true, true, false, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'Y' => {
                self.buf = [
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, false, true, true, true, true, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'Z' => {
                self.buf = [
                    false, true, true, true, true, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '[' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '\\' => {
                self.buf = [
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, false,
                ]
            }
            ']' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '^' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    true, true, false, false, false, true, true, false, false, true, true, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '_' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, true, true, true, true, true, true, true, true,
                ]
            }
            '`' => {
                self.buf = [
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            'a' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    true, true, true, true, true, false, false, true, true, false, false, true,
                    true, false, false, false, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'b' => {
                self.buf = [
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, true, true, true, false, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, true, true, true, false, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'c' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'd' => {
                self.buf = [
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, true, true, true, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'e' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    true, true, true, true, false, false, true, true, false, false, false, false,
                    false, false, false, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'f' => {
                self.buf = [
                    false, false, false, true, true, true, false, false, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, true, true, true, true, true, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            'g' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, false, true, true, true, true, true,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    true, true, true, false, false,
                ]
            }
            'h' => {
                self.buf = [
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, true, true, true, false, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'i' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, true, true, true, true, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            'j' => {
                self.buf = [
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, true, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, true, true, true, false, false, false,
                ]
            }
            'k' => {
                self.buf = [
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, true, true, true,
                    true, true, false, false, false, true, true, false, false, true, true, false,
                    false, true, true, false, false, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            'l' => {
                self.buf = [
                    false, false, true, true, true, false, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'm' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, false, true, true,
                    false, false, true, true, true, true, true, true, true, false, true, true,
                    false, true, false, true, true, false, true, true, false, true, false, true,
                    true, false, true, true, false, false, false, true, true, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'n' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'o' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'p' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, true, true, true, false,
                    false, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false,
                ]
            }
            'q' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, false, true, true, true, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false,
                ]
            }
            'r' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            's' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, true, true,
                    false, false, true, true, false, false, false, false, false, false, false,
                    true, true, true, true, false, false, false, false, false, false, false, true,
                    true, false, false, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            't' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, true, true, true, true, true, true,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, true, true, true, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            'u' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'v' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, false, true, true, true, true, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'w' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, false,
                    true, true, false, true, true, false, true, false, true, true, false, true,
                    true, false, true, false, true, true, false, false, true, true, true, true,
                    true, false, false, false, true, true, false, true, true, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            'x' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, true, true, false,
                    false, false, true, true, false, false, true, true, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            'y' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, false, true, true, true, true, true,
                    false, false, false, false, false, true, true, false, false, false, true, true,
                    true, true, false, false, false,
                ]
            }
            'z' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, true, true, true, true,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, true, true, false, false, false, false, false, true, true, false, false,
                    false, false, false, true, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '{' => {
                self.buf = [
                    false, false, false, false, true, true, true, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, true, true, true, false, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, true, true, true, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '|' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '}' => {
                self.buf = [
                    false, true, true, true, false, false, false, false, false, false, false, true,
                    true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, true, true, true, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, true, true, true, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '~' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, false, false,
                    true, true, false, true, true, false, true, false, true, true, false, true,
                    true, false, false, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false,
                ]
            }
            '◂' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false,
                ]
            }
            '▸' => {
                self.buf = [
                    false, false, false, false, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, true, false, false,
                    false, false, true, true, true, true, false, false, false, false, false, true,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, false, true, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '▴' => {
                self.buf = [
                    false, false, false, true, false, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, true, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '▾' => {
                self.buf = [
                    false, false, false, false, true, false, false, false, false, false, false,
                    true, true, true, false, false, false, false, true, true, true, true, true,
                    false, false, true, true, true, true, true, true, true, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, true, true, true, true, true, true, true, false, false,
                    true, true, true, true, true, false, false, false, false, true, true, true,
                    false, false, false, false, false, false, true, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, true, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, true, true, false, false, false, true, true, true, true, true, true,
                    false, false, true, true, true, true, true, true, false, false, true, true,
                    true, true, true, true, false, false, true, true, true, true, true, true,
                    false, false, false, true, true, true, true, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '┌' => {
                self.buf = [
                    false, false, true, true, true, true, false, false, false, true, true, true,
                    true, true, true, false, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, false, true, true,
                    true, true, true, true, false, false, false, true, true, true, true, false,
                    false,
                ]
            }
            '┌' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, true, true, true, true, true, false, false,
                    false, true, true, true, true, true, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false,
                ]
            }
            '┐' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '│' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, true, true, true, true, true, false, false, false, true, true,
                    true, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false,
                ]
            }
            '└' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false,
                ]
            }
            '┘' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, true, true, false, false, false,
                    true, true, true, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '█' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, true, true, true, true, true, false, false, false, true, true, true,
                    true, true, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '▓' => {
                self.buf = [
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true,
                ]
            }
            '!' => {
                self.buf = [
                    true, false, true, false, true, false, true, false, false, true, false, true,
                    false, true, false, true, true, false, true, false, true, false, true, false,
                    false, true, false, true, false, true, false, true, true, false, true, false,
                    true, false, true, false, false, true, false, true, false, true, false, true,
                    true, false, true, false, true, false, true, false, false, true, false, true,
                    false, true, false, true,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, true, true, true, false, false, true, true, false, false, false, true,
                    true, false, true, true, false, false, false, true, true, false, true, true,
                    false, false, false, true, true, false, false, true, true, false, true, true,
                    false, false, true, true, true, false, true, true, true, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, false, false, true, true,
                    false, false, true, true, false, false, true, true, true, true, true, true,
                    true, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true,
                ]
            }
            '!' => {
                self.buf = [
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false, false, false, false, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, true, true,
                    false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    true, true, true, true, false, false, false, false, true, true, true, true,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, true, false, false, false, false, true, true, true, true,
                    false, false, false, false, true, true, true, true, false, false, false, false,
                    true, true, true, true, false, false, false, false, true, true, true, true,
                    false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, true, true, true, false,
                    false, false, false, true, true, true, true, false, false, false, true, true,
                    true, false, false, false, false, false, true, true, false, false, false,
                    false, false, false, true, true, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, true, true, true, false, false, false, false, false, true, true,
                    true, true, false, false, false, false, false, false, true, true, true, false,
                    false, false, false, false, false, true, true, false, false, false, false,
                    false, false, true, true, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, true, false,
                    false, false, false, false, false, true, true, true, true, false, false, false,
                    false, false, true, true, true, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, true, true, true, false, false,
                    false, true, true, true, true, false, false, false, false, true, true, true,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, true, true, false, false, false,
                    true, true, true, true, true, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, true, true, true, true, true, false, false, false, true, true, true,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, true, false, false, false, true, true, false, false,
                    false, false, false, false, true, true, false, false, false, false, false,
                    false, true, true, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, true, true, false, false, false, false, false, false,
                    true, true, false, false, false, false, false, false, true, true, false, false,
                    false, true, true, true, true, true, true, true, true, true, true, true, true,
                    true, true, true, true, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, false, false, false, false, false, true, false, false, false,
                    false, false, false, true, true, false, false, false, false, false, true, true,
                    false, false, true, true, false, true, true, false, false, false, true, true,
                    true, true, false, false, false, false, true, true, true, false, false, false,
                    false, false, true, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false,
                ]
            }
            '!' => {
                self.buf = [
                    false, false, true, true, true, true, true, false, false, true, true, false,
                    true, false, true, true, false, true, true, false, true, false, true, true,
                    false, true, true, true, true, true, true, true, false, true, false, true,
                    true, true, false, true, false, true, true, false, false, false, true, true,
                    false, false, true, true, true, true, true, false, false, false, false, false,
                    false, false, false, false,
                ]
            }
            _ => self.buf = [false; 64],
        };
        self.buf[index as usize]
    }
}
