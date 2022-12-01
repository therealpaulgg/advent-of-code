import re

inp = open("input.txt", "r")
currentMax = 0
endIndex = 0
currentLines = []

for line in inp:
    currentLines.append(line)
    if re.match(r"^\n", line):
        caloriesList = currentLines[0:endIndex - 1]
        summation = sum(map(lambda x: int(x.strip()), caloriesList))
        if currentMax < summation:
            currentMax = summation
        endIndex = 0
        currentLines = []
    endIndex += 1

print(currentMax)
