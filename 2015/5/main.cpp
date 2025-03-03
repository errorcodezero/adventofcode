#include <fstream>
#include <iostream>

int main() {
  std::ifstream file;

  file.open("input.txt");

  if (!file.is_open()) {
    return 1;
  }

  std::string line;
  while (std::getline(file, line)) {
    for (auto c : line) {
      std::cout << c;
    }
    std::cout << "\n";
  }

  return 0;
}
