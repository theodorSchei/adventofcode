file = open('2021/day4/input.txt', 'r')
lines = file.readlines()

draws = lines[0].strip().split(',')
boardsRaw = []

for line in lines[1:]:
	boardsRaw.append(line.strip())
file.close()

boards = []

# Make the rows
rows = []
for line in boardsRaw:
	if line != '':
		row = line.split()
		rows.append(row)

# Make the columns
columns = []
for i in range(0,len(rows),5):
	column = []
	for j in range(0,5):
		column.append(rows[i+j])
	boards.append(column)

# Helper for printing
def formatBoards(boards):
	txt = ''
	for board in boards:
		for column in board:
			for row in column:
				txt += f'{row} '
			txt += f'\n'
		txt += f'\n'
	return txt

print(f'Draws: {draws}\nBoards:\n{formatBoards(boards)}')
print(f'Board 0, Column 0, Row 0: {boards[0][0][0]}')

# Make array for showing if number is drawn
isDrawn = []

for i in range(len(boards)):
	board = []
	for j in range(len(boards[0])):
		column = []
		for k in range(len(boards[0][0])):
			column.append(False)
		board.append(column)
	isDrawn.append(board)

print(f'\nDrawn:\n{formatBoards(isDrawn)}')

def drawNumbers():
	for draw in draws:
		markNumberAsDrawn(draw)
		# print(f'After draw: {draw}\nDrawn:\n{formatBoards(isDrawn)}')
		winningBoard = checkIfAnyBoardsWon()
		print(f'Number of winners: {len(boardIndexesThatWon)} / {len(boards)}')
		if len(boardIndexesThatWon) == len(boards):
			print(f'Last board to win is {boardIndexesThatWon[-1]} with final score {calculateWinnerScore(boardIndexesThatWon[-1], draw)}')
			break

def markNumberAsDrawn(number):
	for i in range(len(boards)):
		for j in range(len(boards[0])):
			for k in range(len(boards[0][0])):
				if boards[i][j][k] == number:
					isDrawn[i][j][k] = True

boardIndexesThatWon = []

def checkIfBoardWon(boardIndex, board):
	# Check horizontal
	for i, column in enumerate(board):
		numberOfDraws = 0
		for j, row in enumerate(column):
			if isDrawn[boardIndex][i][j]:
				numberOfDraws+=1
			if numberOfDraws == len(board[0]):
				if boardIndex not in boardIndexesThatWon:
					boardIndexesThatWon.append(boardIndex)

	# Check vertical
	for i, column in enumerate(board):
		numberOfDraws = 0
		for j, row in enumerate(column):
			if isDrawn[boardIndex][j][i]:
				numberOfDraws+=1
			if numberOfDraws == len(board[0]):
				if boardIndex not in boardIndexesThatWon:
					boardIndexesThatWon.append(boardIndex)



def checkIfAnyBoardsWon():
	for boardIndex, board in enumerate(boards):
		if boardIndex not in boardIndexesThatWon:
			if checkIfBoardWon(boardIndex, board):
				return boardIndex

def calculateWinnerScore(boardIndex, lastDraw):
	sum = 0

	for j in range(len(boards[0])):
		for k in range(len(boards[0][0])):
			if isDrawn[boardIndex][j][k] == False:
				sum += int(boards[boardIndex][j][k])

	return sum * int(lastDraw)

drawNumbers()

# more than 7770