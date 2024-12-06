package main

import (
	"fmt"
	"os"
	"regexp"
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
	s = strings.ReplaceAll(s, "\n", "@") // regex does not match over newline

	part1(s)
	part2(s)
}

func part1(s string) {
	width := strings.Index(s, "@")

	h := searchXMAS(s, 0)
	v := searchXMAS(s, width)
	slash := searchXMAS(s, width-1)
	backslash := searchXMAS(s, width+1)

	fmt.Printf(`Part 1: -: %d |: %d /: %d \:%d sum: %d`, h, v, slash, backslash, h+v+slash+backslash)
	fmt.Println()
}

func searchXMAS(s string, distance int) int {
	matches := 0
	matches += searchPattern(s, fmt.Sprintf(`X.{%d}M.{%d}A.{%d}S`, distance, distance, distance))
	matches += searchPattern(s, fmt.Sprintf(`S.{%d}A.{%d}M.{%d}X`, distance, distance, distance))
	return matches
}

func part2(s string) {
	width := strings.Index(s, "@")

	matches := 0
	matches += searchPattern(s, fmt.Sprintf(`M.M.{%d}A.{%d}S.S`, width-1, width-1))
	matches += searchPattern(s, fmt.Sprintf(`M.S.{%d}A.{%d}M.S`, width-1, width-1))
	matches += searchPattern(s, fmt.Sprintf(`S.M.{%d}A.{%d}S.M`, width-1, width-1))
	matches += searchPattern(s, fmt.Sprintf(`S.S.{%d}A.{%d}M.M`, width-1, width-1))

	fmt.Println("Part 2: ", matches)
}

func searchPattern(s string, pattern string) int {
	regex := regexp.MustCompile(pattern)
	matches := 0
	for {
		i := regex.FindStringIndex(s)
		if i == nil {
			return matches
		}
		matches++
		s = s[i[0]+1:]
	}
}
