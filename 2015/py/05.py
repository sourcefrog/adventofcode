# %%
inp = open("../input/05.txt").read().splitlines()


def isnice(s):
    if sum(int(c in 'aeiou') for c in s) < 3:
        return False
    seentwo = False
    for a, b in zip(s, s[1:]):
        if a == b:
            seentwo = True
        if a + b in ['ab', 'cd', 'pq', 'xy']:
            return False
    return seentwo


assert isnice('ugknbfddgicrmopn') == True
assert isnice('aaa')

# jchzalrnumimnmhp is naughty because it has no double letter.
assert not isnice('jchzalrnumimnmhp')
# haegwjzuvuyypxyu is naughty because it contains the string xy.
assert not isnice('haegwjzuvuyypxyu')
# dvszwmarrgswjxmb is naughty because it contains only one vowel.
assert not isnice('dvszwmarrgswjxmb')

# Part 1
print(sum(int(isnice(s)) for s in inp))

# %%
# Part 2


def is2nice(s):
    for i in range(len(s) - 3):
        a = s[i:i + 2]
        assert len(a) == 2
        if s.find(a, i + 2) > -1:
            break
    else:
        return False
    for i in range(len(s) - 2):
        if s[i] == s[i + 2]:
            break
    else:
        return False
    return True


# qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
assert is2nice('qjhvhtzxzqqjkmpb')
# xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
assert is2nice('xxyxx')
# uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
assert not is2nice('uurcxstgmygtbstg')
# ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
assert not is2nice('ieodomkazucvgmuy')

print(sum(int(is2nice(s)) for s in inp))

# %%
