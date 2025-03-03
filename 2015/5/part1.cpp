#include <fstream>
#include <iostream>
#include <string>

int main() {
  std::ifstream file;

  file.open("input.txt");

  if (!file.is_open()) {
    return 1;
  }

  std::string line;
  int nice = 0;
  while (std::getline(file, line)) {
    bool naughty = false;

    // counter for vowels as three vowels is minimum
    int vowels = 0;

    // if a character that makes up the first character of a disallowed
    // substring is found, this will be populated with the next character
    // needed for the disallowed substring to be in the line.
    char next_bad = ' ';

    // Set to the previous character in the string and is used to check if there
    // is a repetition of letters as per the requirements to be a nice string
    char prev = ' ';

    bool repeats = false;

    for (char c : line) {
      if (c == next_bad) {
        naughty = true;
        break;
      } else {
        next_bad = ' ';
      }
      if (c == prev) {
        repeats = true;
      }

      switch (c) {
      case 'a': {
        next_bad = 'b';
        vowels++;
        break;
      }
      case 'e':
      case 'i':
      case 'o':
      case 'u':
        vowels++;
        break;

      case 'c':
        next_bad = 'd';
        break;
      case 'p':
        next_bad = 'q';
        break;
      case 'x':
        next_bad = 'y';
        break;
      }

      prev = c;
    }

    if (vowels < 3 || !repeats) {
      naughty = true;
    }

    if (!naughty)
      nice++;

    std::cout << line << ": " << (naughty ? "NAUGHTY" : "NICE") << "\n";
  }

  std::cout << "Nice strings: " << nice << "\n";

  return 0;
}
