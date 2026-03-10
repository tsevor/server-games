#include <SDL2/SDL.h>
#include <SDL2/SDL_net.h>

#include <stdio.h>
#include <stdlib.h>


#include "drawing.h"

SDL_Window* window = NULL;
SDL_Renderer* renderer = NULL;

IPaddress ip;
uint16_t server_port;

TCPsocket socket;

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


int main(int argc, char* argv[]) {
	if (argc != 3) {
		SDL_Log("Usage: %s <IP_ADDRESS> <PORT>\n", argv[0]);
		return -1;
	}

	char* server_ip = argv[1];
	server_port = atoi(argv[2]);
	
	if (SDL_Init(SDL_INIT_VIDEO) < 0) {
		SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
		return -1;
	}

	
	SDL_CreateWindowAndRenderer(640, 480, 0, &window, &renderer);
	SDL_SetWindowTitle(window, "Server Game Client");


	if (SDLNet_Init() < 0) {
		SDL_Log("Failed to initialize SDL_net: %s", SDLNet_GetError());
		SDL_Quit();
		return -1;
	}

	if (SDLNet_ResolveHost(&ip, server_ip, server_port) < 0) {
		SDL_Log("Failed to resolve host %s:%d : %s", server_ip, server_port, SDLNet_GetError());
		SDLNet_Quit();
		SDL_Quit();
		return -1;
	}
	
	socket = SDLNet_TCP_Open(&ip);
	if (!socket) {
		SDL_Log("Failed to connect to %s:%d : %s", server_ip, server_port, SDLNet_GetError());
		cleanup();
		return -1;
	}

	SDL_Log("Successfully connected to %s:%d", server_ip, server_port);

	SDLNet_TCP_Send(socket, "hey", 3);
	uint8_t buf[1024];
	int r = SDLNet_TCP_Recv(socket, buf, 7);

	if (r > 0) {
		// ensure the server replied "hey bud"
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
		int r = SDLNet_TCP_Recv(socket, &packet_type, 1);

		if (r <= 0) {
			SDL_Log("Failed to receive packet type: %s", SDLNet_GetError());
			cleanup();
			return -1;
		}

		switch (packet_type) {
		case 0x00: // refresh screen
			refresh(renderer);
			break;
		case 0x01: { // fill screen with RGB color
			uint8_t color_buf[3];
			r = SDLNet_TCP_Recv(socket, color_buf, 3);
			if (r == 3) {
				SDL_Color color = {color_buf[0], color_buf[1], color_buf[2], 255};
				fillScreen(renderer, color);
			}
			break;
		}
		case 0x03: { // draw points: NRGB(XXYY)
			uint8_t header[1];
			r = SDLNet_TCP_Recv(socket, header, 1);
			if (r != 1) break;
			uint8_t n = header[0];
			for (int i = 0; i < n; i++) {
				uint8_t point_buf[7];
				r = SDLNet_TCP_Recv(socket, point_buf, 7);
				if (r != 7) break;
				SDL_Color color = {point_buf[0], point_buf[1], point_buf[2], 255};
				int x = point_buf[3] | (point_buf[4] << 8);
				int y = point_buf[5] | (point_buf[6] << 8);
				drawPoint(renderer, x, y, color);
			}
			break;
		}
		case 0x04: // fill rects: NRGB(XXYYWWHH)
		case 0x05: { // draw rects: NRGB(XXYYWWHH)
			uint8_t header[1];
			r = SDLNet_TCP_Recv(socket, header, 1);
			if (r != 1) break;
			uint8_t n = header[0];
			for (int i = 0; i < n; i++) {
				uint8_t rect_buf[11];
				r = SDLNet_TCP_Recv(socket, rect_buf, 11);
				if (r != 11) break;
				SDL_Color color = {rect_buf[0], rect_buf[1], rect_buf[2], 255};
				int x = rect_buf[3] | (rect_buf[4] << 8);
				int y = rect_buf[5] | (rect_buf[6] << 8);
				int w = rect_buf[7] | (rect_buf[8] << 8);
				int h = rect_buf[9] | (rect_buf[10] << 8);
				SDL_Rect rect = {x, y, w, h};
				if (packet_type == 0x04) {
					fillRect(renderer, rect, color);
				} else {
					drawRect(renderer, rect, color);
				}
			}
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
