# Simple datablast

This is a simple version of the datablast idea: just a sequence of QR-codes, each of which contains a chunk of the file, in order.
This can be used to transfer files using videos when on a limited connection except to social networks.

## How to use

1. Prepare the file by splitting it into smaller pieces.
These must have names consisting of a number of the piece.
They must also be not longer than 3436 bytes in size -- this is a limitation of the QR code size and of the a85-encoding.

This can be done with standard `split(1)` as follows:

```bash
split -b 3436 -d -a 100 yourfile.bin ""
```

2. Create QR-codes by running the `encode.py` program, pointing it at the directory containing these file chunks.
It will replace them with images of the QR codes. It is safe to run multiple processes at the same time.

3. Use a program like `ffmpeg` to combine the images into a video.

```bash
ffmpeg -i %d.png outp.mp4
```
4. Transfer the video.

5. Split the video into frames, each of which must have a name of the form `[frame number].[ext]`.

```bash
ffmpeg -i inp.mp4 %d.png
```

6. Run the `decode.py` program, which will parse each of the QR-codes and replace it with the decoded data chunk.
It is safe to run multiple processes at the same time.

7. Concatenate the decoded chunks.
Take care to concatenate them in their numeric order.

```bash
cat `ls | sort -n` > yourfile.bin`
```
