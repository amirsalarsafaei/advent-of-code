package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

const human = "humn"

var (
	mathOperationRegex = regexp.MustCompile(`^(?P<monkeyName>\w+):\s(?P<monkey1>\w+)\s(?P<operator>[*-/+])\s(?P<monkey2>\w+)`)
	instantNumberRegex = regexp.MustCompile(`^(?P<monkeyName>\w+):\s(?P<number>\d+)`)
	monkeys            = map[string]Monkey{}
)

type OperatorFunc func(a int64, b int64) int64

type Operator string

var (
	addFunc OperatorFunc = func(a, b int64) int64 {
		return a + b
	}
	subtractFunc OperatorFunc = func(a, b int64) int64 {
		return a - b
	}
	multiplyFunc OperatorFunc = func(a, b int64) int64 {
		return a * b
	}
	divideFunc OperatorFunc = func(a, b int64) int64 {
		return a / b
	}

	add      Operator = "+"
	subtract Operator = "-"
	multiply Operator = "*"
	divide   Operator = "/"
)

type Monkey interface {
	Result() int64
	NeedsHuman() bool
}

type instantAnswerMonkey struct {
	number  int64
	isHuman bool
}

func (m *instantAnswerMonkey) Result() int64 {
	return m.number
}

func (m *instantAnswerMonkey) NeedsHuman() bool {
	return m.isHuman
}

type mathOperationMonkey struct {
	number       int64
	isCalculated bool
	needsHuman   bool
	leftMonkey   string
	rightMonkey  string
	opFunc       OperatorFunc
	op           Operator
}

func (m *mathOperationMonkey) calculate() {
	m.number = m.opFunc(m.getLeftMonkey().Result(), m.getRightMonkey().Result())
	m.needsHuman = m.getLeftMonkey().NeedsHuman() || m.getRightMonkey().NeedsHuman()
	m.isCalculated = true
}

func (m *mathOperationMonkey) getLeftMonkey() Monkey {
	return monkeys[m.leftMonkey]
}

func (m *mathOperationMonkey) getRightMonkey() Monkey {
	return monkeys[m.rightMonkey]
}

func (m *mathOperationMonkey) Result() int64 {
	if !m.isCalculated {
		m.calculate()
	}

	return m.number
}

func (m *mathOperationMonkey) NeedsHuman() bool {
	if !m.isCalculated {
		m.calculate()
	}

	return m.needsHuman
}

func main() {
	fileBytes, err := os.ReadFile("./day21/part2/input.txt")
	if err != nil {
		panic(err)
	}

	inputString := string(fileBytes)

	for _, inputLine := range strings.Split(inputString, "\n") {
		if mathOperationRegex.MatchString(inputLine) {
			groupsMap := getRegexGroupsMap(mathOperationRegex, inputLine)
			monkeys[groupsMap["monkeyName"]] = &mathOperationMonkey{
				opFunc:      getOperatorFunc(Operator(groupsMap["operator"])),
				leftMonkey:  groupsMap["monkey1"],
				rightMonkey: groupsMap["monkey2"],
				op:          Operator(groupsMap["operator"]),
			}
		} else if instantNumberRegex.MatchString(inputLine) {
			groupsMap := getRegexGroupsMap(instantNumberRegex, inputLine)
			number, _ := strconv.ParseInt(groupsMap["number"], 10, 64)
			monkeys[groupsMap["monkeyName"]] = &instantAnswerMonkey{
				number:  number,
				isHuman: groupsMap["monkeyName"] == human,
			}
		} else {
			fmt.Println(inputLine)
			panic("regex didn't match")
		}
	}

	root := (monkeys["root"]).(*mathOperationMonkey)
	if root.getLeftMonkey().NeedsHuman() && root.getRightMonkey().NeedsHuman() {
		panic("I don't know how to solve this version of question :D")
	}

	if !root.getLeftMonkey().NeedsHuman() && !root.getRightMonkey().NeedsHuman() {
		if root.getRightMonkey().Result() != root.getLeftMonkey().Result() {
			fmt.Println("There is no way to solve the problem no matter whatever human says")
		} else {
			fmt.Println("any number is the answer use 42 :D")
		}
		os.Exit(0)
	}

	var monkey Monkey
	var equalTo int64
	if root.getRightMonkey().NeedsHuman() {
		monkey = root.getRightMonkey()
		equalTo = root.getLeftMonkey().Result()
	} else {
		monkey = root.getLeftMonkey()
		equalTo = root.getRightMonkey().Result()
	}

	for {

		switch v := monkey.(type) {
		case *instantAnswerMonkey:
			fmt.Println(equalTo)
			os.Exit(0)
		case *mathOperationMonkey:
			if v.getRightMonkey().NeedsHuman() {
				otherRes := v.getLeftMonkey().Result()
				switch v.op {
				case add:
					equalTo = equalTo - otherRes
				case multiply:
					equalTo = equalTo / otherRes
				case divide:
					equalTo = otherRes / equalTo
				case subtract:
					equalTo = otherRes - equalTo
				default:
					panic("invalid op")
				}
				monkey = v.getRightMonkey()
			} else {
				otherRes := v.getRightMonkey().Result()
				switch v.op {
				case add:
					equalTo = equalTo - otherRes
				case multiply:
					equalTo = equalTo / otherRes
				case divide:
					equalTo = otherRes * equalTo
				case subtract:
					equalTo = otherRes + equalTo
				default:
					panic("invalid op")
				}
				monkey = v.getLeftMonkey()
			}

		}

	}

}

func getOperatorFunc(op Operator) OperatorFunc {
	switch op {
	case add:
		return addFunc
	case subtract:
		return subtractFunc
	case divide:
		return divideFunc
	case multiply:
		return multiplyFunc
	default:
		panic("invalid opFunc")
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
