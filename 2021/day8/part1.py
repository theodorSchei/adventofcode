file = open('2021/day8/input.txt', 'r')
lines = file.readlines()
file.close()

input = []
output = []

for line in lines:
	line = line.strip()
	lineInput = line.split(' | ')[0]
	lineOutput = line.split(' | ')[1]

	input.append(lineInput.split(' '))
	output.append(lineOutput.split(' '))


def findNumberOf1478():
	acceptedLengths = [2, 3, 4, 7]
	total = 0
	for line in output:
		for segment in line:
			if len(segment) in acceptedLengths:
				total += 1
	return total

print(findNumberOf1478())