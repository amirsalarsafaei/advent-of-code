package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/samber/lo"
)

func main() {
	fileBytes, err := os.ReadFile("./day01/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	elvesBags := strings.Split(inputString, "\n\n")

	maxCal := lo.Max(lo.Map(elvesBags, func(bag string, _ int) int64 {
		return getElfCalories(bag)
	}))

	fmt.Print(maxCal)
}

func getElfCalories(elfBag string) int64 {
	var sum int64

	for _, calStr := range strings.Fields(elfBag) {
		cal, _ := strconv.ParseInt(calStr, 10, 64)
		sum += cal
	}

	return sum
}
