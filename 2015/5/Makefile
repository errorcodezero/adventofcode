build/part1.o: part1.cpp
	mkdir -p build
	g++ part1.cpp -o ./build/part1.o -pedantic-errors -O2 -std=c++17

run-part1: build/part1.o
	./build/part1.o

build/part2.o: part2.cpp
	mkdir -p build
	g++ part2.cpp -o ./build/part2.o -pedantic-errors -O2 -std=c++17

run-part2: build/part2.o
	./build/part2.o

clean:
	rm -rf build/
