package main

import (
	_ "embed"
	"fmt"
	"golang.org/x/exp/constraints"
	"strconv"
	"strings"

	"github.com/samber/lo"
	"github.com/sirupsen/logrus"
)

//go:embed input.txt
var input string

type CubeColor string

const (
	Green CubeColor = "green"
	Red   CubeColor = "red"
	Blue  CubeColor = "blue"
)

var AllColors = []CubeColor{
	Green,
	Red,
	Blue,
}

type CubeSet struct {
	Color CubeColor
	Count int
}

var cubesCnt = map[CubeColor]int{
	Green: 13,
	Red:   12,
	Blue:  14,
}

func main() {
	res := 0

	for _, line := range strings.Split(input, "\n") {
		_, game, ok := strings.Cut(line, ":")
		if !ok {
			logrus.Panic("invalid input format no : in game line")
		}

		minimumCubes := GetGameMinimumCubes(game)

		power := 1
		for _, cnt := range minimumCubes {
			power *= cnt
		}

		res += power
	}

	fmt.Println(res)
}

func GetGameMinimumCubes(game string) map[CubeColor]int {

	minimumCubes := lo.SliceToMap(AllColors,
		func(item CubeColor) (CubeColor, int) {
			return item, 0
		},
	)

	subGames := strings.Split(game, ";")
	if len(subGames) == 0 {
		subGames = []string{game}
	}

	for _, subGame := range subGames {
		cubeSets := GetSubGameCubeSets(subGame)

		for _, cubeSet := range cubeSets {
			minimumCubes[cubeSet.Color] = max(minimumCubes[cubeSet.Color], cubeSet.Count)
		}
	}

	return minimumCubes
}

func GetSubGameCubeSets(subGame string) []CubeSet {
	subGame = strings.TrimSpace(subGame)
	cubeSets := lo.Map(strings.Split(subGame, ","), ConvertStringToCubeSet)

	return cubeSets
}

func ConvertStringToCubeSet(cubeSetStr string, _ int) CubeSet {
	cubeSetStr = strings.TrimSpace(cubeSetStr)
	cntStr, c, ok := strings.Cut(cubeSetStr, " ")
	if !ok {
		logrus.WithField("cube_set_string", cubeSetStr).Panic("invalid cube set string format")
	}

	color := CubeColor(c)
	if !lo.Contains(AllColors, color) {
		logrus.WithField("cube_set_string", cubeSetStr).Panic("invalid color")
	}

	count, err := strconv.Atoi(cntStr)
	if err != nil {
		logrus.WithField("cube_set_string", cubeSetStr).Panic("invalid count")
	}

	return CubeSet{Color: color, Count: count}
}

func max[T constraints.Ordered](a ...T) T {
	if len(a) == 0 {
		return *new(T)
	}

	if len(a) == 1 {
		return a[0]
	}

	res := a[0]
	for _, b := range a {
		if res < b {
			res = b
		}
	}

	return res
}
