f = open("input.txt", "r")
input = f.readline()
f.close()

coords = [(0, 0), (0, 0)]
x = 0
y = 0

robo_x = 0
robo_y = 0

for i in range(len(input)):
    if (i % 2 == 0):
        match input[i]:
            case "^":
                y += 1
            case "v":
                y -= 1
            case "<":
                x -= 1
            case ">":
                x += 1
        coords.append((x, y))

    else:
        match input[i]:
            case "^":
                robo_y += 1
            case "v":
                robo_y -= 1
            case "<":
                robo_x -= 1
            case ">":
                robo_x += 1
        coords.append((robo_x, robo_y))

print(f"Total Houses: {len(set(coords))}")
