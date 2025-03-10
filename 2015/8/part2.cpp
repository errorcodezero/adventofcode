#include <fstream>
#include <iostream>
#include <string>
#include <string_view>

int newLength(std::string_view s) {
  int length = 2;

  for (char c : s) {
    if (c == '\\' || c == '"') {
      length += 2;
    } else {
      length += 1;
    }
  }

  return length;
}

int main() {
  std::ifstream file;

  file.open("input.txt");

  std::string line = "";
  int sum = 0;

  while (std::getline(file, line)) {
    int codeLength = line.length();
    sum += newLength(line) - codeLength;
    std::cout << newLength(line);
    std::cout << " - ";
    std::cout << codeLength;
    std::cout << " = ";
    std::cout << newLength(line) - codeLength << "\n";
  }

  std::cout << sum;

  return 0;
}
