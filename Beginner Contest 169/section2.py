import sys

n = int(input())
a = list(map(int, input().split()))

if 0 in a:
    print(0)
else:
    cur = 1
    for x in a:
        cur *= x
        if cur > 10 ** 18:
            print(-1)
            sys.exit()
    print(cur)