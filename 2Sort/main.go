package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := read_input()
	for _, t := range input {
		fmt.Println(solve(t))
	}
}

type test struct {
	n    int
	k    int
	vals []int
}

func solve(t test) int {
	increasing := is_increasing(t.vals)
	ans := 0
	l := len(increasing)
	i := 0
    for ; i < l; {
		j := i + 1
		for ; j < l && increasing[j]; j++ {
		}
		len := j - i
		if len >= t.k+1 {
			count := j - i - t.k
			ans += count
		}
		i = j
	}
	return ans
}

func is_increasing(vals []int) []bool {
	l := len(vals)
	ans := make([]bool, l, l)
	ans[0] = true
	for i := 1; i < l; i++ {
		ans[i] = vals[i]*2 > vals[i-1]
	}
	return ans
}

func read_input() []test {
	reader := bufio.NewReader(os.Stdin)
	t := read_int(reader)
	tests := make([]test, t, t)
	for i := 0; i < t; i += 1 {
		data := read_int_array(reader)
		n, k := data[0], data[1]
		vals := read_int_array(reader)
		tests[i] = test{n, k, vals}
	}
	return tests
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
