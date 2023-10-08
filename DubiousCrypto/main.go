package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type test struct {
	l, r, m int;
}

type answer struct {
	a, b, c int;
}

func main() {
	tests, err := read_input()
	if err != nil {
		log.Fatal(err)
	}
	for i, test := range(tests) {
		if i > 0 {
			fmt.Print("\n")
		}
		ans := solve(test)
		if !validate_ans(test, ans) {
			log.Fatal("invalid answer")
		}
		fmt.Printf("%d %d %d", ans.a, ans.b, ans.c)
	}
}

func validate_ans(test test, ans answer) bool {
	a := ans.a
	b := ans.b
	c := ans.c
	l := test.l
	r := test.r
	if !(int_in_range(a, l, r) && int_in_range(b, l, r) && int_in_range(c, l, r)) {
		return false
	}
	m := test.m
	t1 := m-b+c;
	if t1 % a != 0 {
		return false
	}
	n := t1 / a
	return n > 0
}

func int_in_range(val, l, r int) bool {
	return l <= val && r >= val
}

func solve(test test) answer {
	l := test.l
	r := test.r
	m := test.m
	for a := l; a < r + 1; a++ {
		n_max := (m + r - l) / a
		if n_max == 0 {
			continue
		}
		na := n_max * a
		if m - (r - l) > na || m + (r - l) < na {
			continue
		}
		b, c := findBC(l, r, m, a, n_max)
		return answer{a, b, c}
	}
	log.Fatal("failed to file an answer")
	return answer{ a:0, b: 0, c: 0}
}

func findBC(l, r, m, a, n int) (int, int) {
	k := m - a*n // b - c
	for c := l; c < r+1; c++ {
		b := k + c;
		if int_in_range(b, l, r) {
			return b, c
		}
	}
	log.Fatal("failed to find b c")
	return 0, 0
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func read_input() ([]test, error) {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	line := scanner.Text()
	t, err := strconv.Atoi(line)
	if err != nil {
		return nil, err
	}
	tests := make([]test, t, t)
	for i := 0; scanner.Scan(); i++ {
		line = scanner.Text()
		test, err := parse_test(line)
		if err != nil {
			return nil, err
		}
		tests[i] = test
	}
	return tests, nil
}

func parse_test(line string) (test, error) {
	tokens := strings.Split(line, " ")
	l, err := strconv.Atoi(tokens[0])
	if err != nil {
		return test{l:0, r:0, m:0}, err
	}
	r, err := strconv.Atoi(tokens[1])
	if err != nil {
		return test{l:0, r:0, m:0}, err
	}
	m, err := strconv.Atoi(tokens[2])
	if err != nil {
		return test{l:0, r:0, m:0}, err
	}
	return test {l, r, m}, nil
}
