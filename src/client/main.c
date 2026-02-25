#include <SDL2/SDL.h>

#include "drawing.h"

int main(int argc, char* argv[]) {
	SDL_Window* window = NULL;
	SDL_Renderer* renderer = NULL;

	// Initialize SDL
	if (SDL_Init(SDL_INIT_VIDEO) < 0) {
		SDL_Log("Failed to initialize SDL: %s", SDL_GetError());
		return -1;
	}

	SDL_CreateWindowAndRenderer(640, 480, 0, &window, &renderer);

	SDL_SetWindowTitle(window, "Server Game Client");

	int app_quit = 0;
	
	SDL_Event event;
	while (!app_quit) {
		while (SDL_PollEvent(&event)) {
			if (event.type == SDL_QUIT) {
				app_quit = 1;
			}
		}

		SDL_Color c = {0, 0, 0, 0};
		fillScreen(renderer, c);
		SDL_Rect r = {10, 10, 20, 20};
		*(uint32_t*)&c = 0xffffffff;
		fillRect(renderer, r, c);

		refresh(renderer);
	}

	// Clean up in reverse order of creation
	SDL_DestroyRenderer(renderer);
	SDL_DestroyWindow(window);
	SDL_Quit();

	return 0;
}
