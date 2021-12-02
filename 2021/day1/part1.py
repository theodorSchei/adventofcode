file = open('2021/day1/input.txt', 'r')
lines = file.readlines()

numbers = []

for line in lines:
	numbers.append(int(line.strip()))

total = 0

for previous, current in zip(numbers, numbers[1:]):
	if(current > previous):
		total = total + 1

print(total)

file.close()
