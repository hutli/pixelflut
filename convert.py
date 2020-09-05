from PIL import Image

im = Image.open('font1.png') # Can be many different formats.
pix = im.load()
print(im.size)
chars = [' ','!','"','#','$','%','&','\\\'','(',')','*','+',',','-','.','/','0','1','2','3','4','5','6','7','8','9',':',';','<','=','>','?','@','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','[','\\\\',']','^','_','`','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','{','|','}','~','◂','▸','▴','▾','!','!','!','!','┌','┌','┐','│','└','┘','█','▓','!','!','!','!','!','!','!','!','!','!','!','!','!','!','!','!','!']
print(len(chars))
printstr = '''
use std::collections::HashMap;

pub struct Alphabet {
    arr: [HashMap<u8, bool>; 128],
}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {
            arr: [
'''
for yoffset in range(4):
    for xoffset in range(32):
        i = yoffset*32 + xoffset
        printstr += '                vec![\n'
        test = 0
        for j in range(0,8*8):
            x = j%8 + xoffset*8
            y = j/8 + yoffset*8
            color =  pix[x,y]
            if color == 1:
                test += 1
                printstr += str(j) + ','
        if test:
            printstr = printstr[:-1]
        printstr += '\n                ],\n'
printstr += '''
            ],
        }
    }

    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> Option<bool> {
        match c {
'''
for yoffset in range(4):
    for xoffset in range(32):
        i = yoffset*32 + xoffset
        printstr += "'" + chars[i] + "' => Some(self.arr[" + str(i) + "].contains_key(&index)),\n"
printstr = printstr[:-1] + '        _ => None,\n}\n}\n}'
print(printstr)
    