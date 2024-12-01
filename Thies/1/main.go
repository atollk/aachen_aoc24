package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func abs(i int) int { return max(i, -i) }

func main() {
	file, err := os.Open("input.txt")
	check(err)
	defer file.Close()

	var left []int
	var right []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		var l int
		var r int
		fmt.Sscanln(scanner.Text(), &l, &r)
		left = append(left, l)
		right = append(right, r)
	}

	slices.Sort(left)
	slices.Sort(right)

	partOne(left, right)
	partTwo(left, right)
}

func partOne(left, right []int) {
	sum := 0
	for i := range left {
		sum += abs(left[i] - right[i])
	}

	fmt.Println("Sum: ", sum)
}

func partTwo(left, right []int) {
	score := 0
	for _, l := range left {
		count := 0
		for _, r := range right {
			if l == r {
				count++
			}
		}
		score += count * l
	}

	fmt.Println("Score: ", score)
}
