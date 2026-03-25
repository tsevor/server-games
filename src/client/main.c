#include <SDL2/SDL.h>
#include <SDL2/SDL_net.h>

#include <stdio.h>
#include <stdlib.h>

#include "drawing.h"
#include "keys.h"

SDL_Window *window = NULL;
SDL_Renderer *renderer = NULL;

IPaddress ip;
char *serverIP;
uint16_t serverPort;

TCPsocket socket;

const uint8_t KEEP_ALIVE = 1;



int secure_recv(TCPsocket sock, void* data, int len) {
        int total = 0;
        uint8_t* ptr = (uint8_t*)data;
        while (total < len) {
                int r = SDLNet_TCP_Recv(sock, ptr + total, len - total);
                if (r <= 0) return -1; // Connection closed or error
                total += r;
        }
        return total;
}

void cleanup() {
        if (renderer) {
                SDL_DestroyRenderer(renderer);
        }
        if (window) {
                SDL_DestroyWindow(window);
        }
        if (socket) {
                SDLNet_TCP_Close(socket);
        }
        SDLNet_Quit();
        SDL_Quit();
}


void send_keyboard() {
        if (!socket) {
                return;
        }
        // send as N([key pressed in ascii])
        int numKeys;
        uint8_t buf[256];
        buf[0] = 0x03; // id
        uint8_t count = 2;
        const uint8_t* state = SDL_GetKeyboardState(&numKeys);
        for (int i = 0; i < numKeys; i++) {
                if (state[i]) {
                        buf[count++] = scancode_to_ascii(i);
                        if (count >= 255) {
                                break;
                        }
                }
        }
        buf[1] = count - 2; // number of keys
        SDLNet_TCP_Send(socket, buf, count);
}


void send_mouse() {
        if (!socket) {
                return;
        }
        int x, y;
        uint32_t buttons = SDL_GetMouseState(&x, &y);
        // send as XXYY[button bits: 87654rml]
        uint8_t buf[5];
        ((uint16_t*)buf)[0] = x;
        ((uint16_t*)buf)[1] = y;
        buf[4] = buttons & 0xff;
        SDLNet_TCP_Send(socket, buf, 5);
}


