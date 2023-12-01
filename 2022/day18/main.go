package main

import (
	"adventofcode/day18/models"
	"fmt"
	"os"
	"strings"

	"github.com/samber/lo"
)

var (
	seen = map[string]bool{}
)

func main() {
	fileBytes, err := os.ReadFile("./day18/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)
	cords := lo.Map(strings.Fields(inputString), func(item string, index int) *models.Coordinate {
		return models.NewCoordinate(item)
	})

	ans := 0

	for _, cord := range cords {
		seen[cord.String()] = true
		ans += 6
		for _, adj := range cord.Adjacents() {
			if seen[adj.String()] {
				ans -= 2
			}
		}
	}

	fmt.Println(ans)
}
