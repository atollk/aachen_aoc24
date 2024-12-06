package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	rules, manuals := parse()

	sum1 := 0
	sum2 := 0
	for _, manual := range manuals {
		if validate(manual, rules) {
			sum1 += manual[len(manual)/2]
		} else {
			reorder(manual, rules)
			sum2 += manual[len(manual)/2]
		}
	}
	fmt.Println("Part 1: ", sum1)
	fmt.Println("Part 2: ", sum2)
}

func parse() (rules map[int][]int, manuals [][]int) {
	file, err := os.Open("input.txt")
	check(err)
	defer file.Close()

	rules = make(map[int][]int)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var l, r int
		_, err = fmt.Sscanf(scanner.Text(), "%d|%d", &l, &r)
		if err != nil {
			break
		}
		rule := rules[l]
		rule = append(rule, r)
		rules[l] = rule
	}

	for scanner.Scan() {
		manuals = append(manuals, parseInts(scanner.Text()))
	}
	return
}

func parseInts(line string) (ints []int) {
	for _, word := range strings.Split(line, ",") {
		res, err := strconv.Atoi(word)
		check(err)
		ints = append(ints, res)
	}
	return
}

func validate(manual []int, rules map[int][]int) bool {
	for i, v := range manual {
		for _, rule := range rules[v] {
			if slices.Contains(manual[:i], rule) {
				return false
			}
		}
	}
	return true
}

func reorder(manual []int, rules map[int][]int) {
	for i, v := range manual {
		if isFirst(i, manual, rules) {
			manual[i] = manual[0]
			manual[0] = v
			reorder(manual[1:], rules)
			return
		}
	}
}

func isFirst(i int, manual []int, rules map[int][]int) bool {
	v := manual[i]
	for j, o := range manual {
		if i == j {
			continue
		}
		if slices.Contains(rules[o], v) {
			return false
		}
	}
	return true
}
