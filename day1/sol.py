import re

inp = open("input.txt", "r")

text = inp.readlines()

res = []

startIndex = 0
endIndex = 0

for line in text:
    if re.match(r"^\n", line):
        caloriesList = text[startIndex:endIndex]
        res.append(sum(map(lambda x: int(x.strip()), caloriesList)))
        startIndex = endIndex + 1
    endIndex += 1
    
print(max(res))