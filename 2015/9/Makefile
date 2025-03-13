FLAGS = -std=c++23 -Wall -O2

build/part1.o: part1.cpp utils.cpp node.cpp node.h utils.h
	mkdir -p build
	g++ -c part1.cpp -o ./build/part1.obj $(FLAGS)
	g++ -c node.cpp -o ./build/node.obj $(FLAGS)
	g++ -c utils.cpp -o ./build/utils.obj $(FLAGS)
	g++ ./build/part1.obj ./build/utils.obj ./build/node.obj -o ./build/part1.o $(FLAGS)

run-part1: build/part1.o
	./build/part1.o

build/part2.o: part2.cpp utils.cpp node.cpp node.h utils.h
	mkdir -p build
	g++ -c part2.cpp -o ./build/part2.obj $(FLAGS)
	g++ -c node.cpp -o ./build/node.obj $(FLAGS)
	g++ -c utils.cpp -o ./build/utils.obj $(FLAGS)
	g++ ./build/part2.obj ./build/utils.obj ./build/node.obj -o ./build/part2.o $(FLAGS)

run-part2: build/part2.o
	./build/part2.o

clean:
	rm -rf build/
