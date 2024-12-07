package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// This should be memoized, but can I be bothered? It's fast enough.
func combine(ops []int, canConcat bool) <-chan int {
	chnl := make(chan int)
	go func() {
		defer close(chnl)
		if len(ops) == 1 {
			chnl <- ops[0]
			return
		}
		for x := range combine(ops[:len(ops)-1], canConcat) {
			chnl <- ops[len(ops)-1] * x
			chnl <- ops[len(ops)-1] + x
			if canConcat {
				v, err := strconv.Atoi(strconv.Itoa(x) + strconv.Itoa(ops[len(ops)-1]))
				if err != nil {
					panic(err)
				}
				chnl <- v
			}
		}
	}()
	return chnl
}

func main() {
	/*
	   Out of curiosity: PGO is ~6% faster on this.

	   % go build -pgo=cpu.pprof day07.go && time ./day07 ~/Downloads/input.txt
	   part1: 882304362421
	   part2: 145149066755184
	   ./day07 ~/Downloads/input.txt  4.16s user 1.38s system 180% cpu 3.069 total

	   % go build day07.go && time ./day07 ~/Downloads/input.txt
	   part1: 882304362421
	   part2: 145149066755184
	   ./day07 ~/Downloads/input.txt  4.42s user 1.47s system 185% cpu 3.170 total

	   Profile generator:
	   	f, err := os.Create("cpu.pprof")
	   	if err != nil {
	   		panic(err)
	   	}
	   	pprof.StartCPUProfile(f)
	   	defer pprof.StopCPUProfile()
	*/
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	total1 := 0
	total2 := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		sp := strings.Split(line, ": ")
		target, err := strconv.Atoi(sp[0])
		if err != nil {
			panic(err)
		}
		ops_s := strings.Split(sp[1], " ")
		var ops = make([]int, len(ops_s))
		for i, s := range ops_s {
			n, err := strconv.Atoi(s)
			if err != nil {
				panic(err)
			}
			ops[i] = n
		}
		if canDo(target, ops, false) {
			total1 += target
		}
		if canDo(target, ops, true) {
			total2 += target
		}
	}
	fmt.Printf("part1: %d\n", total1)
	fmt.Printf("part2: %d\n", total2)
}

func canDo(target int, ops []int, canConcat bool) bool {
	for c := range combine(ops, canConcat) {
		if c == target {
			return true
		}
	}
	return false
}
