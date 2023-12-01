package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	fileBytes, err := os.ReadFile("./day02/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	var sum int
	for _, inputLine := range strings.Split(inputString, "\n") {
		opponentMoveCode := inputLine[0] - 'A'
		myMoveCode := inputLine[2] - 'X'
		switch {
		case myMoveCode == opponentMoveCode:
			sum += 3
		case (opponentMoveCode+1)%3 == myMoveCode:
			sum += 6
		}

		sum += int(myMoveCode + 1)
	}

	fmt.Println(sum)
}
