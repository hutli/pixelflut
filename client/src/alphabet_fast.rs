pub struct Alphabet {}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {}
    }
    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool {
        let x = match c {
            ' ' => [].binary_search(&index).is_ok(),
            '!' => [3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 51, 52]
                .binary_search(&index)
                .is_ok(),
            '"' => [1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22]
                .binary_search(&index)
                .is_ok(),
            '#' => [
                1, 2, 5, 6, 9, 10, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 29, 30, 32, 33,
                34, 35, 36, 37, 38, 39, 41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '$' => [
                3, 4, 10, 11, 12, 13, 14, 17, 18, 26, 27, 28, 29, 37, 38, 41, 42, 43, 44, 45, 51,
                52,
            ]
            .binary_search(&index)
            .is_ok(),
            '%' => [
                1, 2, 5, 6, 9, 10, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '&' => [
                2, 3, 4, 9, 10, 12, 13, 17, 18, 20, 21, 26, 27, 28, 30, 31, 33, 34, 36, 37, 38, 41,
                42, 45, 46, 50, 51, 52, 54, 55,
            ]
            .binary_search(&index)
            .is_ok(),
            '\'' => [3, 4, 11, 12, 19, 20].binary_search(&index).is_ok(),
            '(' => [4, 5, 11, 12, 18, 19, 26, 27, 34, 35, 43, 44, 52, 53]
                .binary_search(&index)
                .is_ok(),
            ')' => [2, 3, 11, 12, 20, 21, 28, 29, 36, 37, 43, 44, 50, 51]
                .binary_search(&index)
                .is_ok(),
            '*' => [
                9, 10, 13, 14, 18, 19, 20, 21, 24, 25, 26, 27, 28, 29, 30, 31, 34, 35, 36, 37, 41,
                42, 45, 46,
            ]
            .binary_search(&index)
            .is_ok(),
            '+' => [11, 12, 19, 20, 25, 26, 27, 28, 29, 30, 35, 36, 43, 44]
                .binary_search(&index)
                .is_ok(),
            ',' => [43, 44, 51, 52, 58, 59].binary_search(&index).is_ok(),
            '-' => [25, 26, 27, 28, 29, 30].binary_search(&index).is_ok(),
            '.' => [43, 44, 51, 52].binary_search(&index).is_ok(),
            '/' => [6, 7, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 48, 49]
                .binary_search(&index)
                .is_ok(),
            '0' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 20, 21, 22, 25, 26, 27, 29, 30, 33, 34, 37, 38,
                41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '1' => [
                3, 4, 10, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '2' => [
                2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 27, 28, 29, 34, 35, 41, 42, 49, 50, 51, 52, 53,
                54,
            ]
            .binary_search(&index)
            .is_ok(),
            '3' => [
                2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 28, 29, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '4' => [
                4, 5, 6, 11, 12, 13, 14, 18, 19, 21, 22, 25, 26, 29, 30, 33, 34, 35, 36, 37, 38,
                39, 45, 46, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '5' => [
                1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 19, 20, 21, 29, 30, 37, 38, 41, 42, 45, 46, 50,
                51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '6' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 27, 28, 29, 33, 34, 37, 38, 41, 42, 45,
                46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '7' => [
                1, 2, 3, 4, 5, 6, 9, 10, 13, 14, 21, 22, 28, 29, 35, 36, 43, 44, 51, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            '8' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 33, 34, 37, 38, 41, 42,
                45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '9' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 30, 37, 38, 41, 42, 45,
                46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            ':' => [11, 12, 19, 20, 43, 44, 51, 52]
                .binary_search(&index)
                .is_ok(),
            ';' => [11, 12, 19, 20, 43, 44, 51, 52, 58, 59]
                .binary_search(&index)
                .is_ok(),
            '<' => [4, 5, 11, 12, 18, 19, 25, 26, 34, 35, 43, 44, 52, 53]
                .binary_search(&index)
                .is_ok(),
            '=' => [17, 18, 19, 20, 21, 22, 33, 34, 35, 36, 37, 38]
                .binary_search(&index)
                .is_ok(),
            '>' => [1, 2, 10, 11, 19, 20, 28, 29, 35, 36, 42, 43, 49, 50]
                .binary_search(&index)
                .is_ok(),
            '?' => [2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 28, 29, 35, 36, 51, 52]
                .binary_search(&index)
                .is_ok(),
            '@' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 20, 21, 22, 25, 26, 28, 29, 30, 33, 34, 41, 42,
                46, 47, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'A' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 37, 38,
                41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'B' => [
                1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 37, 38,
                41, 42, 45, 46, 49, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'C' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 33, 34, 41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'D' => [
                1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41,
                42, 45, 46, 49, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'E' => [
                1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 25, 26, 27, 28, 33, 34, 41, 42, 49, 50, 51, 52,
                53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'F' => [
                1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 25, 26, 27, 28, 33, 34, 41, 42, 49, 50,
            ]
            .binary_search(&index)
            .is_ok(),
            'G' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 28, 29, 30, 33, 34, 37, 38, 41, 42, 45,
                46, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'H' => [
                1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 37, 38,
                41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'I' => [
                2, 3, 4, 5, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'J' => [
                5, 6, 13, 14, 21, 22, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'K' => [
                1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 20, 21, 25, 26, 27, 28, 33, 34, 36, 37, 41, 42,
                45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'L' => [
                1, 2, 9, 10, 17, 18, 25, 26, 33, 34, 41, 42, 49, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'M' => [
                1, 2, 6, 7, 9, 10, 11, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 25, 26, 28, 30, 31,
                33, 34, 38, 39, 41, 42, 46, 47, 49, 50, 54, 55,
            ]
            .binary_search(&index)
            .is_ok(),
            'N' => [
                1, 2, 5, 6, 9, 10, 11, 13, 14, 17, 18, 19, 20, 21, 22, 25, 26, 28, 29, 30, 33, 34,
                37, 38, 41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'O' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42,
                45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'P' => [
                1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 41, 42,
                49, 50,
            ]
            .binary_search(&index)
            .is_ok(),
            'Q' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43,
                44, 45, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'R' => [
                1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 37, 38,
                41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'S' => [
                2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 26, 27, 28, 29, 37, 38, 41, 42, 45, 46, 50, 51,
                52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'T' => [
                1, 2, 3, 4, 5, 6, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            'U' => [
                1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42,
                45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'V' => [
                1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43,
                44, 45, 51, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            'W' => [
                1, 2, 6, 7, 9, 10, 14, 15, 17, 18, 22, 23, 25, 26, 28, 30, 31, 33, 34, 35, 36, 37,
                38, 39, 41, 42, 43, 45, 46, 47, 49, 50, 54, 55,
            ]
            .binary_search(&index)
            .is_ok(),
            'X' => [
                1, 2, 5, 6, 9, 10, 13, 14, 18, 19, 20, 21, 27, 28, 34, 35, 36, 37, 41, 42, 45, 46,
                49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'Y' => [
                1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 35, 36, 43, 44, 51, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            'Z' => [
                1, 2, 3, 4, 5, 6, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 49, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '[' => [
                2, 3, 4, 5, 10, 11, 18, 19, 26, 27, 34, 35, 42, 43, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '\\' => [0, 1, 9, 10, 18, 19, 27, 28, 36, 37, 45, 46, 54, 55]
                .binary_search(&index)
                .is_ok(),
            ']' => [
                2, 3, 4, 5, 12, 13, 20, 21, 28, 29, 36, 37, 44, 45, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '^' => [3, 4, 10, 11, 12, 13, 17, 18, 21, 22]
                .binary_search(&index)
                .is_ok(),
            '_' => [56, 57, 58, 59, 60, 61, 62, 63]
                .binary_search(&index)
                .is_ok(),
            '`' => [2, 3, 11, 12, 20, 21].binary_search(&index).is_ok(),
            'a' => [
                18, 19, 20, 21, 29, 30, 34, 35, 36, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'b' => [
                1, 2, 9, 10, 17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46,
                49, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'c' => [
                18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'd' => [
                5, 6, 13, 14, 18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46,
                50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'e' => [
                18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 35, 36, 37, 38, 41, 42, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'f' => [
                3, 4, 5, 10, 11, 18, 19, 25, 26, 27, 28, 29, 34, 35, 42, 43, 50, 51,
            ]
            .binary_search(&index)
            .is_ok(),
            'g' => [
                18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 53, 54, 57,
                58, 59, 60, 61,
            ]
            .binary_search(&index)
            .is_ok(),
            'h' => [
                1, 2, 9, 10, 17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46,
                49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'i' => [3, 4, 18, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53]
                .binary_search(&index)
                .is_ok(),
            'j' => [4, 5, 19, 20, 21, 28, 29, 36, 37, 44, 45, 52, 53, 58, 59, 60]
                .binary_search(&index)
                .is_ok(),
            'k' => [
                1, 2, 9, 10, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 35, 36, 37, 41, 42, 45, 46,
                49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'l' => [
                2, 3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'm' => [
                17, 18, 19, 21, 22, 25, 26, 27, 28, 29, 30, 31, 33, 34, 36, 38, 39, 41, 42, 44, 46,
                47, 49, 50, 54, 55,
            ]
            .binary_search(&index)
            .is_ok(),
            'n' => [
                17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'o' => [
                18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            'p' => [
                17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 43, 44, 45, 49, 50, 57,
                58,
            ]
            .binary_search(&index)
            .is_ok(),
            'q' => [
                18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 53, 54, 61,
                62,
            ]
            .binary_search(&index)
            .is_ok(),
            'r' => [17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 41, 42, 49, 50]
                .binary_search(&index)
                .is_ok(),
            's' => [
                18, 19, 20, 21, 22, 25, 26, 34, 35, 36, 37, 45, 46, 49, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            't' => [
                11, 12, 17, 18, 19, 20, 21, 22, 27, 28, 35, 36, 43, 44, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'u' => [
                17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'v' => [
                17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 51, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            'w' => [
                17, 18, 22, 23, 25, 26, 28, 30, 31, 33, 34, 36, 38, 39, 42, 43, 44, 45, 46, 50, 51,
                53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'x' => [
                17, 18, 21, 22, 26, 27, 28, 29, 35, 36, 42, 43, 44, 45, 49, 50, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            'y' => [
                17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 52, 53, 57, 58,
                59, 60,
            ]
            .binary_search(&index)
            .is_ok(),
            'z' => [
                17, 18, 19, 20, 21, 22, 28, 29, 35, 36, 42, 43, 49, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '{' => [
                4, 5, 6, 11, 12, 19, 20, 25, 26, 27, 35, 36, 43, 44, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            '|' => [3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52]
                .binary_search(&index)
                .is_ok(),
            '}' => [
                1, 2, 3, 11, 12, 19, 20, 28, 29, 30, 35, 36, 43, 44, 49, 50, 51,
            ]
            .binary_search(&index)
            .is_ok(),
            '~' => [18, 19, 22, 23, 25, 26, 28, 30, 31, 33, 34, 37, 38]
                .binary_search(&index)
                .is_ok(),
            '🙊' => [].binary_search(&index).is_ok(),
            '◂' => [
                4, 11, 12, 18, 19, 20, 25, 26, 27, 28, 34, 35, 36, 43, 44, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            '▸' => [
                3, 11, 12, 19, 20, 21, 27, 28, 29, 30, 35, 36, 37, 43, 44, 51,
            ]
            .binary_search(&index)
            .is_ok(),
            '▴' => [
                4, 11, 12, 13, 18, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 31,
            ]
            .binary_search(&index)
            .is_ok(),
            '▾' => [
                25, 26, 27, 28, 29, 30, 31, 34, 35, 36, 37, 38, 43, 44, 45, 52,
            ]
            .binary_search(&index)
            .is_ok(),
            '▪' => [27, 28, 35, 36].binary_search(&index).is_ok(),
            '🞄' => [19, 20, 26, 27, 28, 29, 34, 35, 36, 37, 43, 44]
                .binary_search(&index)
                .is_ok(),
            '●' => [
                10, 11, 12, 13, 17, 18, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 35, 36, 37,
                38, 41, 42, 43, 44, 45, 46, 50, 51, 52, 53,
            ]
            .binary_search(&index)
            .is_ok(),
            '┌' => [
                2, 3, 4, 5, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 49,
                50, 51, 52, 53, 54, 58, 59, 60, 61,
            ]
            .binary_search(&index)
            .is_ok(),
            '━' => [
                27, 28, 29, 30, 31, 35, 36, 37, 38, 39, 43, 44, 51, 52, 59, 60,
            ]
            .binary_search(&index)
            .is_ok(),
            '┐' => [
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
            ]
            .binary_search(&index)
            .is_ok(),
            '│' => [
                24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 43, 44, 51, 52, 59, 60,
            ]
            .binary_search(&index)
            .is_ok(),
            '└' => [3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52, 59, 60]
                .binary_search(&index)
                .is_ok(),
            '┘' => [3, 4, 11, 12, 19, 20, 27, 28, 29, 30, 31, 35, 36, 37, 38, 39]
                .binary_search(&index)
                .is_ok(),
            '█' => [3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36]
                .binary_search(&index)
                .is_ok(),
            '▓' => [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
                44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            ]
            .binary_search(&index)
            .is_ok(),
            'Ω' => [
                0, 2, 4, 6, 9, 11, 13, 15, 16, 18, 20, 22, 25, 27, 29, 31, 32, 34, 36, 38, 41, 43,
                45, 47, 48, 50, 52, 54, 57, 59, 61, 63,
            ]
            .binary_search(&index)
            .is_ok(),
            'µ' => [
                10, 11, 12, 13, 14, 17, 18, 22, 23, 25, 26, 30, 31, 33, 34, 38, 39, 42, 43, 45, 46,
                49, 50, 51, 53, 54, 55,
            ]
            .binary_search(&index)
            .is_ok(),
            '▁' => [
                17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 43, 44, 45, 46, 47, 49, 50,
                57, 58,
            ]
            .binary_search(&index)
            .is_ok(),
            '▂' => [
                48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            ]
            .binary_search(&index)
            .is_ok(),
            '▎' => [
                32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
                53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
            ]
            .binary_search(&index)
            .is_ok(),
            '▌' => [0, 1, 8, 9, 16, 17, 24, 25, 32, 33, 40, 41, 48, 49, 56, 57]
                .binary_search(&index)
                .is_ok(),
            '😼' => [
                0, 1, 2, 3, 8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40, 41,
                42, 43, 48, 49, 50, 51, 56, 57, 58, 59,
            ]
            .binary_search(&index)
            .is_ok(),
            '🙎' => [29, 30, 31, 36, 37, 38, 39, 43, 44, 45, 51, 52, 59, 60]
                .binary_search(&index)
                .is_ok(),
            '🙍' => [24, 25, 26, 32, 33, 34, 35, 42, 43, 44, 51, 52, 59, 60]
                .binary_search(&index)
                .is_ok(),
            '🙌' => [3, 4, 11, 12, 19, 20, 21, 28, 29, 30, 31, 37, 38, 39]
                .binary_search(&index)
                .is_ok(),
            '🙋' => [3, 4, 11, 12, 18, 19, 20, 24, 25, 26, 27, 32, 33, 34]
                .binary_search(&index)
                .is_ok(),
            '🙀' => [
                3, 4, 11, 12, 19, 20, 27, 28, 29, 30, 31, 35, 36, 37, 38, 39, 43, 44, 51, 52, 59,
                60,
            ]
            .binary_search(&index)
            .is_ok(),
            '😵' => [
                3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 43, 44, 51, 52, 59,
                60,
            ]
            .binary_search(&index)
            .is_ok(),
            '😦' => [
                24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 43, 44, 51, 52, 59,
                60,
            ]
            .binary_search(&index)
            .is_ok(),
            '😣' => [
                3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                39,
            ]
            .binary_search(&index)
            .is_ok(),
            '😏' => [
                7, 14, 15, 21, 22, 25, 26, 28, 29, 33, 34, 35, 36, 41, 42, 43, 49, 50,
            ]
            .binary_search(&index)
            .is_ok(),
            '😀' => [
                2, 3, 4, 5, 6, 9, 10, 12, 14, 15, 17, 18, 20, 22, 23, 25, 26, 27, 28, 29, 30, 31,
                33, 35, 36, 37, 39, 41, 42, 46, 47, 50, 51, 52, 53, 54,
            ]
            .binary_search(&index)
            .is_ok(),
            _ => return false,
        };
        x
    }
}
