#include "commands.h"
#include <string_view>

std::string_view to_string(Commands c) {
  switch (c) {
  case Commands::OFF:
    return "OFF";
  case Commands::ON:
    return "ON";
  case Commands::TOGGLE:
    return "TOGGLE";
  }
  return "";
}
