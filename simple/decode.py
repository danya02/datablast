from pyzbar.pyzbar import decode
from PIL import Image
import os
import base64
import random
os.chdir(input('path to chunks: '))

l = os.listdir('.')
random.shuffle(l)
for file in l:
    if not os.path.isfile(file): continue
    if '.' not in file: continue
    print(file)
    img = Image.open(file)
    codes = decode(img)
    data = codes[0].data
    data_dec = base64.a85decode(data)
    with open(file.split('.')[0], 'wb') as handle:
        handle.write(data_dec)
    os.unlink(file)
