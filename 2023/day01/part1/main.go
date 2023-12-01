package main

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

func main() {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		firstDigitIdx := strings.IndexFunc(line, isDigit)
		lastDigitIdx := strings.LastIndexFunc(line, isDigit)
		num := toDigit(rune(line[firstDigitIdx]))*10 + toDigit(rune(line[lastDigitIdx]))
		sum += num
	}
	fmt.Println(sum)
}

func isDigit(r rune) bool {
	return '0' <= r && r <= '9'
}

func toDigit(r rune) int {
	return int(r - '0')
}
