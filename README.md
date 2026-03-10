# SERVER GAMES

## protocol

| ID | NAME          | BYTES
| -- | ------------- | -----
| 00 | refresh       | -
| 01 | fill          | RGB
| 02 | image (gzip)  | SSXXYYWWHH{DATA}
| 03 | points        | NRGB(XXYY)
| 04 | fill rects    | NRGB(XXYYWWHH)
| 05 | draw rects    | NRGB(XXYYWWHH)
| 06 | fill ellipses | NRGB(XXYYWWHH)
| 07 | draw ellipses | NRGB(XXYYWWHH)
| 08 | fill polygon  | NRGBXXYY(XXYY)
| 09 | draw polygon  | NRGBXXYY(XXYY)
