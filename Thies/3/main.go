package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.ReadFile("input.txt")
	check(err)

	s := string(file)
	fmt.Println("Part 1: ", mul(s))
	fmt.Println("Part 2: ", do(s))
}

func mul(s string) int {
	regex, err := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	check(err)
	matches := regex.FindAllStringSubmatch(s, -1)

	mul := 0
	for _, match := range matches {
		a, err := strconv.Atoi(string(match[1]))
		check(err)
		b, err := strconv.Atoi(string(match[2]))
		check(err)
		mul += a * b
	}
	return mul
}

func do(s string) int {
	end := strings.Index(s, "don't()")
	if end == -1 {
		return mul(s)
	}
	mul := mul(s[:end])
	s = s[end+7:]

	start := strings.Index(s, "do()")
	if start == -1 {
		return mul
	}

	return mul + do(s[start+3:])
}
