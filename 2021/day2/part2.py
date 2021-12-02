file = open('2021/day2/input.txt', 'r')
lines = file.readlines()

input = []

commands = []
units = []

for line in lines:
	commands.append(line.split(' ')[0])
	units.append(int(line.split(' ')[1].strip()))

aim = 0
horizontal_position = 0
depth = 0

for command, unit in zip(commands, units):
	
	match command:
		case 'forward':
			horizontal_position += unit
			depth = depth + aim * unit
		case 'up':
			aim -= unit
		case 'down':
			aim += unit

print(horizontal_position * depth)

file.close()
