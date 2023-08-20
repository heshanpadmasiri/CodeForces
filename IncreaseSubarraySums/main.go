package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type problem struct {
	array []int
	n, x int
}

func main() {
	problems, err := read_input()
	if err != nil {
		log.Fatal(err)
	}
	for _, problem := range problems {
		ans := solve(problem)
		for i, each := range ans {
			if i > 0 {
				fmt.Print(" ")
			}
			fmt.Print(each)
		}
		fmt.Println("")
	}
}

func solve(problem problem) []int {
	max_subarray_sum := make([]int, problem.n+1, problem.n+1)
	for i := 0; i <= problem.n; i++ {
		max_subarray_sum[i] = max_subarray(&problem.array, i)
	}
	// log.Println(max_subarray_sum, problem)
	ans := make([]int, problem.n+1, problem.n+1)
	for k := 0; k < problem.n+1; k++ {
		current_max := math.MinInt
		for j:=0; j < problem.n+1; j++ {
			var fk int;
			if j < k {
				fk = j*problem.x + max_subarray_sum[j]
			} else {
				fk = k*problem.x + max_subarray_sum[j]
			}
			if fk > current_max {
				current_max = fk
			}
		}
		// log.Print(current_max, k)
		ans[k] = current_max
	}
	return ans
}

func max_subarray(array_ptr *[]int, length int) int {
	arr := *array_ptr
	if length == 0 {
		return 0
	}
	maxSum := math.MinInt32
	currentSum := 0

	// Calculate the sum of the first "length" elements
	for i := 0; i < length; i++ {
		currentSum += arr[i]
	}

	maxSum = currentSum

	// Slide the window and update the current sum
	for i := length; i < len(arr); i++ {
		currentSum = currentSum - arr[i-length] + arr[i]
		maxSum = int(math.Max(float64(maxSum), float64(currentSum)))
	}

	return maxSum}

func read_input() ([]problem, error) {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	line := scanner.Text()
	t, err := strconv.Atoi(line)
	if err != nil {
		return nil, err
	}
	problems := make([]problem, t, t)
	for i := 0; i < t; i++ {
		parts := tokenize(scanner)
		n, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, err
		}
		x, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, err
		}
		array := make([]int, n, n)
		parts = tokenize(scanner);
		for j := 0; j < n; j++ {
			val, err := strconv.Atoi(parts[j])
			if err != nil {
				return nil, err
			}
			array[j] = val
		}
		problems[i] = problem { array, n, x }
	}
	return problems, nil
}

func tokenize(scanner *bufio.Scanner) []string {
	scanner.Scan()
	line := scanner.Text()
	return strings.Split(line, " ")
}
