f = open("input.txt", "r")
input = f.readlines()
f.close()

sum = 0

for line in input:
    nums = []
    print(line)
    line = line.replace("one", "one1one")
    line = line.replace("two", "two2two")
    line = line.replace("three", "three3three")
    line = line.replace("four", "four4four")
    line = line.replace("five", "five5five")
    line = line.replace("six", "six6six")
    line = line.replace("seven", "seven7seven")
    line = line.replace("eight", "eight8eight")
    line = line.replace("nine", "nine9nine")
    for char in line:
        try:
            nums.append(int(char))
        except Exception:
            pass
    sum += 10 * nums[0] + nums[-1]
    print(line)
    print(nums[0], nums[-1])

print(sum)
