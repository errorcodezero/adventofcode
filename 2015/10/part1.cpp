#include <fstream>
#include <iostream>
#include <memory>
#include <string>

std::string lookAndSay(std::string_view input) {
  int count = 1;
  std::string output = "";
  int prevChar = ' ';
  for (char c : input) {
    if (c == prevChar) {
      count++;
    } else {
      output += std::to_string(count);
      output += c;
    }
    prevChar = c;
  }

  return output;
}

int main() {
  std::ifstream file;
  file.open("input.txt");

  std::string * input = new std::string("");

  std::getline(file, *input);

  for (int i = 0; i < 40; i++) {
	std::cout << i;
    *input = lookAndSay(*input);
  }

  std::cout << *input;
  std::cout << input->length();

  delete input;

  return 0;
}
