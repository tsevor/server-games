client: clean
	gcc src/client/*.c -o bin/client

test: client
	./bin/client


clean:
	rm -rf bin/
	mkdir -p bin/