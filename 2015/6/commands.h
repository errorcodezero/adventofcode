#pragma once

#include <string>

enum class Commands { ON, OFF, TOGGLE };

std::string_view to_string(Commands c);
