FLAGS = -std=c++20 -Wall -O2

build/main.o: main.cpp
	mkdir -p build
	g++ -c main.cpp -o ./build/main.obj $(FLAGS)
	g++ ./build/main.obj -o ./build/main.o $(FLAGS)

run-part1: build/main.o
	./build/main.o

build/part2.o: main.cpp
	mkdir -p build
	g++ -c main.cpp -o ./build/main.obj $(FLAGS)
	g++ ./build/main.obj -o ./build/main.o $(FLAGS)

run-part2: build/main.o
	./build/main.o

clean:
	rm -rf build/

run: build/main.o
	./build/main.o
