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

type problem struct {
	k       int
	weights []int
}

func main() {
	input := read_input()
	for _, problem := range input {
		fmt.Println(solve(problem))
	}
}

func solve(p problem) int {
	n := len(p.weights)
	remainders := make([]int, n)
	cost := 0
	for i, weight := range p.weights {
		cost += weight / p.k
		remainders[i] = weight % p.k
	}
	sort.Ints(remainders)
	i := 0
	j := n - 1
	for ; i < j; j-- {
		for ; i < j; i++ {
			if remainders[i]+remainders[j] >= p.k {
				cost += 1
				i += 1
				break
			}
		}
	}
	return cost
}

func read_input() []problem {
	scanner := bufio.NewScanner(os.Stdin)
	const maxBufferSize = 102400000 // 100mb
	buf := make([]byte, maxBufferSize)
	scanner.Buffer(buf, maxBufferSize)
	scanner.Scan()
	t, err := strconv.Atoi(scanner.Text())
	if err != nil {
		log.Fatal(err)
	}
	problems := make([]problem, t)
	for i := 0; i < t; i++ {
		data := read_ints(scanner)
		k := data[1]
		weights := read_ints(scanner)
		problems[i] = problem{k, weights}
	}
	return problems
}

func read_ints(scanner *bufio.Scanner) []int {
	scanner.Scan()
	line := scanner.Text()
	sVals := strings.Split(line, " ")
	n := len(sVals)
	vals := make([]int, n)
	log.Println(line)
	for i := 0; i < n; i++ {
		val, err := strconv.Atoi(sVals[i])
		if err != nil {
			log.Fatal(err)
		}
		vals[i] = val
	}
	return vals
}
