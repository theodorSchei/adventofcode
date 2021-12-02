input = open('day_1/part_1/input.txt')
numbersraw = input.readlines()

numbers = []

for i in numbersraw:
    numbers.append(int(i.rstrip('\n')))

total = 0

for previous, current in zip(numbers, numbers[1:]):
    if(current > previous):
        total = total + 1

print(total)

input.close()