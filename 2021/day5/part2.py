file = open('2021/day5/input.txt', 'r')
linesRaw = file.readlines()
file.close()

#Break lines up in tuples
lines = []
for line in linesRaw:
	line = line.strip()

	fromXY, toXY = line.split(' -> ')[0], line.split(' -> ')[1]

	fromTuple = (int(fromXY.split(',')[0]), int(fromXY.split(',')[1]))
	toTuple = (int(toXY.split(',')[0]), int(toXY.split(',')[1]))
	
	together = (fromTuple, toTuple)
	lines.append(together)
# Line[0 = from, 1 = to][0 = X, 1 = Y] 

# Find diagram size
diagramWidth = 0
diagramHeight = 0

for line in lines:
	if line[0][0] >= diagramWidth:
		diagramWidth = line[0][0] + 1
	if line[1][0] >= diagramWidth:
		diagramWidth = line[1][0] + 1
	if line[0][1] >= diagramHeight:
		diagramHeight = line[0][1] + 1
	if line[1][1] >= diagramHeight:
		diagramHeight = line[1][1] + 1

print(f'Diagram supposed width: {diagramWidth}, Diagram supposed height: {diagramHeight}')

# Make diagram
diagram = []
for i in range(diagramHeight):
   diagram.append([0] * diagramWidth)
# Diagram[x][y]

print(f'Diagram actual width: {len(diagram[0])}, Diagram acual height: {len(diagram)}')

def printDiagram(diagram):
	out = ''
	for y in range(len(diagram)):
		for x in range (len(diagram[0])):
			if diagram[x][y] == 0:
				out += '.'
			else:
				out += str(diagram[x][y])
		out += '\n'
	print(out)

# printDiagram(diagram)

def isStraight(line):
	if line[0][0] == line[1][0] or line[0][1] == line [1][1]: return True
	else: return False

def plotStraightLine(line):
	print(f'Plotting straight line {line}:')
	for x in range(min(line[0][0], line[1][0]), max(line[0][0], line[1][0]) + 1): # From x1 to x2
		for y in range(min(line[0][1], line[1][1]), max(line[0][1], line[1][1]) + 1):
			# print(x, y)
			diagram[x][y] += 1
	# printDiagram(diagram)

def plotHorizontalLine(line):
	print(f'Plotting horizontal line {line}:')
	fromX = line[0][0]
	toX = line[1][0]
	minX = min(fromX, toX)
	maxX = max(fromX, toX)

	
	fromY = line[0][1]
	toY = line[1][1]
	minY = min(fromY, toY)
	maxY = max(fromY, toY)


	horizontalLength = abs(fromX-toX) + 1

	print(f'Horizontal Length: {horizontalLength}')

	for i in range(horizontalLength):
			if fromX < toX and fromY < toY : #right downwards \>
				print('right downwards \u2198')
				diagram[fromX + i][fromY + i] += 1
			elif fromX > toX and fromY < toY: # left downwards </
				print('left downwards \u2199')
				diagram[fromX - i][fromY + i] += 1
			elif fromX < toX and fromY > toY : #right upwards />
				print('right upwards \u2197')
				diagram[fromX + i][fromY - i] += 1
			elif fromX > toX and fromY > toY: # left upwards <\
				print('left upwards \u2196')
				diagram[fromX - i][fromY - i] += 1

	# printDiagram(diagram)

def plotLines():
	for line in lines:
		if isStraight(line):
			plotStraightLine(line)
			# print('skipping straight')
		else:
			plotHorizontalLine(line)


plotLines()

def findNumberOfOverlaps (diagram, minimumOverlap: int):
	numberOfOverlaps = 0
	for y in range(len(diagram)):
		for x in range (len(diagram[0])):
			if diagram[x][y] >= minimumOverlap:
				numberOfOverlaps += 1
	return numberOfOverlaps

print(f'Number of overlaps: {findNumberOfOverlaps(diagram, 2)}')