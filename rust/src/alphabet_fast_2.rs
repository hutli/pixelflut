pub struct Alphabet {
    data: [Vec<u8>; 128],
}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {
            data: [
                vec![],
                vec![3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 51, 52],
                vec![1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 29, 30, 32,
                    33, 34, 35, 36, 37, 38, 39, 41, 42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    3, 4, 10, 11, 12, 13, 14, 17, 18, 26, 27, 28, 29, 37, 38, 41, 42, 43, 44, 45,
                    51, 52,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 45, 46, 49, 50, 53,
                    54,
                ],
                vec![
                    2, 3, 4, 9, 10, 12, 13, 17, 18, 20, 21, 26, 27, 28, 30, 31, 33, 34, 36, 37, 38,
                    41, 42, 45, 46, 50, 51, 52, 54, 55,
                ],
                vec![3, 4, 11, 12, 19, 20],
                vec![4, 5, 11, 12, 18, 19, 26, 27, 34, 35, 43, 44, 52, 53],
                vec![2, 3, 11, 12, 20, 21, 28, 29, 36, 37, 43, 44, 50, 51],
                vec![
                    9, 10, 13, 14, 18, 19, 20, 21, 24, 25, 26, 27, 28, 29, 30, 31, 34, 35, 36, 37,
                    41, 42, 45, 46,
                ],
                vec![11, 12, 19, 20, 25, 26, 27, 28, 29, 30, 35, 36, 43, 44],
                vec![43, 44, 51, 52, 58, 59],
                vec![25, 26, 27, 28, 29, 30],
                vec![43, 44, 51, 52],
                vec![6, 7, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 48, 49],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 20, 21, 22, 25, 26, 27, 29, 30, 33, 34, 37,
                    38, 41, 42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    3, 4, 10, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 27, 28, 29, 34, 35, 41, 42, 49, 50, 51, 52,
                    53, 54,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 28, 29, 37, 38, 41, 42, 45, 46, 50, 51, 52,
                    53,
                ],
                vec![
                    4, 5, 6, 11, 12, 13, 14, 18, 19, 21, 22, 25, 26, 29, 30, 33, 34, 35, 36, 37,
                    38, 39, 45, 46, 53, 54,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 19, 20, 21, 29, 30, 37, 38, 41, 42, 45, 46,
                    50, 51, 52, 53,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 27, 28, 29, 33, 34, 37, 38, 41, 42,
                    45, 46, 50, 51, 52, 53,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 9, 10, 13, 14, 21, 22, 28, 29, 35, 36, 43, 44, 51, 52,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 33, 34, 37, 38, 41,
                    42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 30, 37, 38, 41, 42,
                    45, 46, 50, 51, 52, 53,
                ],
                vec![11, 12, 19, 20, 43, 44, 51, 52],
                vec![11, 12, 19, 20, 43, 44, 51, 52, 58, 59],
                vec![4, 5, 11, 12, 18, 19, 25, 26, 34, 35, 43, 44, 52, 53],
                vec![17, 18, 19, 20, 21, 22, 33, 34, 35, 36, 37, 38],
                vec![1, 2, 10, 11, 19, 20, 28, 29, 35, 36, 42, 43, 49, 50],
                vec![2, 3, 4, 5, 9, 10, 13, 14, 21, 22, 28, 29, 35, 36, 51, 52],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 20, 21, 22, 25, 26, 28, 29, 30, 33, 34, 41,
                    42, 46, 47, 50, 51, 52, 53, 54,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 37,
                    38, 41, 42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 37,
                    38, 41, 42, 45, 46, 49, 50, 51, 52, 53,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 33, 34, 41, 42, 45, 46, 50, 51, 52,
                    53,
                ],
                vec![
                    1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38,
                    41, 42, 45, 46, 49, 50, 51, 52, 53,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 25, 26, 27, 28, 33, 34, 41, 42, 49, 50, 51,
                    52, 53, 54,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 9, 10, 17, 18, 25, 26, 27, 28, 33, 34, 41, 42, 49, 50,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 25, 26, 28, 29, 30, 33, 34, 37, 38, 41, 42,
                    45, 46, 50, 51, 52, 53, 54,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 37,
                    38, 41, 42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    2, 3, 4, 5, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
                ],
                vec![
                    5, 6, 13, 14, 21, 22, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 20, 21, 25, 26, 27, 28, 33, 34, 36, 37, 41,
                    42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    1, 2, 9, 10, 17, 18, 25, 26, 33, 34, 41, 42, 49, 50, 51, 52, 53, 54,
                ],
                vec![
                    1, 2, 6, 7, 9, 10, 11, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 25, 26, 28, 30,
                    31, 33, 34, 38, 39, 41, 42, 46, 47, 49, 50, 54, 55,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 11, 13, 14, 17, 18, 19, 20, 21, 22, 25, 26, 28, 29, 30, 33,
                    34, 37, 38, 41, 42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41,
                    42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 41,
                    42, 49, 50,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42,
                    43, 44, 45, 52, 53, 54,
                ],
                vec![
                    1, 2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 27, 28, 29, 33, 34, 37,
                    38, 41, 42, 45, 46, 49, 50, 53, 54,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 13, 14, 17, 18, 26, 27, 28, 29, 37, 38, 41, 42, 45, 46, 50,
                    51, 52, 53,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41,
                    42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42,
                    43, 44, 45, 51, 52,
                ],
                vec![
                    1, 2, 6, 7, 9, 10, 14, 15, 17, 18, 22, 23, 25, 26, 28, 30, 31, 33, 34, 35, 36,
                    37, 38, 39, 41, 42, 43, 45, 46, 47, 49, 50, 54, 55,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 18, 19, 20, 21, 27, 28, 34, 35, 36, 37, 41, 42, 45,
                    46, 49, 50, 53, 54,
                ],
                vec![
                    1, 2, 5, 6, 9, 10, 13, 14, 17, 18, 21, 22, 26, 27, 28, 29, 35, 36, 43, 44, 51,
                    52,
                ],
                vec![
                    1, 2, 3, 4, 5, 6, 13, 14, 20, 21, 27, 28, 34, 35, 41, 42, 49, 50, 51, 52, 53,
                    54,
                ],
                vec![
                    2, 3, 4, 5, 10, 11, 18, 19, 26, 27, 34, 35, 42, 43, 50, 51, 52, 53,
                ],
                vec![0, 1, 9, 10, 18, 19, 27, 28, 36, 37, 45, 46, 54, 55],
                vec![
                    2, 3, 4, 5, 12, 13, 20, 21, 28, 29, 36, 37, 44, 45, 50, 51, 52, 53,
                ],
                vec![3, 4, 10, 11, 12, 13, 17, 18, 21, 22],
                vec![56, 57, 58, 59, 60, 61, 62, 63],
                vec![2, 3, 11, 12, 20, 21],
                vec![
                    18, 19, 20, 21, 29, 30, 34, 35, 36, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53, 54,
                ],
                vec![
                    1, 2, 9, 10, 17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45,
                    46, 49, 50, 51, 52, 53,
                ],
                vec![
                    18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 41, 42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    5, 6, 13, 14, 18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45,
                    46, 50, 51, 52, 53, 54,
                ],
                vec![
                    18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 35, 36, 37, 38, 41, 42, 50, 51, 52, 53,
                    54,
                ],
                vec![
                    3, 4, 5, 10, 11, 18, 19, 25, 26, 27, 28, 29, 34, 35, 42, 43, 50, 51,
                ],
                vec![
                    18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 53, 54,
                    57, 58, 59, 60, 61,
                ],
                vec![
                    1, 2, 9, 10, 17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45,
                    46, 49, 50, 53, 54,
                ],
                vec![3, 4, 18, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53],
                vec![4, 5, 19, 20, 21, 28, 29, 36, 37, 44, 45, 52, 53, 58, 59, 60],
                vec![
                    1, 2, 9, 10, 17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 35, 36, 37, 41, 42, 45,
                    46, 49, 50, 53, 54,
                ],
                vec![
                    2, 3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 50, 51, 52, 53,
                ],
                vec![
                    17, 18, 19, 21, 22, 25, 26, 27, 28, 29, 30, 31, 33, 34, 36, 38, 39, 41, 42, 44,
                    46, 47, 49, 50, 54, 55,
                ],
                vec![
                    17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 49, 50, 53,
                    54,
                ],
                vec![
                    18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 43, 44, 45, 49, 50,
                    57, 58,
                ],
                vec![
                    18, 19, 20, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 53, 54,
                    61, 62,
                ],
                vec![17, 18, 19, 20, 21, 25, 26, 29, 30, 33, 34, 41, 42, 49, 50],
                vec![
                    18, 19, 20, 21, 22, 25, 26, 34, 35, 36, 37, 45, 46, 49, 50, 51, 52, 53,
                ],
                vec![
                    11, 12, 17, 18, 19, 20, 21, 22, 27, 28, 35, 36, 43, 44, 52, 53, 54,
                ],
                vec![
                    17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 45, 46, 50, 51, 52, 53,
                    54,
                ],
                vec![
                    17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 51, 52,
                ],
                vec![
                    17, 18, 22, 23, 25, 26, 28, 30, 31, 33, 34, 36, 38, 39, 42, 43, 44, 45, 46, 50,
                    51, 53, 54,
                ],
                vec![
                    17, 18, 21, 22, 26, 27, 28, 29, 35, 36, 42, 43, 44, 45, 49, 50, 53, 54,
                ],
                vec![
                    17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 42, 43, 44, 45, 46, 52, 53, 57,
                    58, 59, 60,
                ],
                vec![
                    17, 18, 19, 20, 21, 22, 28, 29, 35, 36, 42, 43, 49, 50, 51, 52, 53, 54,
                ],
                vec![
                    4, 5, 6, 11, 12, 19, 20, 25, 26, 27, 35, 36, 43, 44, 52, 53, 54,
                ],
                vec![3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52],
                vec![
                    1, 2, 3, 11, 12, 19, 20, 28, 29, 30, 35, 36, 43, 44, 49, 50, 51,
                ],
                vec![18, 19, 22, 23, 25, 26, 28, 30, 31, 33, 34, 37, 38],
                vec![],
                vec![
                    4, 11, 12, 18, 19, 20, 25, 26, 27, 28, 34, 35, 36, 43, 44, 52,
                ],
                vec![
                    3, 11, 12, 19, 20, 21, 27, 28, 29, 30, 35, 36, 37, 43, 44, 51,
                ],
                vec![
                    4, 11, 12, 13, 18, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 31,
                ],
                vec![
                    25, 26, 27, 28, 29, 30, 31, 34, 35, 36, 37, 38, 43, 44, 45, 52,
                ],
                vec![27, 28, 35, 36],
                vec![19, 20, 26, 27, 28, 29, 34, 35, 36, 37, 43, 44],
                vec![
                    10, 11, 12, 13, 17, 18, 19, 20, 21, 22, 25, 26, 27, 28, 29, 30, 33, 34, 35, 36,
                    37, 38, 41, 42, 43, 44, 45, 46, 50, 51, 52, 53,
                ],
                vec![
                    2, 3, 4, 5, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                    47, 49, 50, 51, 52, 53, 54, 58, 59, 60, 61,
                ],
                vec![
                    27, 28, 29, 30, 31, 35, 36, 37, 38, 39, 43, 44, 51, 52, 59, 60,
                ],
                vec![
                    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                ],
                vec![
                    24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 43, 44, 51, 52, 59, 60,
                ],
                vec![3, 4, 11, 12, 19, 20, 27, 28, 35, 36, 43, 44, 51, 52, 59, 60],
                vec![3, 4, 11, 12, 19, 20, 27, 28, 29, 30, 31, 35, 36, 37, 38, 39],
                vec![3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36],
                vec![
                    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
                    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
                    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
                    62, 63,
                ],
                vec![
                    0, 2, 4, 6, 9, 11, 13, 15, 16, 18, 20, 22, 25, 27, 29, 31, 32, 34, 36, 38, 41,
                    43, 45, 47, 48, 50, 52, 54, 57, 59, 61, 63,
                ],
                vec![
                    10, 11, 12, 13, 14, 17, 18, 22, 23, 25, 26, 30, 31, 33, 34, 38, 39, 42, 43, 45,
                    46, 49, 50, 51, 53, 54, 55,
                ],
                vec![
                    17, 18, 21, 22, 25, 26, 29, 30, 33, 34, 37, 38, 41, 42, 43, 44, 45, 46, 47, 49,
                    50, 57, 58,
                ],
                vec![
                    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
                ],
                vec![
                    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
                    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
                ],
                vec![0, 1, 8, 9, 16, 17, 24, 25, 32, 33, 40, 41, 48, 49, 56, 57],
                vec![
                    0, 1, 2, 3, 8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 32, 33, 34, 35, 40,
                    41, 42, 43, 48, 49, 50, 51, 56, 57, 58, 59,
                ],
                vec![29, 30, 31, 36, 37, 38, 39, 43, 44, 45, 51, 52, 59, 60],
                vec![24, 25, 26, 32, 33, 34, 35, 42, 43, 44, 51, 52, 59, 60],
                vec![3, 4, 11, 12, 19, 20, 21, 28, 29, 30, 31, 37, 38, 39],
                vec![3, 4, 11, 12, 18, 19, 20, 24, 25, 26, 27, 32, 33, 34],
                vec![
                    3, 4, 11, 12, 19, 20, 27, 28, 29, 30, 31, 35, 36, 37, 38, 39, 43, 44, 51, 52,
                    59, 60,
                ],
                vec![
                    3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 32, 33, 34, 35, 36, 43, 44, 51, 52,
                    59, 60,
                ],
                vec![
                    24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 43, 44, 51, 52,
                    59, 60,
                ],
                vec![
                    3, 4, 11, 12, 19, 20, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
                    38, 39,
                ],
                vec![
                    7, 14, 15, 21, 22, 25, 26, 28, 29, 33, 34, 35, 36, 41, 42, 43, 49, 50,
                ],
                vec![
                    2, 3, 4, 5, 6, 9, 10, 12, 14, 15, 17, 18, 20, 22, 23, 25, 26, 27, 28, 29, 30,
                    31, 33, 35, 36, 37, 39, 41, 42, 46, 47, 50, 51, 52, 53, 54,
                ],
            ],
        }
    }

    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool {
        match c {
            ' ' => self.data[0].binary_search(&index).is_ok(),
            '!' => self.data[1].binary_search(&index).is_ok(),
            '"' => self.data[2].binary_search(&index).is_ok(),
            '#' => self.data[3].binary_search(&index).is_ok(),
            '$' => self.data[4].binary_search(&index).is_ok(),
            '%' => self.data[5].binary_search(&index).is_ok(),
            '&' => self.data[6].binary_search(&index).is_ok(),
            '\'' => self.data[7].binary_search(&index).is_ok(),
            '(' => self.data[8].binary_search(&index).is_ok(),
            ')' => self.data[9].binary_search(&index).is_ok(),
            '*' => self.data[10].binary_search(&index).is_ok(),
            '+' => self.data[11].binary_search(&index).is_ok(),
            ',' => self.data[12].binary_search(&index).is_ok(),
            '-' => self.data[13].binary_search(&index).is_ok(),
            '.' => self.data[14].binary_search(&index).is_ok(),
            '/' => self.data[15].binary_search(&index).is_ok(),
            '0' => self.data[16].binary_search(&index).is_ok(),
            '1' => self.data[17].binary_search(&index).is_ok(),
            '2' => self.data[18].binary_search(&index).is_ok(),
            '3' => self.data[19].binary_search(&index).is_ok(),
            '4' => self.data[20].binary_search(&index).is_ok(),
            '5' => self.data[21].binary_search(&index).is_ok(),
            '6' => self.data[22].binary_search(&index).is_ok(),
            '7' => self.data[23].binary_search(&index).is_ok(),
            '8' => self.data[24].binary_search(&index).is_ok(),
            '9' => self.data[25].binary_search(&index).is_ok(),
            ':' => self.data[26].binary_search(&index).is_ok(),
            ';' => self.data[27].binary_search(&index).is_ok(),
            '<' => self.data[28].binary_search(&index).is_ok(),
            '=' => self.data[29].binary_search(&index).is_ok(),
            '>' => self.data[30].binary_search(&index).is_ok(),
            '?' => self.data[31].binary_search(&index).is_ok(),
            '@' => self.data[32].binary_search(&index).is_ok(),
            'A' => self.data[33].binary_search(&index).is_ok(),
            'B' => self.data[34].binary_search(&index).is_ok(),
            'C' => self.data[35].binary_search(&index).is_ok(),
            'D' => self.data[36].binary_search(&index).is_ok(),
            'E' => self.data[37].binary_search(&index).is_ok(),
            'F' => self.data[38].binary_search(&index).is_ok(),
            'G' => self.data[39].binary_search(&index).is_ok(),
            'H' => self.data[40].binary_search(&index).is_ok(),
            'I' => self.data[41].binary_search(&index).is_ok(),
            'J' => self.data[42].binary_search(&index).is_ok(),
            'K' => self.data[43].binary_search(&index).is_ok(),
            'L' => self.data[44].binary_search(&index).is_ok(),
            'M' => self.data[45].binary_search(&index).is_ok(),
            'N' => self.data[46].binary_search(&index).is_ok(),
            'O' => self.data[47].binary_search(&index).is_ok(),
            'P' => self.data[48].binary_search(&index).is_ok(),
            'Q' => self.data[49].binary_search(&index).is_ok(),
            'R' => self.data[50].binary_search(&index).is_ok(),
            'S' => self.data[51].binary_search(&index).is_ok(),
            'T' => self.data[52].binary_search(&index).is_ok(),
            'U' => self.data[53].binary_search(&index).is_ok(),
            'V' => self.data[54].binary_search(&index).is_ok(),
            'W' => self.data[55].binary_search(&index).is_ok(),
            'X' => self.data[56].binary_search(&index).is_ok(),
            'Y' => self.data[57].binary_search(&index).is_ok(),
            'Z' => self.data[58].binary_search(&index).is_ok(),
            '[' => self.data[59].binary_search(&index).is_ok(),
            '\\' => self.data[60].binary_search(&index).is_ok(),
            ']' => self.data[61].binary_search(&index).is_ok(),
            '^' => self.data[62].binary_search(&index).is_ok(),
            '_' => self.data[63].binary_search(&index).is_ok(),
            '`' => self.data[64].binary_search(&index).is_ok(),
            'a' => self.data[65].binary_search(&index).is_ok(),
            'b' => self.data[66].binary_search(&index).is_ok(),
            'c' => self.data[67].binary_search(&index).is_ok(),
            'd' => self.data[68].binary_search(&index).is_ok(),
            'e' => self.data[69].binary_search(&index).is_ok(),
            'f' => self.data[70].binary_search(&index).is_ok(),
            'g' => self.data[71].binary_search(&index).is_ok(),
            'h' => self.data[72].binary_search(&index).is_ok(),
            'i' => self.data[73].binary_search(&index).is_ok(),
            'j' => self.data[74].binary_search(&index).is_ok(),
            'k' => self.data[75].binary_search(&index).is_ok(),
            'l' => self.data[76].binary_search(&index).is_ok(),
            'm' => self.data[77].binary_search(&index).is_ok(),
            'n' => self.data[78].binary_search(&index).is_ok(),
            'o' => self.data[79].binary_search(&index).is_ok(),
            'p' => self.data[80].binary_search(&index).is_ok(),
            'q' => self.data[81].binary_search(&index).is_ok(),
            'r' => self.data[82].binary_search(&index).is_ok(),
            's' => self.data[83].binary_search(&index).is_ok(),
            't' => self.data[84].binary_search(&index).is_ok(),
            'u' => self.data[85].binary_search(&index).is_ok(),
            'v' => self.data[86].binary_search(&index).is_ok(),
            'w' => self.data[87].binary_search(&index).is_ok(),
            'x' => self.data[88].binary_search(&index).is_ok(),
            'y' => self.data[89].binary_search(&index).is_ok(),
            'z' => self.data[90].binary_search(&index).is_ok(),
            '{' => self.data[91].binary_search(&index).is_ok(),
            '|' => self.data[92].binary_search(&index).is_ok(),
            '}' => self.data[93].binary_search(&index).is_ok(),
            '~' => self.data[94].binary_search(&index).is_ok(),
            '◂' => self.data[95].binary_search(&index).is_ok(),
            '▸' => self.data[96].binary_search(&index).is_ok(),
            '▴' => self.data[97].binary_search(&index).is_ok(),
            '▾' => self.data[98].binary_search(&index).is_ok(),
            '!' => self.data[99].binary_search(&index).is_ok(),
            '!' => self.data[100].binary_search(&index).is_ok(),
            '!' => self.data[101].binary_search(&index).is_ok(),
            '!' => self.data[102].binary_search(&index).is_ok(),
            '┌' => self.data[103].binary_search(&index).is_ok(),
            '┌' => self.data[104].binary_search(&index).is_ok(),
            '┐' => self.data[105].binary_search(&index).is_ok(),
            '│' => self.data[106].binary_search(&index).is_ok(),
            '└' => self.data[107].binary_search(&index).is_ok(),
            '┘' => self.data[108].binary_search(&index).is_ok(),
            '█' => self.data[109].binary_search(&index).is_ok(),
            '▓' => self.data[110].binary_search(&index).is_ok(),
            '!' => self.data[111].binary_search(&index).is_ok(),
            '!' => self.data[112].binary_search(&index).is_ok(),
            '!' => self.data[113].binary_search(&index).is_ok(),
            '!' => self.data[114].binary_search(&index).is_ok(),
            '!' => self.data[115].binary_search(&index).is_ok(),
            '!' => self.data[116].binary_search(&index).is_ok(),
            '!' => self.data[117].binary_search(&index).is_ok(),
            '!' => self.data[118].binary_search(&index).is_ok(),
            '!' => self.data[119].binary_search(&index).is_ok(),
            '!' => self.data[120].binary_search(&index).is_ok(),
            '!' => self.data[121].binary_search(&index).is_ok(),
            '!' => self.data[122].binary_search(&index).is_ok(),
            '!' => self.data[123].binary_search(&index).is_ok(),
            '!' => self.data[124].binary_search(&index).is_ok(),
            '!' => self.data[125].binary_search(&index).is_ok(),
            '!' => self.data[126].binary_search(&index).is_ok(),
            '!' => self.data[127].binary_search(&index).is_ok(),
            _ => false,
        }
    }
}
