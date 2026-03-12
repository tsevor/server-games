# SERVER GAMES

## protocol

client says `hey`

server responds with `sup`

### server to client

| ID | NAME           | BYTES
| -- | -------------- | -----
| 00 | refresh        | -
| 01 | fill           | RGB
| 02 | image (gzip)   | SSXXYYWWHH{DATA}
| 03 | points         | NRGB(XXYY)
| 04 | fill rects     | NRGB(XXYYWWHH)
| 05 | draw rects     | NRGB(XXYYWWHH)
| 06 | fill ellipses  | NRGB(XXYYWWHH)
| 07 | draw ellipses  | NRGB(XXYYWWHH)
| 08 | fill polygon   | NRGBXXYY(XXYY)
| 09 | draw polygon   | NRGBXXYY(XXYY)
| 80 | request kb     | -

### client to server

| ID | NAME           | BYTES
| -- | -------------- | -----
| 00 | -------        | -
| 01 | key pressed    | K
| 02 | key released   | K
| 03 | mouse state    | XXYY[button bits: 87654mrl]
| 04 | keyboard state | every K that is pressed


key codes in `src/client/keys.h`