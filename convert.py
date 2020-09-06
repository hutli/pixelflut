from PIL import Image

def generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end, fileName):
    printstr = '''
        pub struct Alphabet {
        }

        impl Alphabet {
            pub fn new() -> Alphabet {
                Alphabet { }
            }
            pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool {
                let x = match c {
    '''
    for yoffset in range(4):
        for xoffset in range(32):
            i = yoffset*32 + xoffset
            printstr += "\n" + "'" + chars[i] + "' => " + match_arm_prefix
            test = 0
            for j in range(0,8*8):
                x = j%8 + xoffset*8
                y = j/8 + yoffset*8
                color =  pix[x,y]
                if color == 1:
                    test += 1
                    printstr += if_present(j)
                else:
                    printstr += if_not_present()
            
            printstr += match_arm_suffix
    # printstr = printstr[:-1] + '        _ => None,\n}\n}\n}'
    printstr += "\n_ => return false};\n"

    printstr += end
    
    printstr += "}}"
    save_result(fileName, printstr)

def save_result(fileName, printstr):
    with open("src/" + fileName, 'w') as f:
        f.write(printstr)

def bit_map():
    match_arm_prefix = "0b"
    match_arm_suffix = "u64,"
    if_present = lambda x: "1"
    if_not_present = lambda: "0"
    end = '''((x << index)
                    & 0x8000000000000000)
                    > 0x1
                '''
    generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end, "alphabet_bitarr.rs")

def binary_search():
    match_arm_prefix = "["
    match_arm_suffix = "].binary_search(&index).is_ok(),"
    if_present = lambda x: str(x) + ","
    if_not_present = lambda: ""
    end = '''x'''
    generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end, "alphabet_fast.rs")

def bit_arr():
    printstr = '''
use bitvec::{bitarr, bits, order::Lsb0, slice::BitSlice};
use bitvec::prelude::LocalBits;

pub struct Alphabet {
    data: [bitarr!(for 64, in Lsb0, u8); 128],
}

impl Alphabet {
    pub fn new() -> Alphabet {
        Alphabet {
            data: ['''
    for yoffset in range(4):
        for xoffset in range(32):
            i = yoffset*32 + xoffset
            printstr += "\n            bitarr![Lsb0,u8;"
            test = 0
            for j in range(0,8*8):
                x = j%8 + xoffset*8
                y = j/8 + yoffset*8
                color =  pix[x,y]
                if color == 1:
                    test += 1
                    printstr += '1,'
                else:
                    printstr += '0,'
            if test:
                printstr = printstr[:-1]
            printstr += '],'
    printstr += '''
],
        }
    }

    pub fn is_pixel_in_char(&mut self, c: char, index: u8) -> bool {
        let ans = match c {
'''
    for yoffset in range(4):
        for xoffset in range(32):
            i = yoffset*32 + xoffset
            printstr += "'" + chars[i] + "' => self.data[" + str(i) + "],\n"
    printstr = printstr[:-1] + '''
    _ => self.data[0],
    }
    ans.get(index as usize) == Some(&true)
    }
    }'''
    save_result("alphabet_bitarr_2.rs", printstr)

im = Image.open('../font1.png') # Can be many different formats.
pix = im.load()
chars = [' ','!','"','#','$','%','&','\\\'','(',')','*','+',',','-','.','/','0','1','2','3','4','5','6','7','8','9',':',';','<','=','>','?',
        
        '@','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','[','\\\\',']','^','_',
        
        '`','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','{','|','}','~', '🙊',

        '◂','▸','▴','▾','▪','🞄','●','⬤','┌','━','┐','│','└','┘','█','▓','Ω','µ','▁','▂','▎','▌','😼','🙎','🙍','🙌','🙋','🙀','😵','😦','😣','😏','😀']
print(len(chars))
# binary_search()
bit_map()
binary_search()
bit_arr()