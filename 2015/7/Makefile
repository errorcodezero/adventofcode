FLAGS = -std=c++20 -Wall -O2

build/part1.o: part1.cpp
	mkdir -p build
	g++ -c part1.cpp -o ./build/part1.obj $(FLAGS)
	g++ -c utils.cpp -o ./build/utils.obj $(FLAGS)
	g++ ./build/part1.obj ./build/utils.obj -o ./build/part1.o $(FLAGS)

run-part1: build/part1.o
	./build/part1.o

build/part2.o: part2.cpp
	mkdir -p build
	g++ -c part2.cpp -o ./build/part2.obj $(FLAGS)
	g++ -c utils.cpp -o ./build/utils.obj $(FLAGS)
	g++ ./build/part2.obj ./build/utils.obj -o ./build/part2.o $(FLAGS)

run-part2: build/part2.o
	./build/part2.o

clean:
	rm -rf build/
