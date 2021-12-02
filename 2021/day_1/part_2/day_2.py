input = open('day_1/part_2/input.txt')
numbersraw = input.readlines()

numbers = []

for i in numbersraw:
    numbers.append(int(i.rstrip('\n')))

total = 0

for i in range(0, len(numbers)-3, 1):
    current = numbers[i] + numbers[i+1] + numbers[i+2]
    next = numbers[i+1] + numbers[i+2] + numbers[i+3]
    # print(current)
    # print(next)
    
    if next > current: total = total + 1

print(total)

input.close()
