from typing import List


file = open('2021/day3/input.txt', 'r')
lines = file.readlines()

binaries = []

for line in lines:
	binaries.append(line.strip())
file.close()

binariesCopy = binaries.copy()


def findMostPopularBinary(binaries: List, index: int):

	numberOfOnes = 0

	for number in binaries:
		if number[index] == '1':
			numberOfOnes += 1

	print(f'Number of ones: {numberOfOnes}, Total/2: {len(binaries)/2}')
	if numberOfOnes >= len(binaries)/2:
		return '1'
	else:
		return '0'


def findOxygenGeneratorRating(binaries: List):
	
	print(f'\n\n--------Oxygen--------')
	
	oxygenGeneratorRating = ''

	for i in range(len(binaries[0])):

		print(f'Before deletion: {binaries}')

		if len(binaries) == 1:
			oxygenGeneratorRating += binaries[0][i]
		else:
			binaryToKeep = findMostPopularBinary(binaries, i)
			oxygenGeneratorRating += binaryToKeep
		
		print(f'Binary to keep: {binaryToKeep} at index {i}')
		print(f'Added {binaryToKeep} to Oxygen')

		indexesToDelete = []

		for j, number in enumerate(binaries):
			if number[i] != binaryToKeep:
				indexesToDelete.append(j)
		if len(binaries) > 1:
			for index in sorted(indexesToDelete, reverse=True):
				del binaries[index]

		print(f'After deletion: {binaries}\n')

	return oxygenGeneratorRating


def findCO2ScrubberRating(binaries: List):
	
	print(f'\n\n--------CO2--------')
	
	CO2ScrubberRating = ''

	for i in range(len(binaries[0])):
		print(f'Before deletion: {binaries}')
		if len(binaries) == 1:
			CO2ScrubberRating += binaries[0][i]
		else:
			binaryToKeep = findMostPopularBinary(binaries, i).replace(
				'1', '2').replace('0', '1').replace('2', '0')
			CO2ScrubberRating += binaryToKeep
		
		print(f'Binary to keep: {binaryToKeep} at index {i}')
		print(f'Added {binaryToKeep} to CO2ScrubberRating')

		indexesToDelete = []

		for j, number in enumerate(binaries):
			if number[i] != binaryToKeep:
				indexesToDelete.append(j)
		if len(binaries) > 1:
			for index in sorted(indexesToDelete, reverse=True):
				del binaries[index]
		
		print(f'After deletion: {binaries}\n')

	return CO2ScrubberRating


oxygenGeneratorRating = findOxygenGeneratorRating(binaries)
oxygenGeneratorRatingDecimal = int(oxygenGeneratorRating, 2)
CO2ScrubberRating = findCO2ScrubberRating(binariesCopy)
CO2ScrubberRatingDecimal = int(CO2ScrubberRating, 2)

print(f'OGR: {oxygenGeneratorRating} = {oxygenGeneratorRatingDecimal}')
print(f'CO2: {CO2ScrubberRating} = {CO2ScrubberRatingDecimal}')
print(f'Sum: {oxygenGeneratorRatingDecimal * CO2ScrubberRatingDecimal}')