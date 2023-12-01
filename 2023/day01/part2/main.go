package main

import (
	_ "embed"
	"errors"
	"fmt"
	"strings"
)

//go:embed input.txt
var input string

var spelledDigits = []string{
	"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
}

func main() {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		num, err := getStringCalibrationValue(line)
		if err != nil {
			panic(err.Error())
		}

		sum += num
	}
	fmt.Println(sum)
}

func getStringCalibrationValue(line string) (int, error) {
	firstDigitCharIdx := strings.IndexFunc(line, isDigit)
	lastDigitCharIdx := strings.LastIndexFunc(line, isDigit)

	var firstDigitIdx, firstDigit, lastDigitIdx, lastDigit int
	if firstDigitCharIdx != -1 {
		firstDigitIdx = firstDigitCharIdx
		firstDigit = toDigit(rune(line[firstDigitIdx]))
		lastDigitIdx = lastDigitCharIdx
		lastDigit = toDigit(rune(line[lastDigitIdx]))
	} else {
		firstDigitIdx = -1
		lastDigitIdx = -1
	}

	for i, spell := range spelledDigits {
		digit := i + 1
		idx := strings.Index(line, spell)
		if idx == -1 {
			continue
		}

		if idx < firstDigitIdx || firstDigitIdx == -1 {
			firstDigitIdx = idx
			firstDigit = digit
		}

		lastIdx := strings.LastIndex(line, spell)

		if lastIdx > lastDigitIdx || lastDigitIdx == -1 {
			lastDigitIdx = lastIdx
			lastDigit = digit
		}
	}

	if firstDigitIdx == -1 {
		return 0, errors.New("string has no digit")
	}

	return 10*firstDigit + lastDigit, nil
}

func isDigit(r rune) bool {
	return '0' <= r && r <= '9'
}

func toDigit(r rune) int {
	return int(r - '0')
}
