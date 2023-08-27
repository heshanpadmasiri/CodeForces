package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type testCase struct {
	scores [][5]int
}

func main() {
	input := read_input()
	for _, test := range input {
		fmt.Println(solve(test))
	}
}

func solve(test testCase) int {
	scores := test.scores
	n := len(scores)
	w := 0
	for i := 1; i < n; i++ {
		if !is_superior(&scores[w], &scores[i]) {
			w = i
		}
	}
	for i := 0; i < w; i++ {
		if !is_superior(&scores[w], &scores[i]) {
			return -1
		}
	}
	return w + 1
}

func is_superior(lhs *[5]int, rhs *[5]int) bool {
	count := 0
	for i := 0; i < 5; i++ {
		if lhs[i] < rhs[i] {
			count++
		}
	}
	return count >= 3
}

func read_input() []testCase {
	scanner := bufio.NewScanner(os.Stdin)
	t := read_int(scanner)
	cases := make([]testCase, t, t)
	for i := 0; i < t; i++ {
		cases[i] = read_testcase(scanner)
	}
	err := scanner.Err()
	if err != nil {
		log.Fatal(err)
	}
	return cases
}

func read_testcase(scanner *bufio.Scanner) testCase {
	n := read_int(scanner)
	scores := make([][5]int, n, n)
	for i := 0; i < n; i++ {
		var score [5]int
		ints := read_ints(scanner)
		for j := 0; j < 5; j++ {
			score[j] = ints[j]
		}
		scores[i] = score
	}
	return testCase{scores}
}

func read_int(scanner *bufio.Scanner) int {
	line := read_line(scanner)
	val, err := strconv.Atoi(strings.TrimSpace(line))
	if err != nil {
		log.Fatal(err)
	}
	return val
}

func read_ints(scanner *bufio.Scanner) []int {
	line := read_line(scanner)
	vals := strings.Split(line, " ")
	result := make([]int, 0)
	for _, val := range vals {
		int_val, err := strconv.Atoi(val)
		if err != nil {
			log.Fatal(err)
		}
		result = append(result, int_val)
	}
	return result
}

func read_line(scanner *bufio.Scanner) string {
	scanner.Scan()
	return scanner.Text()
}
