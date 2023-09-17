package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
    "strings"
)

type graph map[string][]string

func main() {
    g := parse_input()
    valid, sets := solve(g)
    if valid {
        a := sets[0]
        b := sets[1]
        fmt.Println(len(a))
        fmt.Println(strings.Join(a, " "))
        fmt.Println(len(b))
        fmt.Println(strings.Join(b, " "))
    } else {
        fmt.Println("-1")
    }
}

func parse_input() graph {
    input := read_input()
    g := make(graph)
    for i, line := range input {
        if i == 0 {
            // n, k
            continue
        }
        edges := strings.Split(line, " ")
        g[edges[0]] = append(g[edges[0]], edges[1])
        g[edges[1]] = append(g[edges[1]], edges[0])
    }
    return g
}

func solve(g graph) (bool, [2][]string) {
    ans := [2][]string{make([]string, 0), make([]string, 0)}
    set := make(map[string]int)
    stack := make([]string, 0)
    // TODO: add the first edge to stack
    stack = append(stack, "1")
    for len(stack) > 0 {
        node := stack[len(stack)-1]
        stack = stack[:len(stack)-1]
        children := g[node]
        color := -1
        for _, child := range children {
            childColor, prs := set[child]
                if color == -1 {
                    color = (childColor + 1) % 2
                } else if color == childColor {
                    return false, ans
                }
        }
    }
    return false, ans
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
