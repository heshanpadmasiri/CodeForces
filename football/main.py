def solve() -> str:
    input_str = input().strip()
    val = int(input_str, 2)
    mask = (1 << 7) - 1
    input_len = len(input_str)
    while input_len >= 7:
        section = val & mask
        if section == 0 or section == mask:
            return "YES"
        input_len -= 1;
        val = val >> 1
    return "NO"


if __name__ == "__main__":
    print(solve())

