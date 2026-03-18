#include <stdint.h>
#include <SDL2/SDL.h>


uint8_t scancode_to_ascii(uint8_t scancode) {
	switch (scancode) {
		case SDL_SCANCODE_ESCAPE: return 27;
		case SDL_SCANCODE_RETURN: return 13;
		case SDL_SCANCODE_BACKSPACE: return 8;
		case SDL_SCANCODE_TAB: return 9;
		case SDL_SCANCODE_SPACE: return 32;

		case SDL_SCANCODE_1: return '1';
		case SDL_SCANCODE_2: return '2';
		case SDL_SCANCODE_3: return '3';
		case SDL_SCANCODE_4: return '4';
		case SDL_SCANCODE_5: return '5';
		case SDL_SCANCODE_6: return '6';
		case SDL_SCANCODE_7: return '7';
		case SDL_SCANCODE_8: return '8';
		case SDL_SCANCODE_9: return '9';
		case SDL_SCANCODE_0: return '0';

		case SDL_SCANCODE_A: return 'a';
		case SDL_SCANCODE_B: return 'b';
		case SDL_SCANCODE_C: return 'c';
		case SDL_SCANCODE_D: return 'd';
		case SDL_SCANCODE_E: return 'e';
		case SDL_SCANCODE_F: return 'f';
		case SDL_SCANCODE_G: return 'g';
		case SDL_SCANCODE_H: return 'h';
		case SDL_SCANCODE_I: return 'i';
		case SDL_SCANCODE_J: return 'j';
		case SDL_SCANCODE_K: return 'k';
		case SDL_SCANCODE_L: return 'l';
		case SDL_SCANCODE_M: return 'm';
		case SDL_SCANCODE_N: return 'n';
		case SDL_SCANCODE_O: return 'o';
		case SDL_SCANCODE_P: return 'p';
		case SDL_SCANCODE_Q: return 'q';
		case SDL_SCANCODE_R: return 'r';
		case SDL_SCANCODE_S: return 's';
		case SDL_SCANCODE_T: return 't';
		case SDL_SCANCODE_U: return 'u';
		case SDL_SCANCODE_V: return 'v';
		case SDL_SCANCODE_W: return 'w';
		case SDL_SCANCODE_X: return 'x';
		case SDL_SCANCODE_Y: return 'y';
		case SDL_SCANCODE_Z: return 'z';

		case SDL_SCANCODE_MINUS: return '-';
		case SDL_SCANCODE_EQUALS: return '=';
		case SDL_SCANCODE_LEFTBRACKET: return '[';
		case SDL_SCANCODE_RIGHTBRACKET: return ']';
		case SDL_SCANCODE_BACKSLASH: return '\\';
		case SDL_SCANCODE_SEMICOLON: return ';';
		case SDL_SCANCODE_APOSTROPHE: return '\'';
		case SDL_SCANCODE_GRAVE: return '`';
		case SDL_SCANCODE_COMMA: return ',';
		case SDL_SCANCODE_PERIOD: return '.';
		case SDL_SCANCODE_SLASH: return '/';

		case SDL_SCANCODE_UP: return 128;
		case SDL_SCANCODE_DOWN: return 129;
		case SDL_SCANCODE_LEFT: return 130;
		case SDL_SCANCODE_RIGHT: return 131;

		case SDL_SCANCODE_LCTRL: return 132;
		case SDL_SCANCODE_LSHIFT: return 133;
		case SDL_SCANCODE_LALT: return 134;
		case SDL_SCANCODE_LGUI: return 135;
		case SDL_SCANCODE_RCTRL: return 136;
		case SDL_SCANCODE_RSHIFT: return 137;
		case SDL_SCANCODE_RALT: return 138;
		case SDL_SCANCODE_RGUI: return 139;

		case SDL_SCANCODE_CAPSLOCK: return 140;

		case SDL_SCANCODE_F1: return 141;
		case SDL_SCANCODE_F2: return 142;
		case SDL_SCANCODE_F3: return 143;
		case SDL_SCANCODE_F4: return 144;
		case SDL_SCANCODE_F5: return 145;
		case SDL_SCANCODE_F6: return 146;
		case SDL_SCANCODE_F7: return 147;
		case SDL_SCANCODE_F8: return 148;
		case SDL_SCANCODE_F9: return 149;
		case SDL_SCANCODE_F10: return 150;
		case SDL_SCANCODE_F11: return 151;
		case SDL_SCANCODE_F12: return 152;

		//number pad keys
		case SDL_SCANCODE_KP_1: return 153;
		case SDL_SCANCODE_KP_2: return 154;
		case SDL_SCANCODE_KP_3: return 155;
		case SDL_SCANCODE_KP_4: return 156;
		case SDL_SCANCODE_KP_5: return 157;
		case SDL_SCANCODE_KP_6: return 158;
		case SDL_SCANCODE_KP_7: return 159;
		case SDL_SCANCODE_KP_8: return 160;
		case SDL_SCANCODE_KP_9: return 161;
		case SDL_SCANCODE_KP_0: return 162;
		case SDL_SCANCODE_KP_PERIOD: return 163;
		case SDL_SCANCODE_KP_DIVIDE: return 164;
		case SDL_SCANCODE_KP_MULTIPLY: return 165;
		case SDL_SCANCODE_KP_MINUS: return 166;
		case SDL_SCANCODE_KP_PLUS: return 167;
		case SDL_SCANCODE_KP_ENTER: return 168;

		case SDL_SCANCODE_INSERT: return 169;
		case SDL_SCANCODE_HOME: return 170;
		case SDL_SCANCODE_PAGEUP: return 171;
		case SDL_SCANCODE_DELETE: return 172;
		case SDL_SCANCODE_END: return 173;
		case SDL_SCANCODE_PAGEDOWN: return 174;

		default:
			return 0;
	}
}