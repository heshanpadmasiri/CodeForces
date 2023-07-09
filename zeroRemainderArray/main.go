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
	n    int
	k    int
	vals []int
}

func main() {
	input := read_input()
	for _, c := range input {
		ops := solve(c)
		fmt.Println(ops)
	}
}

func solve(c test) int {
	cnts := make(map[int]int)
	k := c.k
	for _, val := range c.vals {
		remainder := val % k
		if remainder == 0 {
			continue
		}
		index := k - remainder
		cur, _ := cnts[index]
		cnts[index] = cur + 1
	}
	if len(cnts) == 0 {
		return 0
	}
	max_key, max_val := -1, -1
	for key, val := range cnts {
		if val > max_val {
			max_val = val
			max_key = key
		} else if val == max_val && key > max_key {
			max_key = key
		}
	}
	return (k * (max_val - 1)) + max_key + 1
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

type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	// Push and Pop use pointer receivers because they modify the slice's length,
	// not just its contents.
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
