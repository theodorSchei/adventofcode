file = open('2021/day2/input.txt', 'r')
lines = file.readlines()

input = []

commands = []
units = []

for line in lines:
    commands.append(line.split(' ')[0])
    units.append(int(line.split(' ')[1].strip()))

depth = 0
horizontal_position = 0

for command, unit in zip(commands, units):

    if command == 'forward':
        horizontal_position += unit
    elif command == 'up':
        depth -= unit
    elif command == 'down':
        depth += unit

print(horizontal_position * depth)

file.close()
