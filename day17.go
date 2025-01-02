package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const A = byte('A')
const B = byte('B')
const C = byte('C')

func ü(rdr *bufio.Reader) string {
	s, err := rdr.ReadString('\n')
	if err != nil {
		panic(err)
	}
	return strings.TrimSuffix(s, "\n")
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	regs := map[byte]int{}
	rdr := bufio.NewReader(file)
	for {
		rv := strings.Split(ü(rdr), ": ")
		if len(rv) < 2 {
			break
		}
		reg := strings.Split(rv[0], " ")[1]
		val, _ := strconv.Atoi(rv[1])
		regs[reg[0]] = val
	}
	pd := strings.Split(ü(rdr), ": ")
	prog := []byte{}
	for _, cmd := range strings.Split(pd[1], ",") {
		val, _ := strconv.Atoi(cmd)
		prog = append(prog, byte(val))
	}
	out := run(regs, prog)
	strout := ""
	for _, x := range out {
		strout += "," + strconv.Itoa(int(x))
	}
	fmt.Printf("part1: %v\n", strout[1:])

	valueForA := []int{0}
	for i := range prog {
		valueForA = bruteforceOne(valueForA, prog, i+1)
	}
	var m int
	for i, f := range valueForA {
		if i == 0 || f < m {
			m = f
		}
	}
	fmt.Printf("part2: %v\n", m)
}

func bruteforceOne(valuesForA []int, prog []byte, idx int) []int {
	newValuesForA := []int{}
	out := []byte{}
	regs := map[byte]int{}
	for _, val := range valuesForA {
		base := val << 3
		for i := range 8 {
			regs[A] = base + i
			regs[B] = 0
			regs[C] = 0
			out = run(regs, prog)
			if len(out) == idx && out[len(out)-idx] == prog[len(prog)-idx] {
				newValuesForA = append(newValuesForA, base+i)
			}
		}
	}
	return newValuesForA
}

func run(regs map[byte]int, prog []byte) []byte {
	ip := 0
	combo := func(arg byte) int {
		if arg <= 3 {
			return int(arg)
		} else if arg <= 6 {
			return regs[byte('A')+arg-4]
		} else {
			panic(arg)
		}
	}
	out := []byte{}
	dt := map[byte]func(byte){
		0: func(arg byte) { regs[A] = regs[A] >> combo(arg) },
		1: func(arg byte) { regs[B] = regs[B] ^ int(arg) },
		2: func(arg byte) { regs[B] = combo(arg) % 8 },
		3: func(arg byte) {
			if regs[A] != 0 {
				ip = int(arg) - 2
			}
		},
		4: func(arg byte) { regs[B] = regs[B] ^ regs[C] },
		5: func(arg byte) {
			out = append(out, byte(combo(arg)%8))
		},
		6: func(arg byte) { regs[B] = regs[A] >> combo(arg) },
		7: func(arg byte) { regs[C] = regs[A] >> combo(arg) },
	}
	for ip < len(prog) {
		is := prog[ip]
		arg := prog[ip+1]
		dt[is](arg)
		ip += 2
	}
	return out
}
