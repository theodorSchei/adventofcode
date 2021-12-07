file = open('2021/day6/input.txt', 'r')
lines = file.readlines()
file.close()

internalTimersString=lines[0].strip().split(',')
internalTimers = []

for timer in internalTimersString:
	internalTimers.append(int(timer))

def passOneDay():
	for i in range(len(internalTimers)):
		if internalTimers[i] == 0:
			internalTimers[i] = 6
			internalTimers.append(8)
		else: internalTimers[i] -= 1	

def passDays(numberofDays: int):
	for i in range(1, numberofDays + 1):
		passOneDay()
		print(f'After {i} days: {len(internalTimers)}')


print(internalTimers)
passDays(256)
print(len(internalTimers))
