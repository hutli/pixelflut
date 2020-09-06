from PIL import Image

def generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end):
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
    print(printstr)

def bit_map():
    match_arm_prefix = "0b"
    match_arm_suffix = "u64,"
    if_present = lambda x: "1"
    if_not_present = lambda: "0"
    end = '''((x << index)
                    & 0x8000000000000000)
                    > 0x1
                '''
    generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end)

def binary_search():
    match_arm_prefix = "["
    match_arm_suffix = "].binary_search(&index).is_ok(),"
    if_present = lambda x: str(x) + ","
    if_not_present = lambda: ""
    end = '''x'''
    generic_printer(match_arm_prefix, match_arm_suffix, if_present, if_not_present, end)


im = Image.open('font1.png') # Can be many different formats.
pix = im.load()
chars = [' ','!','"','#','$','%','&','\\\'','(',')','*','+',',','-','.','/','0','1','2','3','4','5','6','7','8','9',':',';','<','=','>','?'
        ,'@','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','[','\\\\',']','^','_'
        ,'`','a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','{','|','}','~', '🙊',
        '◂','▸','▴','▾','▪','🞄','●','┌','━','┐','│','└','┘','█','▓','Ω','µ','▁','▂','▎','▌','😼','🙎','🙍','🙌','🙋','🙀','😵','😦','😣','😏','😀']
print(len(chars))
# binary_search()
bit_map()

