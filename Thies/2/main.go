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
	reports := parse()
	solve(reports)
}

func parse() [][]int {
	file, err := os.Open("input.txt")
	check(err)
	defer file.Close()

	var reports [][]int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var report []int
		for _, word := range strings.Split(scanner.Text(), " ") {
			res, err := strconv.Atoi(word)
			check(err)
			report = append(report, res)
		}

		reports = append(reports, report)
	}
	return reports
}

func solve(reports [][]int) {
	safe := 0
	safeDampened := 0
	for _, report := range reports {
		reverse := slices.Clone(report)
		slices.Reverse(reverse)
		if isSafe(report) || isSafe(reverse) {
			safe++
		}
		if isSafe2(report) || isSafe2(reverse) {
			safeDampened++
		}
	}

	fmt.Println("Safe reports: ", safe)
	fmt.Println("Dampened Safe reports: ", safeDampened)
}

func isSafe(report []int) bool {
	for i := range len(report) - 1 {
		if report[i] <= report[i+1] || report[i] > report[i+1]+3 {
			return false
		}
	}
	return true
}

func isSafe2(report []int) bool {
	if isSafe(report[1:]) || isSafe(report[:len(report)-1]) {
		return true
	}
	for i := 1; i < len(report)-1; i++ {
		first := report[:i]
		second := report[i+1:]
		if isSafe(slices.Concat(first, second)) {
			return true
		}
	}
	return false
}
