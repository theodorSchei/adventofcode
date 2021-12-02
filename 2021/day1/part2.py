file = open('2021/day1/input.txt', 'r')
lines = file.readlines()

numbers = []

for line in lines:
    numbers.append(int(line.strip()))

total = 0

for i in range(0, len(numbers)-3, 1):
    current = numbers[i] + numbers[i+1] + numbers[i+2]
    next = numbers[i+1] + numbers[i+2] + numbers[i+3]

    if next > current:
        total = total + 1

print(total)

file.close()
