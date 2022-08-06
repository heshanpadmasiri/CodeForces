def main():
    t = int(input())
    for _ in range(t):
        if solve():
            print("YES")
        else:
            print("NO")

TWO_PALINDROMES = set(["11", "00"])

def solve() -> bool:
    n = int(input())
    seq = input()
    if n > 2 :
        return False
    return seq not in TWO_PALINDROMES

if __name__ == "__main__":
    main()
