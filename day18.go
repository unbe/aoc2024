package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type XY struct {
	x, y int
}

func XYFromString(line string) XY {
	parts := strings.Split(strings.TrimSuffix(line, "\n"), ",")
	x, err := strconv.Atoi(parts[0])
	if err != nil {
		panic(err)
	}
	y, err := strconv.Atoi(parts[1])
	if err != nil {
		panic(err)
	}
	return XY{x, y}
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	bad := map[XY]int{}
	badr := map[int]XY{}
	scanner := bufio.NewScanner(file)
	n := 0
	for scanner.Scan() {
		line := scanner.Text()
		badr[n] = XYFromString(line)
		bad[badr[n]] = n
		n += 1
	}
	fmt.Println("part1:", solve(bad, 1024))

	a := 1024
	b := n
	for b-a > 1 {
		k := (b + a) / 2
		if solve(bad, k) > 0 {
			a = k
		} else {
			b = k - 1
		}
	}
	fmt.Printf("part2: %v,%v\n", badr[b].x, badr[b].y)
}

func solve(bad map[XY]int, limit int) int {
	dist := map[XY]int{}
	front := []XY{{0, 0}}
	dist[front[0]] = 0
	dirs := []XY{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	sz := XY{70, 70}
	for len(front) > 0 {
		p := front[0]
		front = front[1:]
		for _, d := range dirs {
			np := XY{p.x + d.x, p.y + d.y}
			if np.x < 0 || np.x > sz.x || np.y < 0 || np.y > sz.y {
				continue
			}
			ns, found := bad[np]
			if found && ns <= limit {
				continue
			}
			_, seen := dist[np]
			if seen {
				continue
			}
			dist[np] = dist[p] + 1
			front = append(front, np)
		}
	}
	dd := dist[sz]
	return dd
}
