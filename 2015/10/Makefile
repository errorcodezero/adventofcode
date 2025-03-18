FLAGS = -std=c++23 -Wall -g

build/part1.o: part1.cpp
	mkdir -p build
	clang++ -c part1.cpp -o ./build/part1.obj $(FLAGS)
	clang++ ./build/part1.obj -o ./build/part1.o $(FLAGS)

run-part1: build/part1.o
	./build/part1.o

build/part2.o: part2.cpp
	mkdir -p build
	clang++ -c part2.cpp -o ./build/part2.obj $(FLAGS)
	clang++ ./build/part2.obj -o ./build/part2.o $(FLAGS)

run-part2: build/part2.o
	./build/part2.o

clean:
	rm -rf build/
