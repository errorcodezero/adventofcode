f = open("input.txt", "r")
input = f.readlines()
f.close()

total_paper = 0
total_ribbon = 0

for i in range(0, len(input)):
    a = list(input[i].split("x"))
    length = int(a[0])
    width = int(a[1])
    height = int(a[2])
    dimensions = [length, width, height]
    dimensions.sort()

    total_paper += (
        (3 * dimensions[0] * dimensions[1])
        + (2 * dimensions[1] * dimensions[2])
        + (2 * dimensions[0] * dimensions[2])
    )
    total_ribbon += (2 * dimensions[0]) + (2 * dimensions[1])
    total_ribbon += dimensions[0] * dimensions[1] * dimensions[2]

print(f"Wrapping Paper to Order: {total_paper} ft^2")
print(f"Ribbon to Order: {total_ribbon} ft")
