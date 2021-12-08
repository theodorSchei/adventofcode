file = open('2021/day8/input.txt', 'r')
lines = file.readlines()
file.close()

inputs = []
outputs = []

for line in lines:
	line = line.strip()
	lineInput = line.split(' | ')[0].split(' ')
	lineOutput = line.split(' | ')[1].split(' ')

	inputs.append(lineInput)
	outputs.append(lineOutput)

	

def convertLineToNumberOfOccurences(inputs, outputs):
	
	ourOccurences = {
		'f': 0,
		'a': 0,
		'c': 0,
		'g': 0,
		'd': 0,
		'b': 0,
		'e': 0
	}

	# map all inputs to its number of occurences in line
	# Find total occurences:
	for input in inputs:
		for character in input:
			ourOccurences[character] += 1
	
	# remake input list to sum of number of occurences
	inputsAsCountSum = []
	for input in inputs:
		segment = 0
		for character in input:
			segment += ourOccurences[character]
		inputsAsCountSum.append(segment)

	# remake output list to sum of number of occurences
	outputsAsCountSum = []
	for input in outputs:
		segment = 0
		for character in input:
			segment += ourOccurences[character]
		outputsAsCountSum.append(segment)
	
	return inputsAsCountSum, outputsAsCountSum



def decodeLine(input, output):
	Dict = {
		42: 0,
		17: 1,
		34: 2,
		39: 3,
		30: 4,
		37: 5,
		41: 6,
		25: 7,
		49: 8,
		45: 9
	}
	
	number = ''
	print(output)
	
	input, output = convertLineToNumberOfOccurences(input, output)
	
	print(output)
	for segment in output:
		number += str(Dict[segment])
	return number


def decodeLines():
	total = 0
	for input, output in zip(inputs, outputs):
		decoded = decodeLine(input, output)
		total += int(decoded)
	return total

print(decodeLines())