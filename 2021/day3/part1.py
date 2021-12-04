file = open('2021/day3/input.txt', 'r')
lines = file.readlines()

binaries = []

for line in lines:
	binaries.append(line.strip())

gammaRate = ''

for i in range(len(binaries[0])):
		
	numberOfOnes = 0

	for j, number in enumerate(binaries):
		if number[i] == '1':
			numberOfOnes += 1

	if numberOfOnes > len(binaries)/2:
		gammaRate+='1'
	else:
		gammaRate+='0'
	

gammaRateDecimal = int(gammaRate, 2)
epsilonRate = gammaRate.replace('1', '2').replace('0','1').replace('2','0')
epsilonRateDecimal = int(epsilonRate, 2)

print('Gamma Rate:   ' + gammaRate + ' = ' + str(gammaRateDecimal))
print('Epsilon Rate: ' + str(epsilonRate) + ' = ' + str(epsilonRateDecimal))

print('Sum: ' + str(gammaRateDecimal * epsilonRateDecimal))


file.close()
