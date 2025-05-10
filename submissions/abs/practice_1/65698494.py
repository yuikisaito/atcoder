import sys

def main():
    data = sys.stdin.read().split()
    a = int(data[0])
    b = int(data[1])
    c = int(data[2])
    s = data[3]
    print(a + b + c, s)

if __name__ == "__main__":
    main()
