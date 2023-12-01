package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var (
	mathOperationRegex = regexp.MustCompile(`^(?P<monkeyName>\w+):\s(?P<monkey1>\w+)\s(?P<operator>[*-/+])\s(?P<monkey2>\w+)`)
	instantNumberRegex = regexp.MustCompile(`^(?P<monkeyName>\w+):\s(?P<number>\d+)`)
	monkeys            = map[string]Monkey{}
)

type Operator func(a int64, b int64) int64

var (
	add Operator = func(a, b int64) int64 {
		return a + b
	}
	subtract Operator = func(a, b int64) int64 {
		return a - b
	}
	multiply Operator = func(a, b int64) int64 {
		return a * b
	}
	divide Operator = func(a, b int64) int64 {
		return a / b
	}
)

type Monkey interface {
	Result() int64
}

type instantAnswerMonkey struct {
	number int64
}

func (m *instantAnswerMonkey) Result() int64 {
	return m.number
}

type mathOperationMonkey struct {
	number       int64
	isCalculated bool
	leftMonkey   string
	rightMonkey  string
	op           Operator
}

func (m *mathOperationMonkey) Result() int64 {
	if !m.isCalculated {
		m.number = m.op(monkeys[m.leftMonkey].Result(), monkeys[m.rightMonkey].Result())
		m.isCalculated = true
	}
	return m.number
}

func main() {
	fileBytes, err := os.ReadFile("./day21/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	for _, inputLine := range strings.Split(inputString, "\n") {
		if mathOperationRegex.MatchString(inputLine) {
			groupsMap := getRegexGroupsMap(mathOperationRegex, inputLine)
			monkeys[groupsMap["monkeyName"]] = &mathOperationMonkey{
				op:          getOperator(groupsMap["operator"]),
				leftMonkey:  groupsMap["monkey1"],
				rightMonkey: groupsMap["monkey2"],
			}
		} else if instantNumberRegex.MatchString(inputLine) {
			groupsMap := getRegexGroupsMap(instantNumberRegex, inputLine)
			number, _ := strconv.ParseInt(groupsMap["number"], 10, 64)
			monkeys[groupsMap["monkeyName"]] = &instantAnswerMonkey{
				number: number,
			}
		} else {
			fmt.Println(inputLine)
			panic("regex didn't match")
		}
	}

	fmt.Println(monkeys["root"].Result())

}

func getOperator(op string) Operator {
	switch op {
	case "+":
		return add
	case "-":
		return subtract
	case "/":
		return divide
	case "*":
		return multiply
	default:
		panic("invalid op")
	}
}

func getRegexGroupsMap(regex *regexp.Regexp, s string) map[string]string {
	match := regex.FindStringSubmatch(s)
	result := make(map[string]string)
	for i, name := range regex.SubexpNames() {
		if i != 0 && name != "" {
			result[name] = match[i]
		}
	}
	return result
}
