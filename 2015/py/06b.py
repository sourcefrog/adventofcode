# %%

import collections


def points(p1, p2):
    x1, x2 = sorted([p1[0], p2[0]])
    y1, y2 = sorted([p1[1], p2[1]])
    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            yield((x, y))


def toggle(lit, p):
    lit[p] += 2


def turn_on(lit, p):
    lit[p] += 1


def turn_off(lit, p):
    lit[p] = max(0, lit[p]-1)


lit = collections.defaultdict(lambda: 0)


for l in open('../input/06.txt').read().splitlines():
    if l.startswith('toggle'):
        fn = toggle
        l = l[7:]
    elif l.startswith('turn on'):
        fn = turn_on
        l = l[8:]
    elif l.startswith('turn off'):
        fn = turn_off
        l = l[9:]
    else:
        raise ValueError(l)
    words = l.split()
    p1 = list(map(int, words[0].split(',')))
    assert words[1] == 'through'
    p2 = list(map(int, words[2].split(',')))
    for p in points(p1, p2):
        fn(lit, p)

print(sum(lit.values()))

# %%
