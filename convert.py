from PIL import Image

im = Image.open('font1.png') # Can be many different formats.
pix = im.load()
print(im.size)
printstr = "    match c {"
for yoffset in range(4):
    for xoffset in range(32):
        printstr += f"\n'!' => Some(&["
        test = 0
        for j in range(0,8*8):
            x = j%8 + xoffset*8
            y = j/8 + yoffset*8
            color =  pix[x,y]
            if color == 1:
                test += 1
                printstr += f'{j},'
        if test:
            printstr = printstr[:-1]
        printstr += ']),\n'
printstr = printstr[:-1] + '        _ => None,\n}'
print(printstr)
    