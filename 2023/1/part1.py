f = open("input.txt", "r")
input = f.readlines()
f.close()

sum = 0

for line in input:
    nums = []
    for char in line:
        try:
            nums.append(int(char))
        except Exception:
            pass
    sum += 10 * nums[0] + nums[-1]

print(sum)
