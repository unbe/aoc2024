package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"log"
	"os"
)

type XY struct {
	x, y int
}
type Dir XY
type PosKey struct {
	XY
	Dir
}
type Pos struct {
	PosKey
	dist int
	prev []*Pos
}
type PQ []*Pos

func (pq PQ) Len() int           { return len(pq) }
func (pq PQ) Less(i, j int) bool { return pq[i].dist < pq[j].dist }
func (pq *PQ) Push(x any)        { *pq = append(*pq, x.(*Pos)) }
func (pq PQ) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i] }
func (pq *PQ) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	*pq = old[0 : n-1]
	return item
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var mp [][]byte
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		mp = append(mp, []byte(line))
	}
	var start Pos
	var goal Pos
	q := make(PQ, 0)
	seen := make(map[PosKey]*Pos)
	goals := []*Pos{}
	heap.Init(&q)
	for i, l := range mp {
		for j, r := range l {
			if r == byte('S') {
				start = Pos{PosKey{XY{i, j}, Dir{0, -1}}, 0, nil}
				heap.Push(&q, &start)
			}
			if r == byte('E') {
				goal = Pos{PosKey{XY{i, j}, Dir{0, 0}}, 0, nil}
			}
		}
	}
	for len(q) > 0 {
		p := *heap.Pop(&q).(*Pos)
		if p.XY == goal.XY {
			if len(goals) > 0 && p.dist > goals[0].dist {
				break
			}
			goals = append(goals, &p)
		}
		other, didsee := seen[p.PosKey]
		if didsee {
			if other.dist == p.dist {
				other.prev = append(other.prev, p.prev...)
			}
			continue
		}
		seen[p.PosKey] = &p
		np := Pos{PosKey{XY{p.XY.x + p.Dir.x, p.XY.y + p.Dir.y}, p.Dir}, p.dist + 1, []*Pos{&p}}
		if mp[np.XY.x][np.XY.y] != byte('#') {
			heap.Push(&q, &np)
		}
		heap.Push(&q, &Pos{PosKey{p.XY, Dir{-p.Dir.y, p.Dir.x}}, p.dist + 1000, []*Pos{&p}})
		heap.Push(&q, &Pos{PosKey{p.XY, Dir{p.Dir.y, -p.Dir.x}}, p.dist + 1000, []*Pos{&p}})
	}
	fmt.Printf("part1: %v\n", goals[0].dist)
	part := make(map[XY]bool)
	for len(goals) > 0 {
		part[goals[0].XY] = true
		goals = append(goals[0].prev, goals[1:]...)
	}
	fmt.Printf("part2: %v\n", len(part))
}
