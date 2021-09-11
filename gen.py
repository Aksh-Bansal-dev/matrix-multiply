import random
import sys

n = int(sys.argv[1])

for t in range(2):
    print(n)
    for i in range(n):
        for j in range(n):
            print(random.randint(1,1000), end=" ")
        print()