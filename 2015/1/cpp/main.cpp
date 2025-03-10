#include <fstream>
#include <iostream>
#include <string>

int main() {
  std::ifstream file;

  file.open("input.txt");

  std::string line = "";

  std::getline(file, line);

  int counter = 0;
  int i = 0;
  int basementFirstEntered = 0;
  bool basementEntered = false;

  for (char c : line) {
    switch (c) {
    case '(':
      counter++;
      break;
    case ')':
      counter--;
      break;
    }
    if (counter < 0 && !basementEntered) {
      basementFirstEntered = i + 1;
      basementEntered = true;
    }
    i++;
  }
  std::cout << "Floor: ";
  std::cout << counter;
  std::cout << "\n";
  std::cout << "Basement First Entered: ";
  std::cout << basementFirstEntered;
  std::cout << "\n";
}
