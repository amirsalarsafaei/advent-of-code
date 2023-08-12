package main

import (
	"fmt"
	"os"
	"sort"
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

	elvesCals := lo.Map(elvesBags, func(bag string, _ int) int {
		return getElfCalories(bag)
	})

	sort.Ints(elvesCals)

	var sum int

	for i := 1; i <= 3; i++ {
		sum += elvesCals[len(elvesCals)-i]
	}

	fmt.Print(sum)
}

func getElfCalories(elfBag string) int {
	var sum int

	for _, calStr := range strings.Fields(elfBag) {
		cal, _ := strconv.Atoi(calStr)
		sum += cal
	}

	return sum
}
