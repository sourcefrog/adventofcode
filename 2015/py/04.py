# %%
# Part 1
import hashlib
input = b'ckczppom'
for i in range(1, 100000000):
    body = input + b'%d' % i
    h = hashlib.md5(body).hexdigest()
    if h[:5] == '00000':
        print(i, body, h)
        break

# %%
# Part 2
for i in range(1, 100000000):
    body = input + b'%d' % i
    h = hashlib.md5(body).hexdigest()
    if h[:6] == '000000':
        print(i, body, h)
        break

# %%
