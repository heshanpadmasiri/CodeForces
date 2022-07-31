def main():
    x = input();
    y = input();
    length = len(x)
    x = int(x, 2)
    y = int(y, 2)
    ans = x ^ y
    ans = format(ans, f'0{length}b')
    print(ans)

if __name__ == "__main__":
    main()
