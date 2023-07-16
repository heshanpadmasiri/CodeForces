package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := read_input()
	fmt.Println(solve(input))
}

type test struct {
	n    int
	k    int
	vals []int
}

func solve(t test) int {
	diffs := pair_diffs(t.vals)
	ans := t.vals[t.n-1] - t.vals[0]
	for i := 0; i < t.k-1; i++ {
		ans += diffs[i]
	}
	return ans
}

func pair_diffs(vals []int) []int {
	diffs := make([]int, 0)
	for i := 1; i < len(vals); i++ {
		diffs = append(diffs, vals[i-1]-vals[i])
	}
	sort.Ints(diffs)
	return diffs
}

func read_input() test {
	reader := bufio.NewReader(os.Stdin)
	data := read_int_array(reader)
	n, k := data[0], data[1]
	vals := read_int_array(reader)
	return test{n, k, vals}
}

func read_int_array(reader *bufio.Reader) []int {
	line, err := reader.ReadString('\n')
	if err != nil {
		log.Fatal(err)
	}
	line = strings.TrimSpace(line)
	vals := make([]int, 0)
	for _, val_s := range strings.Split(line, " ") {
		val, err := strconv.Atoi(val_s)
		if err != nil {
			log.Fatal(err)
		}
		vals = append(vals, val)
	}
	return vals
}

func read_int(reader *bufio.Reader) int {
	line, err := reader.ReadString('\n')
	if err != nil {
		log.Fatal(err)
	}
	line = strings.TrimSpace(line)
	val, err := strconv.Atoi(line)
	if err != nil {
		log.Fatal(err)
	}
	return val
}
