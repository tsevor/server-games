client: clean
	gcc src/client/*.c -lSDL3 -o bin/client

windows: clean
	x86_64-w64-mingw32-gcc src/client/*.c -lSDL3 -o bin/client.exe

all: client windows

test: client
	./bin/client


clean:
	rm -rf bin/
	mkdir -p bin/