import re

inp = open("input.txt", "r")
currentMaxes = [0, 0, 0]
endIndex = 0
currentLines = []

for line in inp:
    currentLines.append(line)
    if re.match(r"^\n", line):
        caloriesList = currentLines[0:endIndex - 1]
        summation = sum(map(lambda x: int(x.strip()), caloriesList))
        if currentMaxes[0] < summation:
            currentMaxes[0] = summation
        currentMaxes.sort()
        endIndex = 0
        currentLines = []
    endIndex += 1

print(sum(currentMaxes))