int main(int argc, char *argv[]) {
        if (argc != 3) {
                SDL_Log("Usage: %s <IP_ADDRESS> <PORT>\n", argv[0]);
                return -1;
        }

        serverIP = argv[1];
        serverPort = atoi(argv[2]);

        if (SDL_Init(SDL_INIT_VIDEO) < 0) {
                SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
                return -1;
        }




        if (SDLNet_Init() < 0) {
                SDL_Log("Failed to initialize SDL_net: %s", SDLNet_GetError());
                SDL_Quit();
                return -1;
        }

        if (SDLNet_ResolveHost(&ip, serverIP, serverPort) < 0) {
                SDL_Log("Failed to resolve host %s:%d : %s", serverIP, serverPort, SDLNet_GetError());
                SDLNet_Quit();
                SDL_Quit();
                return -1;
        }

        socket = SDLNet_TCP_Open(&ip);
        if (!socket) {
                SDL_Log("Failed to connect to %s:%d : %s", serverIP, serverPort, SDLNet_GetError());
                cleanup();
                return -1;
        }

        SDL_Log("Successfully connected to %s:%d", serverIP, serverPort);

        SDLNet_TCP_Send(socket, "hey", 3);
        uint8_t buf[4];
        int r = secure_recv(socket, buf, 3);

        if (r > 0) {
                // ensure the server replied "sup"
                if (r == 3 && strncmp(buf, "sup", 3) == 0) {
                        SDL_Log("Handshake successful!");
                } else {
                        SDL_Log("Unexpected server reply: %.*s", r, buf);
                        cleanup();
                        return -1;
                }
        } else {
                SDL_Log("Failed to receive handshake reply: %s", SDLNet_GetError());
                cleanup();
                return -1;
        }

        SDLNet_SocketSet socketSet = SDLNet_AllocSocketSet(1);
        if (!socketSet) {
                SDL_Log("Failed to allocate socket set: %s", SDLNet_GetError());
                cleanup();
                return -1;
        }
        SDLNet_TCP_AddSocket(socketSet, socket);

        r = secure_recv(socket, buf, 4);
        if (r != 4) {
                SDL_Log("Failed to receive window size: %s", SDLNet_GetError());
                cleanup();
                return -1;
        }

        int w = ((uint16_t*)buf)[0];
        int h = ((uint16_t*)buf)[1];


        SDL_CreateWindowAndRenderer(w, h, 0, &window, &renderer);
        SDL_SetWindowTitle(window, "Server Game Client");

        int app_quit = 0;
        SDL_Event event;

        while (!app_quit) {
                while (SDL_PollEvent(&event)) {
                        if (event.type == SDL_QUIT) {
                                app_quit = 1;
                        }
                }

                if (app_quit) {
                        break;
                }

                int numready = SDLNet_CheckSockets(socketSet, 0);

                // check if there's any data ready on the socket
                if (numready <= 0 || !SDLNet_SocketReady(socket)) {
                        SDL_Delay(1);
                        continue;
                }

                // read one byte for packet type
                uint8_t packet_type;
                int r = secure_recv(socket, &packet_type, 1);

                if (r <= 0) {
                        SDL_Log("Failed to receive packet type: %s", SDLNet_GetError());
                        cleanup();
                        return -1;
                }

                switch (packet_type) {
                case 0x00: // refresh screen
                        refresh(renderer);
                        int result = SDLNet_TCP_Send(socket, &KEEP_ALIVE, sizeof(KEEP_ALIVE));
                        break;
                case 0x01: { // fill screen with RGB color
                        uint8_t color_buf[3];
                        r = secure_recv(socket, color_buf, 3);
                        if (r == 3) {
                                SDL_Color color = {color_buf[0], color_buf[1], color_buf[2], 255};
                                fillScreen(renderer, color);
                        }
                        break;
                }
                case 0x02: { // draw image (LZ4): CS US XX YY WW HH {DATA}
                        uint8_t header[16];
                        if (secure_recv(socket, header, 16) < 0) break;

                        uint32_t comp_len = *(uint32_t*)&header[0];
                        uint32_t uncomp_len = *(uint32_t*)&header[4];
                        int16_t x = *(int16_t*)&header[8];
                        int16_t y = *(int16_t*)&header[10];
                        int16_t w = *(int16_t*)&header[12];
                        int16_t h = *(int16_t*)&header[14];

                        uint8_t *comp_buf = malloc(comp_len);
                        if (secure_recv(socket, comp_buf, comp_len) < 0) {
                                free(comp_buf);
                                break;
                        }

                        blitLZ4Image(renderer, comp_buf, comp_len, uncomp_len, x, y, w, h);

                        free(comp_buf);
                        break;
                }

                case 0x03: { // draw points: NRGB(XXYY)
                        uint8_t header[4];
                        r = secure_recv(socket, header, 4);
                        if (r != 4) break;
                        uint8_t n = header[0];
                        SDL_Color color = {header[1], header[2], header[3], 255};
                        for (int i = 0; i < n; i++) {
                                uint8_t point_buf[4];
                                r = secure_recv(socket, point_buf, 4);
                                if (r != 4) break;
                                int x = *(int16_t*)&point_buf[0];
                                int y = *(int16_t*)&point_buf[2];
                                drawPoint(renderer, x, y, color);
                        }
                        break;
                }
                case 0x04: // fill rects: NRGB(XXYYWWHH)
                case 0x05: // draw rects: NRGB(XXYYWWHH)
                case 0x06: // fill ellipses: NRGB(XXYYWWHH)
                case 0x07: { // draw ellipses: NRGB(XXYYWWHH)
                        uint8_t header[4];
                        r = secure_recv(socket, header, 1);
                        if (r != 1) break;
                        uint8_t n = header[0];
                        for (int i = 0; i < n; i++) {
                                uint8_t rect_buf[11];
                                r = secure_recv(socket, rect_buf, 11);
                                if (r != 11) break;
                                int x = *(int16_t*)&rect_buf[0];
                                int y = *(int16_t*)&rect_buf[2];
                                int w = *(uint16_t*)&rect_buf[4];
                                int h = *(uint16_t*)&rect_buf[6];
                                SDL_Rect rect = {x, y, w, h};
                                SDL_Color color = {rect_buf[8], rect_buf[9], rect_buf[10], 255};
                                if (packet_type == 0x04) {
                                        fillRect(renderer, rect, color);
                                } else if (packet_type == 0x05) {
                                        drawRect(renderer, rect, color);
                                } else if (packet_type == 0x06) {
                                        fillEllipse(renderer, rect, color);
                                } else if (packet_type == 0x07) {
                                        drawEllipse(renderer, rect, color);
                                }
                        }
                        break;
                }

                case 0x80:
                        send_keyboard();
                        break;
                case 0x81:
                        send_mouse();
                        break;

                case 0xc0: { // resize window: WWHH
                        uint8_t header[4];
                        r = secure_recv(socket, header, 4);
                        if (r != 4) break;
                        int w = *(uint16_t*)&header[0];
                        int h = *(uint16_t*)&header[2];
                        SDL_SetWindowSize(window, w, h);
                        break;
                }
                case 0xc1: { // rename window: L(TITLESTRING)
                        uint8_t str_len;
                        r = secure_recv(socket, &str_len, 1);
                        if (r != 1) break;
                        char *title = malloc(str_len + 1);
                        r = secure_recv(socket, title, str_len);
                        if (r != str_len) break;
                        title[str_len] = '\0'; // null terminate
                        SDL_SetWindowTitle(window, title);
                        free(title);
                        break;
                }
                case 0xc2: { // move window: XXYY
                        uint8_t pos_buf[4];
                        r = secure_recv(socket, pos_buf, 4);
                        if (r != 4) break;
                        int x = *(uint16_t*)&pos_buf[0];
                        int y = *(uint16_t*)&pos_buf[2];
                        SDL_SetWindowPosition(window, x, y);
                        break;
                }
                default:
                        SDL_Log("Unknown packet type: 0x%02x", packet_type);
                        break;
                }

        }

        cleanup();

        return 0;
}