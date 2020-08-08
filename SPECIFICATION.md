# Specification v.0

A **video** is an addressable sequence of **frames**, each of which is an image file.
It may not actually be stored this way on disk, but it must be accessible to FFmpeg in this way.

Each frame contains any number of **symbols**, and it may not contain any at all.
Each symbol is a QR code symbol containing data encoded in the "binary/byte" mode.
Depending on context, this data can be interpreted as ASCII-encoded text.
(TODO: support other types of symbol formats)

A **sequence** is an ordered list of symbols.
A single sequence is identified by a single number between 0 and 255 inclusive.

All the symbols belonging to a sequence must exist in a single contiguous set of frames in a video.
There may be at most one sequence in one video file.
(TODO: remove requirement for contiguity and single-sequence)

## Symbols

There are two types of symbols: **content** symbols and **meta** symbols.

### Content

A content symbol contains a part of the binary data encoded by this sequence.

In a content symbol:
- first the number of this sequence is written as two lowercase base-16 numbers is written;
- then there is an arbitrary number of lowercase base-16 numbers representing this symbol's sequence number (the data sequence numbering starts from 0, and is distinct from the frame numbering);
- then a '@' (commercial at, ASCII 0x40) is written;
- the rest of the symbol's content is Base64-encoded binary data for this content symbol.

To reassemble the file, one needs to concatenate the binary data in each of the content symbols, in ascending order of the sequence number.

### Meta

A meta symbol contains a JSON string. It must contain these fields:

- `ver`: integer, the version of the specification used to encode this sequence. Equals the literal value `0`.
- `seq_id`: integer, must be between 0 and 255 inclusive. Corresponds to the sequence number in the data symbols.
- `frames`: integer, the number of frames (including meta frames, and including this one) used to encode this sequence. 
- `cur_frame`: integer, the number of this frame as an offset from the start of this sequence. The first frame is 0.
- `content_len`: array containing 2 elements:
        - 0: integer, the length of the encoded file in bytes;
        - 1: integer, the number of content symbols in this sequence.
- `sha3`: string, containing 64 characters in "0123456789abcdef", representing the SHA3-256 hash of the file.
- `name`: string, the name of the resulting file.

There must be at least one meta symbol in the sequence.
It's a good idea to include one regularly throughout the sequence, as it allows the start of the sequence to be located quicker.
