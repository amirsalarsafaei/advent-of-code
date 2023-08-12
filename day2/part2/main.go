package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	fileBytes, err := os.ReadFile("./day2/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	var sum int
	for _, inputLine := range strings.Split(inputString, "\n") {
		opponentMoveCode := int(inputLine[0] - 'A')
		var myMoveCode int
		switch inputLine[2] {
		case 'X':
			myMoveCode = (opponentMoveCode + 2) % 3
		case 'Y':
			myMoveCode = opponentMoveCode
			sum += 3
		case 'Z':
			myMoveCode = (opponentMoveCode + 1) % 3
			sum += 6
		}
		sum += myMoveCode + 1
	}

	fmt.Println(sum)
}
