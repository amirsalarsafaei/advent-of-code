package main

import (
	"adventofcode/day18/models"
	"fmt"
	"math"
	"os"
	"strings"

	"github.com/samber/lo"
)

var (
	maxX, maxY, maxZ int
	seen             = map[string]bool{}
	isDroplet        = map[string]bool{}
	ans              int
	bfsStack         []models.Coordinate
)

func main() {
	fileBytes, err := os.ReadFile("./day18/part2/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)
	cords := lo.Map(strings.Fields(inputString), func(item string, index int) *models.Coordinate {
		return models.NewCoordinate(item)
	})

	// Transform so that 0 planes are empty
	for _, cord := range cords {
		cord.X += 1
		cord.Z += 1
		cord.Y += 1
		isDroplet[cord.String()] = true
	}

	// Find Space Dimensions
	for _, cord := range cords {
		maxX = max(maxX, cord.X)
		maxY = max(maxY, cord.Y)
		maxZ = max(maxZ, cord.Z)
	}

	maxX += 1
	maxY += 1
	maxZ += 1

	dfs(models.Coordinate{})

	fmt.Println(ans)
}

func dfs(c models.Coordinate) {
	seen[c.String()] = true
	for _, adj := range c.Adjacents() {
		if isDroplet[adj.String()] {
			ans += 1
		} else if isValid(adj) {
			dfs(adj)
		}
	}
}

func isValid(c models.Coordinate) bool {
	return !seen[c.String()] && c.X <= maxX && c.Y <= maxY && c.Z <= maxZ && min(c.X, c.Y, c.Z) >= 0
}

func max(nums ...int) int {
	res := math.MinInt
	for _, num := range nums {
		if num > res {
			res = num
		}
	}
	return res
}

func min(nums ...int) int {
	res := math.MaxInt
	for _, num := range nums {
		if num < res {
			res = num
		}
	}
	return res
}
