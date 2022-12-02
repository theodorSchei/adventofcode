file = open('2021/day9/input.txt', 'r')
lines = file.readlines()
file.close()

heightMap = []

for line in lines:
	line = line.strip()
	inputLine = []
	for number in line:
		inputLine.append(number)

	heightMap.append(inputLine)

#print(heightMap)

def findLowPoints(heightMap):
	lowPoints = []

	for y, line in enumerate(heightMap):
		for x, number in enumerate(line):
			neighbours = getNeighbours(heightMap, y, x)
			isLowest = True
			for neighbour in neighbours:
				if number >= neighbour:
					isLowest = False
			if isLowest:
				lowPoints.append(number)

	return lowPoints

def getNeighbours(heightMap, y, x):
	neighbours = []

	if y-1 >= 0:
		print('Has north!')
		north = heightMap[y-1][x]
		neighbours.append(north)

	if y + 1 < len(heightMap): 
		print('Has south!')
		south = heightMap[y+1][x]
		neighbours.append(south)

	if x - 1 >= 0:
		print('Has west!')
		west = heightMap[y][x-1]
		neighbours.append(west)

	if x + 1 < len(heightMap[y]):
		print('Has east!')
		east = heightMap[y][x+1]
		neighbours.append(east)

	print(f'Point Y:{y}X:{x} V:{heightMap[y][x]} has neighbours {neighbours}')
	return neighbours

def sumOfLowPoints(lowpoints):
	sum = 0
	for point in lowpoints:
		sum += int(point) + 1

	return sum

print(sumOfLowPoints(findLowPoints(heightMap)))