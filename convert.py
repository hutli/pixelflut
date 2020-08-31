from PIL import Image

im = Image.open('font1.png') # Can be many different formats.
pix = im.load()
print(im.size)
printstr = "let alphabet:Vec<Vec<i32>> = vec!["
for yoffset in range(4):
    for xoffset in range(32):
        printstr += f"vec!["
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
        printstr += '],'
printstr = printstr[:-1] + '];'
print(printstr)
    