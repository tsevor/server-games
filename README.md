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
| 02 | image (LZ4)    | SSSSXXYYWWHH{RGBA8888}
| 03 | points         | N(XXYYRGB)
| 04 | fill rects     | N(XXYYWWHHRGB)
| 05 | draw rects     | N(XXYYWWHHRGB)
| 06 | fill ellipses  | N(XXYYWWHHRGB)
| 07 | draw ellipses  | N(XXYYWWHHRGB)
| 08 | fill polygon   | NXXYY(XXYYRGB)
| 09 | draw polygon   | NXXYY(XXYYRGB)
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