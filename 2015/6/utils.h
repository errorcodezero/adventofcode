#pragma once

#include <string>
#include <string_view>
#include <vector>

enum class Commands { ON, OFF, TOGGLE };

Commands from_string(std::string_view c);

std::vector<std::string> split(std::string_view s, std::string_view delim);
