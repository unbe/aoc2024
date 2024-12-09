package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type XY struct {
	x, y int
}

func (xy *XY) mv(other XY, v int) XY {
	return XY{xy.x + other.x*v, xy.y + other.y*v}
}

func (xy *XY) inside(sz XY) bool {
	return xy.x >= 0 && xy.y >= 0 && xy.x < sz.x && xy.y < sz.y
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	l := 0
	w := 0
	antMap := make(map[uint8][]XY)
	for scanner.Scan() {
		line := scanner.Text()
		for i, c := range line {
			if c != '.' {
				antMap[uint8(c)] = append(antMap[uint8(c)], XY{l, i})
			}
			w = i
		}
		l += 1
	}
	sz := XY{l, w + 1}
	p1nodes := make(map[XY]bool)
	p2nodes := make(map[XY]bool)
	for _, antennas := range antMap {
		for i, a1 := range antennas {
			for _, a2 := range antennas[i+1:] {
				dist := a2.mv(a1, -1)
				loop := func(start XY, dir int) {
					for m := 0; ; m += dir {
						node := start.mv(dist, m)
						if !node.inside(sz) {
							break
						}
						if m == dir {
							p1nodes[node] = true
						}
						p2nodes[node] = true
					}
				}
				loop(a2, 1)
				loop(a1, -1)
			}
		}
	}
	fmt.Printf("part1: %d\n", len(p1nodes))
	fmt.Printf("part2: %d\n", len(p2nodes))
}
