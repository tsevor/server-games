#pragma once

#include <SDL2/SDL.h>
#include <SDL2/SDL2_gfxPrimitives.h>

#include <lz4.h>

void refresh(SDL_Renderer *renderer) {
	SDL_RenderPresent(renderer);
}

void blitLZ4Image(SDL_Renderer *renderer, uint8_t *comp_data, uint32_t comp_len, uint32_t uncomp_len, int16_t x, int16_t y, int16_t w, int16_t h) {
	if (w <= 0 || h <= 0 || uncomp_len == 0) return;

	uint8_t *uncomp_data = malloc(uncomp_len);

	int result = LZ4_decompress_safe(comp_data, uncomp_data, comp_len, uncomp_len);
	
	if (result > 0) {
		SDL_Surface *surf = SDL_CreateRGBSurfaceWithFormatFrom(
			uncomp_data, w, h, 32, w * 4, SDL_PIXELFORMAT_RGBA32);
		
		if (surf) {
			SDL_Texture *tex = SDL_CreateTextureFromSurface(renderer, surf);
			if (tex) {
				SDL_Rect dest = {x, y, w, h};
				SDL_RenderCopy(renderer, tex, NULL, &dest);
				SDL_DestroyTexture(tex);
			}
			SDL_FreeSurface(surf);
		}
	} else {
		SDL_Log("LZ4 Decompression Failed!");
	}

	free(uncomp_data);
}

void setRendererColor(SDL_Renderer *renderer, SDL_Color color) {
	SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);
}

void fillScreen(SDL_Renderer *renderer, SDL_Color color) {
	setRendererColor(renderer, color);
	SDL_RenderClear(renderer);
}

void drawPoint(SDL_Renderer *renderer, int16_t x, int16_t y, SDL_Color color) {
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
