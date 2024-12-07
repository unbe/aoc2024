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
