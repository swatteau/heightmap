import sys
import math
from tkinter import Tk, Canvas, PhotoImage, mainloop

def hexcolor(r, g, b):
    return '#%02x%02x%02x' % (r, g, b)

def main(args):

    try:
        path = args[1]
    except:
        path = "/home/sebastien/terrain.dat"

    print(path)
    with open(path, 'rb') as f:
        bytes = f.read()

    size = int(math.sqrt(len(bytes)))
    window = Tk()
    canvas = Canvas(window, width=size, height=size, bg="#000000")
    canvas.pack()
    img = PhotoImage(width=size, height=size)
    canvas.create_image((size/2, size/2), image=img, state="normal")

    b = 0
    for y in range(size):
        for x in range(size):
            h = bytes[b]
            b+=1
            if h < 60:
                img.put(hexcolor(int(0.1*h), int(0.5*h), max(20,h)), (x, y))
            elif h == 60:
                img.put(hexcolor(int(0.5*h), int(0.5*h), h), (x, y))
            elif h == 61:
                img.put(hexcolor(h, h, h), (x, y))
            elif h == 62:
                img.put(hexcolor(int(2*h), int(1.5*h), int(0.85*h)), (x, y))
            else:
                if h % 10 == 0:
                    #img.put(hexcolor(140,80,80), (x, y))
                    img.put(hexcolor(int(0.23*h), int(0.46*h), 0), (x, y))
                else:
                    img.put(hexcolor(int(0.25*h), int(0.5*h), 0), (x, y))

    mainloop()

if __name__ == "__main__":
    sys.exit(main(sys.argv))
