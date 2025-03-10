#include <fstream>
#include <iostream>
#include <string>

int realLength(std::string &s) {
  std::cout << s << "\n";
  if (s == "") {
    return -2;
  } else if (s.substr(0, 2) == "\\\\") {
    s.erase(0, 2);
    goto finish;
  } else if (s.substr(0, 2) == "\\\"") {
    s.erase(0, 2);
    goto finish;
  } else if (s.substr(0, 2) == "\\x") {
    s.erase(0, 4);
    goto finish;
  } else {
    s.erase(0, 1);
    goto finish;
  }

finish:
  return 1 + realLength(s);
}

int main() {
  std::ifstream file;

  file.open("input.txt");

  std::string line = "";
  int sum = 0;

  while (std::getline(file, line)) {
    int codeLength = line.length();
    sum += codeLength - realLength(line);
    std::cout << sum - codeLength;
    std::cout << " - ";
    std::cout << codeLength;
    std::cout << " = ";
    std::cout << sum << "\n";
  }

  std::cout << sum;

  return 0;
}
