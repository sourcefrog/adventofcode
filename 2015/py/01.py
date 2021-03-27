#! /usr/bin/python3

input = open("input/01.txt").read()
floor = input.count("(") - input.count(")")

print(floor)

floor = 0
for i, a in enumerate(input):
    floor = floor + (1 if a == '(' else - 1)
    if floor == -1:
        print(i + 1)
        break
