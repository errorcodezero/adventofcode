#include "utils.h"
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

Commands from_string(std::string_view c) {
  if (c.starts_with("turn off")) {
    return Commands::OFF;
  } else if (c.starts_with("turn on")) {
    return Commands::ON;
  } else {
    return Commands::TOGGLE;
  }
}

/*
 * Splits a string into two pieces with a certain delimiter
 */
std::vector<std::string> split(std::string_view s, std::string_view delim) {
  auto v = std::vector<std::string>();
  int start = 0;
  for (long unsigned int i = 0; i < s.length(); i++) {
    if (delim == s.substr(i, delim.length())) {
      v.push_back(static_cast<std::string>(s).substr(start, i - start));
      v.push_back(static_cast<std::string>(s).substr(i + delim.length()));
      break;
    }
  }

  return v;
}
