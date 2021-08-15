# %%
#

example = """123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"""


def load(l):
    # connections to each wire, either an integer or a formula
    cn = dict()
    for l in l.splitlines():
        expr, output = l.split(' -> ')
        cn[output] = expr
    return cn


def solve_memo(cons, var):
    if var in cons and isinstance(cons[var], int):
        return cons[var]
    v = solve1(cons, var)
    cons[var] = v
    print(f'{var} = {v}')
    return v


def solve1(cons, var):
    try:
        return int(var)
    except ValueError:
        pass
    expr = cons[var]
    try:
        return int(expr)
    except ValueError:
        pass
    w = expr.split()
    if len(w) == 1 and w[0] in cons:
        return solve_memo(cons, w[0])
    # print(w)
    if w[0] == 'NOT':
        assert len(w) == 2
        return 65535 ^ solve_memo(cons, w[1])
    op = w[1]
    assert len(w) == 3
    f = {'AND':
         lambda x, y: x & y,
         'OR': lambda x, y: x | y,
         'LSHIFT': lambda x, y: (x << y) & 0xffff,
         'RSHIFT': lambda x, y: (x >> y)
         }
    return f[op](solve_memo(cons, w[0]), solve_memo(cons, w[2]))

# %%


cons = load(example)
assert solve1(cons, 'd') == 72
assert solve1(cons, 'e') == 507
assert solve1(cons, 'f') == 492
assert solve1(cons, 'g') == 114
assert solve1(cons, 'h') == 65412
assert solve1(cons, 'i') == 65079
assert solve1(cons, 'x') == 123
assert solve1(cons, 'y') == 456

# %%

cons = load(open('../input/07.txt').read())
print(solve_memo(cons, 'a'))


# %%

# Part 2
cons = load(open('../input/07.txt').read())
cons['b'] = 16076

print(solve_memo(cons, 'a'))

# %%
