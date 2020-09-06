pub struct Alphabet {
    arr: [Vec<u8>; 128],
}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {
            arr: [
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

    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> Option<bool> {
        match c {
            ' ' => Some(self.arr[0].binary_search(&index).is_ok()),
            '!' => Some(self.arr[1].binary_search(&index).is_ok()),
            '"' => Some(self.arr[2].binary_search(&index).is_ok()),
            '#' => Some(self.arr[3].binary_search(&index).is_ok()),
            '$' => Some(self.arr[4].binary_search(&index).is_ok()),
            '%' => Some(self.arr[5].binary_search(&index).is_ok()),
            '&' => Some(self.arr[6].binary_search(&index).is_ok()),
            '\'' => Some(self.arr[7].binary_search(&index).is_ok()),
            '(' => Some(self.arr[8].binary_search(&index).is_ok()),
            ')' => Some(self.arr[9].binary_search(&index).is_ok()),
            '*' => Some(self.arr[10].binary_search(&index).is_ok()),
            '+' => Some(self.arr[11].binary_search(&index).is_ok()),
            ',' => Some(self.arr[12].binary_search(&index).is_ok()),
            '-' => Some(self.arr[13].binary_search(&index).is_ok()),
            '.' => Some(self.arr[14].binary_search(&index).is_ok()),
            '/' => Some(self.arr[15].binary_search(&index).is_ok()),
            '0' => Some(self.arr[16].binary_search(&index).is_ok()),
            '1' => Some(self.arr[17].binary_search(&index).is_ok()),
            '2' => Some(self.arr[18].binary_search(&index).is_ok()),
            '3' => Some(self.arr[19].binary_search(&index).is_ok()),
            '4' => Some(self.arr[20].binary_search(&index).is_ok()),
            '5' => Some(self.arr[21].binary_search(&index).is_ok()),
            '6' => Some(self.arr[22].binary_search(&index).is_ok()),
            '7' => Some(self.arr[23].binary_search(&index).is_ok()),
            '8' => Some(self.arr[24].binary_search(&index).is_ok()),
            '9' => Some(self.arr[25].binary_search(&index).is_ok()),
            ':' => Some(self.arr[26].binary_search(&index).is_ok()),
            ';' => Some(self.arr[27].binary_search(&index).is_ok()),
            '<' => Some(self.arr[28].binary_search(&index).is_ok()),
            '=' => Some(self.arr[29].binary_search(&index).is_ok()),
            '>' => Some(self.arr[30].binary_search(&index).is_ok()),
            '?' => Some(self.arr[31].binary_search(&index).is_ok()),
            '@' => Some(self.arr[32].binary_search(&index).is_ok()),
            'A' => Some(self.arr[33].binary_search(&index).is_ok()),
            'B' => Some(self.arr[34].binary_search(&index).is_ok()),
            'C' => Some(self.arr[35].binary_search(&index).is_ok()),
            'D' => Some(self.arr[36].binary_search(&index).is_ok()),
            'E' => Some(self.arr[37].binary_search(&index).is_ok()),
            'F' => Some(self.arr[38].binary_search(&index).is_ok()),
            'G' => Some(self.arr[39].binary_search(&index).is_ok()),
            'H' => Some(self.arr[40].binary_search(&index).is_ok()),
            'I' => Some(self.arr[41].binary_search(&index).is_ok()),
            'J' => Some(self.arr[42].binary_search(&index).is_ok()),
            'K' => Some(self.arr[43].binary_search(&index).is_ok()),
            'L' => Some(self.arr[44].binary_search(&index).is_ok()),
            'M' => Some(self.arr[45].binary_search(&index).is_ok()),
            'N' => Some(self.arr[46].binary_search(&index).is_ok()),
            'O' => Some(self.arr[47].binary_search(&index).is_ok()),
            'P' => Some(self.arr[48].binary_search(&index).is_ok()),
            'Q' => Some(self.arr[49].binary_search(&index).is_ok()),
            'R' => Some(self.arr[50].binary_search(&index).is_ok()),
            'S' => Some(self.arr[51].binary_search(&index).is_ok()),
            'T' => Some(self.arr[52].binary_search(&index).is_ok()),
            'U' => Some(self.arr[53].binary_search(&index).is_ok()),
            'V' => Some(self.arr[54].binary_search(&index).is_ok()),
            'W' => Some(self.arr[55].binary_search(&index).is_ok()),
            'X' => Some(self.arr[56].binary_search(&index).is_ok()),
            'Y' => Some(self.arr[57].binary_search(&index).is_ok()),
            'Z' => Some(self.arr[58].binary_search(&index).is_ok()),
            '[' => Some(self.arr[59].binary_search(&index).is_ok()),
            '\\' => Some(self.arr[60].binary_search(&index).is_ok()),
            ']' => Some(self.arr[61].binary_search(&index).is_ok()),
            '^' => Some(self.arr[62].binary_search(&index).is_ok()),
            '_' => Some(self.arr[63].binary_search(&index).is_ok()),
            '`' => Some(self.arr[64].binary_search(&index).is_ok()),
            'a' => Some(self.arr[65].binary_search(&index).is_ok()),
            'b' => Some(self.arr[66].binary_search(&index).is_ok()),
            'c' => Some(self.arr[67].binary_search(&index).is_ok()),
            'd' => Some(self.arr[68].binary_search(&index).is_ok()),
            'e' => Some(self.arr[69].binary_search(&index).is_ok()),
            'f' => Some(self.arr[70].binary_search(&index).is_ok()),
            'g' => Some(self.arr[71].binary_search(&index).is_ok()),
            'h' => Some(self.arr[72].binary_search(&index).is_ok()),
            'i' => Some(self.arr[73].binary_search(&index).is_ok()),
            'j' => Some(self.arr[74].binary_search(&index).is_ok()),
            'k' => Some(self.arr[75].binary_search(&index).is_ok()),
            'l' => Some(self.arr[76].binary_search(&index).is_ok()),
            'm' => Some(self.arr[77].binary_search(&index).is_ok()),
            'n' => Some(self.arr[78].binary_search(&index).is_ok()),
            'o' => Some(self.arr[79].binary_search(&index).is_ok()),
            'p' => Some(self.arr[80].binary_search(&index).is_ok()),
            'q' => Some(self.arr[81].binary_search(&index).is_ok()),
            'r' => Some(self.arr[82].binary_search(&index).is_ok()),
            's' => Some(self.arr[83].binary_search(&index).is_ok()),
            't' => Some(self.arr[84].binary_search(&index).is_ok()),
            'u' => Some(self.arr[85].binary_search(&index).is_ok()),
            'v' => Some(self.arr[86].binary_search(&index).is_ok()),
            'w' => Some(self.arr[87].binary_search(&index).is_ok()),
            'x' => Some(self.arr[88].binary_search(&index).is_ok()),
            'y' => Some(self.arr[89].binary_search(&index).is_ok()),
            'z' => Some(self.arr[90].binary_search(&index).is_ok()),
            '{' => Some(self.arr[91].binary_search(&index).is_ok()),
            '|' => Some(self.arr[92].binary_search(&index).is_ok()),
            '}' => Some(self.arr[93].binary_search(&index).is_ok()),
            '~' => Some(self.arr[94].binary_search(&index).is_ok()),
            '◂' => Some(self.arr[95].binary_search(&index).is_ok()),
            '▸' => Some(self.arr[96].binary_search(&index).is_ok()),
            '▴' => Some(self.arr[97].binary_search(&index).is_ok()),
            '▾' => Some(self.arr[98].binary_search(&index).is_ok()),
            '!' => Some(self.arr[99].binary_search(&index).is_ok()),
            '!' => Some(self.arr[100].binary_search(&index).is_ok()),
            '!' => Some(self.arr[101].binary_search(&index).is_ok()),
            '!' => Some(self.arr[102].binary_search(&index).is_ok()),
            '┌' => Some(self.arr[103].binary_search(&index).is_ok()),
            '┌' => Some(self.arr[104].binary_search(&index).is_ok()),
            '┐' => Some(self.arr[105].binary_search(&index).is_ok()),
            '│' => Some(self.arr[106].binary_search(&index).is_ok()),
            '└' => Some(self.arr[107].binary_search(&index).is_ok()),
            '┘' => Some(self.arr[108].binary_search(&index).is_ok()),
            '█' => Some(self.arr[109].binary_search(&index).is_ok()),
            '▓' => Some(self.arr[110].binary_search(&index).is_ok()),
            '!' => Some(self.arr[111].binary_search(&index).is_ok()),
            '!' => Some(self.arr[112].binary_search(&index).is_ok()),
            '!' => Some(self.arr[113].binary_search(&index).is_ok()),
            '!' => Some(self.arr[114].binary_search(&index).is_ok()),
            '!' => Some(self.arr[115].binary_search(&index).is_ok()),
            '!' => Some(self.arr[116].binary_search(&index).is_ok()),
            '!' => Some(self.arr[117].binary_search(&index).is_ok()),
            '!' => Some(self.arr[118].binary_search(&index).is_ok()),
            '!' => Some(self.arr[119].binary_search(&index).is_ok()),
            '!' => Some(self.arr[120].binary_search(&index).is_ok()),
            '!' => Some(self.arr[121].binary_search(&index).is_ok()),
            '!' => Some(self.arr[122].binary_search(&index).is_ok()),
            '!' => Some(self.arr[123].binary_search(&index).is_ok()),
            '!' => Some(self.arr[124].binary_search(&index).is_ok()),
            '!' => Some(self.arr[125].binary_search(&index).is_ok()),
            '!' => Some(self.arr[126].binary_search(&index).is_ok()),
            '!' => Some(self.arr[127].binary_search(&index).is_ok()),
            _ => None,
        }
    }
}