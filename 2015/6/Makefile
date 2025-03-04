FLAGS = -std=c++17 -Wall

build/part1.o: part1.cpp commands.cpp
	mkdir -p build
	g++ -c commands.cpp -o ./build/commands.obj $(FLAGS)
	g++ -c part1.cpp -o ./build/part1.obj $(FLAGS)
	g++ ./build/part1.obj ./build/commands.obj -o ./build/part1.o $(FLAGS)

run-part1: build/part1.o
	./build/part1.o

build/part2.o: part2.cpp commands.cpp
	mkdir -p build
	g++ -c commands.cpp -o ./build/commands.obj $(FLAGS)
	g++ -c part2.cpp -o ./build/part2.obj $(FLAGS)
	g++ ./build/part2.obj ./build/commands.obj -o ./build/part2.o $(FLAGS)

run-part2: build/part2.o
	./build/part2.o

clean:
	rm -rf build/
