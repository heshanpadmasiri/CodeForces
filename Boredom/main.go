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
	values := parse_input()
	fmt.Println(solve(values))
}

type element struct {
	value int
	count int
}

func solve(values []element) int {
	n := len(values)
	dp := make([]int, n)
	for i := range dp {
		dp[i] = -1
	}
	for i := range dp {
		if i > 0 && dp[i-1] == -1 {
			panic("unexpect")
		}
		dp[i] = best_score_upto_i(&values, &dp, i)
	}
	return dp[n-1]
}

func best_score_upto_i(values *[]element, dp *[]int, i int) int {
	if i < 0 {
		return 0
	}
	memo := (*dp)[i]
	if memo != -1 {
		return memo
	}
	value := (*values)[i]
	current_score := value.value * value.count
	if i == 0 {
		return current_score
	}
	last := (*values)[i-1]
	if last.value != value.value-1 {
		return best_score_upto_i(values, dp, i-1) + current_score
	} else {
		return max(best_score_upto_i(values, dp, i-2)+current_score, best_score_upto_i(values, dp, i-1))
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func read_input() []string {
	scanner := bufio.NewScanner(os.Stdin)
	buf := make([]byte, 0, 64*1024)
	scanner.Buffer(buf, 1024*1024)
	lines := make([]string, 0)
	for {
		scanner.Scan()
		line := scanner.Text()
		if len(line) == 0 {
			break
		}
		lines = append(lines, line)
	}
	err := scanner.Err()
	if err != nil {
		log.Fatal(err)
	}
	return lines
}

func parse_input() []element {
	lines := read_input()
	n, err := strconv.Atoi(lines[0])
	if err != nil {
		log.Fatal(err)
	}
	values := make([]int, n)
	for i, v := range strings.Split(lines[1], " ") {
		val, err := strconv.Atoi(v)
		if err != nil {
			log.Fatal(err)
		}
		values[i] = val
	}
	sort.Ints(values)
	result := make([]element, 0)
	for i := 0; i < n; {
		value := values[i]
		count := 1
		j := i + 1
		for ; j < n && values[j] == value; j++ {
			count += 1
		}
		result = append(result, element{value, count})
		i = j
	}
	return result
}
