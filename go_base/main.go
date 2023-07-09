package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	input := read_input()
	for i, line := range input {
		fmt.Println(i, line)
	}
}

func read_input() []string {
	scanner := bufio.NewScanner(os.Stdin)
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
