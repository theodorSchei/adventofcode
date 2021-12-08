file = open('2021/day7/input.txt', 'r')
lines = file.readlines()
file.close()

positionsString=lines[0].strip().split(',')
positions = []

for timer in positionsString:
	positions.append(int(timer))
	
def findAverage(list: list):
	total = 0
	for item in list: total += item
	avgFloat = total / len(list )
	avg = int(avgFloat)

	print(f'AVG: {avgFloat} {avg}')
	return avg

def moveCrabsToAverage():
	average = findAverage(positions)
	fuel = 0
	for crab in positions:
		fuel += abs(average - crab)
		for i in range(abs(average - crab)):
			fuel += i
	return fuel

print(moveCrabsToAverage())

# round() = 99788438 is too high
# int() = 99788435