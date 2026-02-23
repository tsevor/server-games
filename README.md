# SERVER GAMES

## protocol

| ID | NAME          | BYTES
| -- | ------------- | -----
| 00 | refresh       | -
| 01 | fill          | RGB
| 02 | image (gzip)  | SSXYWH{DATA}
| 03 | points        | NRGB(XY)
| 04 | fill rects    | NRGB(XYWH)
| 05 | draw rects    | NRGB(XYWH)
| 06 | fill ellipses | NRGB(XYWH)
| 07 | draw ellipses | NRGB(XYWH)
| 08 | fill polygon  | NRGBXY(XY)
| 09 | draw polygon  | NRGBXY(XY)
