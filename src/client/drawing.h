#pragma once

#include <SDL2/SDL.h>

void refresh(SDL_Renderer *renderer) {
	SDL_RenderPresent(renderer);
}

void setRendererColor(SDL_Renderer *renderer, SDL_Color color) {
	SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);
}

void fillScreen(SDL_Renderer *renderer, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderClear(renderer);
}

void fillRect(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderFillRect(renderer, &rect);
}

void fillRects(SDL_Renderer *renderer, SDL_Rect rect[], int num, SDL_Color color) {
	setRendererColor(renderer, color);
	for (int i = 0; i < num; i++) {
		SDL_RenderFillRect(renderer, &(rect[i]));
	}
}