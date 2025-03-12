#include "utils.h"
#include <string>
#include <vector>

std::vector<std::string> split(std::string_view s, std::string_view delim) {
  auto v = std::vector<std::string>();

  for (const auto word : std::views::split(s, delim))
    v.push_back(static_cast<std::string>(std::string_view(word)));

  return v;
}
