data = [[*map(int, l.split())] for l in open('/home/rochen/Documents/advent_of_code_2024/day_2/input/in.tx')]

def good(d, s=0):
    for i in range(len(d)-1):
        if not 1 <= d[i]-d[i+1] <= 3:
            return s and any(good(d[j-1:j] + d[j+1:]) for j in (i,i+1))
    return True
c = 0
f = open("/home/rochen/Documents/advent_of_code_2024/day_2/input/debug1.txt", "w")
for d in data:
    if good(d, 1) or good(d[::-1], 1):
        c += 1
        line = " ".join(map(str,d))
        print(line)
        f.writelines(f"{line}\n")

