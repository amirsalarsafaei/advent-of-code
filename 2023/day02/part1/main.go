package main

import (
	_ "embed"
	"fmt"
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
		gameTitle, game, ok := strings.Cut(line, ":")
		if !ok {
			logrus.Panic("invalid input format no : in game line")
		}

		gameID, err := strconv.Atoi(strings.TrimPrefix(gameTitle, "Game "))
		if err != nil {
			logrus.WithField("game_title", gameTitle).Panic("could not extract game id")
		}

		if isGameValid(game) {
			res += gameID
		}
	}

	fmt.Println(res)
}

func isGameValid(game string) bool {

	subGames := strings.Split(game, ";")
	if len(subGames) == 0 {
		subGames = []string{game}
	}

	return lo.EveryBy(subGames, isSubGameValid)
}

func isSubGameValid(subGame string) bool {
	subGame = strings.TrimSpace(subGame)
	cubeSets := lo.Map(strings.Split(subGame, ","), ConvertStringToCubeSet)
	for _, cubeSet := range cubeSets {
		if cubesCnt[cubeSet.Color] < cubeSet.Count {
			return false
		}
	}
	return true
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
