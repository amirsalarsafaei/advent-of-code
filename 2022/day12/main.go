package main

import (
	"fmt"
	"math"
	"os"
	"strings"
)

type point struct {
	x int
	y int
}

var (
	dx           = []int{-1, 1, 0, 0}
	dy           = []int{0, 0, -1, 1}
	n            int
	m            int
	seen         [][]bool
	heatMapLines []string
	bfsStack     []point
	endPoint     point
)

func main() {
	fileBytes, err := os.ReadFile("./day12/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	heatMapLines = strings.Fields(inputString)

	n = len(heatMapLines)
	m = len(heatMapLines[0])

	seen = make([][]bool, n)
	for i := 0; i < n; i++ {
		seen[i] = make([]bool, m)
	}

	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if heatMapLines[i][j] == 'S' {
				bfsStack = []point{
					{
						x: i,
						y: j,
					},
				}
				seen[i][j] = true
				runes := []rune(heatMapLines[i])
				runes[j] = 'a'
				heatMapLines[i] = string(runes)
			} else if heatMapLines[i][j] == 'E' {
				runes := []rune(heatMapLines[i])
				runes[j] = 'z'
				heatMapLines[i] = string(runes)
				endPoint = point{
					x: i,
					y: j,
				}
			}
		}
	}

	i := 0
	level := 0
	for {
		endLevel := len(bfsStack)
		for ; i < endLevel; i++ {
			if bfsStack[i].x == endPoint.x && bfsStack[i].y == endPoint.y {
				fmt.Println(level)
				os.Exit(0)
			}
			bfs(bfsStack[i])
		}
		level++
		if i == len(bfsStack) {
			break
		}
	}

	fmt.Println("no path found")
}

func bfs(p point) {
	for z := 0; z < 4; z++ {
		xi := p.x + dx[z]
		yi := p.y + dy[z]
		if isValid(xi, yi) {
			if !seen[xi][yi] && int(heatMapLines[xi][yi])-int(heatMapLines[p.x][p.y]) <= 1 {
				bfsStack = append(bfsStack, point{
					x: xi,
					y: yi,
				})
				seen[xi][yi] = true
			}
		}
	}
}

func isValid(x, y int) bool {
	return min(x, y) >= 0 && x < n && y < m
}

func min(a ...int) int {
	res := math.MaxInt
	for _, i := range a {
		if i < res {
			res = i
		}
	}
	return res
}
