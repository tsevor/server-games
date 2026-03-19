# SERVER GAMES

## protocol

initial handshake

client says `hey`

server responds with `supWWHH` the `WWHH` being the initial window size

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
| 81 | request mouse  | -
| C0 | resize window  | WWHH
| C1 | rename window  | L(TITLESTRING)
| C2 | move window    | XXYY


### client to server

| ID | NAME           | BYTES
| -- | -------------- | -----
| 00 | -------        | -
| 01 | keep alive     | -
| 02 | key released   | K
| 03 | keyboard state | N([key pressed in ascii])
| 04 | mouse state    | XXYY[button bits: 87654rml]

key codes in `src/client/keys.h`