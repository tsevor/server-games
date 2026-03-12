#pragma once

#include <SDL2/SDL.h>
#include <SDL2/SDL2_gfxPrimitives.h>

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

void drawPoint(SDL_Renderer *renderer, int x, int y, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawPoint(renderer, x, y);
}

void fillRect(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderFillRect(renderer, &rect);
}

void drawRect(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawRect(renderer, &rect);
}

void fillEllipse(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	filledEllipseRGBA(renderer, rect.x, rect.y, rect.w, rect.h, color.r, color.g, color.b, color.a);
}

void drawEllipse(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	ellipseRGBA(renderer, rect.x, rect.y, rect.w, rect.h, color.r, color.g, color.b, color.a);
}
