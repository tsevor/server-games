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

void drawPoint(SDL_Renderer *renderer, int x, int y, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawPoint(renderer, x, y);
}

void drawPoints(SDL_Renderer *renderer, SDL_Point points[], int num, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawPoints(renderer, points, num);
}

void fillRect(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderFillRect(renderer, &rect);
}

void fillRects(SDL_Renderer *renderer, SDL_Rect rect[], int num, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderFillRects(renderer, rect, num);
}

void drawRect(SDL_Renderer *renderer, SDL_Rect rect, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawRect(renderer, &rect);
}

void drawRects(SDL_Renderer *renderer, SDL_Rect rect[], int num, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderDrawRects(renderer, rect, num);
}