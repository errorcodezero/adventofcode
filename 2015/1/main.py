f = open("input1.txt", "r")
input = f.readline()

floors = 0
first_basement = True

for i in range(0, len(input)):
    match input[i]:
        case "(":
            floors += 1
        case ")":
            floors -= 1
    if floors < 0 and first_basement is True:
        print(f"First basement entry: {i + 1}")
        first_basement = False

print(f"Floors: {floors}")
