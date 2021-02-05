import base64
import qrcode
import os
import random
os.chdir(input('path to chunks: '))

l = os.listdir('.')
random.shuffle(l)
for file in l:
    print(file)
    if '.' in file: continue
    if not os.path.isfile(file): continue
    int(file)
    with open(file, 'rb') as handle:
        data = handle.read()

    data_enc = str(base64.a85encode(data), 'utf-8')
    img = qrcode.make(data_enc)
    img.save(str(int(file)) + '.png')
    os.unlink(file)

