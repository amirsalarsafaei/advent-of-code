package models

import (
	"strconv"
	"strings"

	"github.com/samber/lo"
)

var (
	dx = []int{0, 0, 0, 0, 1, -1}
	dy = []int{0, 0, 1, -1, 0, 0}
	dz = []int{1, -1, 0, 0, 0, 0}
)

type Coordinate struct {
	X int
	Y int
	Z int
}

func NewCoordinate(point string) *Coordinate {
	nums := lo.Map(strings.Split(point, ","), func(item string, _ int) int {
		res, _ := strconv.Atoi(item)
		return res
	})
	return &Coordinate{
		X: nums[0],
		Y: nums[1],
		Z: nums[2],
	}
}

func (c *Coordinate) String() string {
	return strconv.Itoa(c.X) + "," + strconv.Itoa(c.Y) + "," + strconv.Itoa(c.Z)
}

func (c *Coordinate) Adjacents() (res []Coordinate) {
	res = make([]Coordinate, 6)
	for i := 0; i < 6; i++ {
		res[i] = Coordinate{
			X: c.X + dx[i],
			Y: c.Y + dy[i],
			Z: c.Z + dz[i],
		}
	}
	return
}
