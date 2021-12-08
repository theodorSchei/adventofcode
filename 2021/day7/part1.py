file = open('2021/day7/input.txt', 'r')
lines = file.readlines()
file.close()

positionsString=lines[0].strip().split(',')
positions = []

for timer in positionsString:
	positions.append(int(timer))


def findMedian(list: list):
	list.sort()
	median = list[int(len(list)/2)]
	return median


def moveCrabsToMedian():
	median = findMedian(positions)
	fuel = 0
	for crab in positions:
		fuel += abs(median-crab)
	return fuel

print(moveCrabsToMedian())