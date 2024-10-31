f = open("input.txt", "r")
input = f.readline()
f.close()

coords = [(0, 0)]
x = 0
y = 0

for i in input:
    match i:
        case "^":
            y += 1
        case "v":
            y -= 1
        case "<":
            x -= 1
        case ">":
            x += 1

    coords.append((x, y))

print(f"Total Houses: {len(set(coords))}")
