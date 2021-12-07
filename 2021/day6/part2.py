file = open('2021/day6/input.txt', 'r')
lines = file.readlines()
file.close()

internalTimersString=lines[0].strip().split(',')
internalTimers = []

for timer in internalTimersString:
	internalTimers.append(int(timer))

dayIndexes = [0] * 9

for i in range(0, 9):
	for timer in internalTimers:
		if i == timer:
			dayIndexes[i] += 1
	
print(dayIndexes)


def passOneDay():	
	newDay = dayIndexes

	numberOfZeros = dayIndexes[0]
	
	for i in range(0, 8):
		newDay[i] = dayIndexes[i+1]

	newDay[8] = numberOfZeros
	newDay[6] += numberOfZeros

def passDays(numberofDays: int):
	for i in range(1, numberofDays + 1):
		passOneDay()
		total = 0
		for number in dayIndexes:
			total += number
		print(f'After {i} days: {total}')

print(internalTimers)
passDays(256)
