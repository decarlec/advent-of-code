package day10

import (
	"fmt"
	"bufio"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Point struct {
	x, y int
}

var runemat [][]rune
var len int = 0
var cur Point
var prev Point

func Day10() {

	init_runemat()

	for cur.x != -1 {
		cur = get_next(cur)
	}
}

// initialize matrix
func init_runemat() {
	file, err := os.Open("./input/day10")
	check(err)

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		runes := []rune(scanner.Text())
		runemat = append(runemat, runes)
		println(string(runes))
	}
}

// find the starting point
func find_s() Point {
	for i, row := range runemat {
		for j, rune := range row {
			if string(rune) == "S" {
				return Point{j, i}
			}
		}
	}
	panic("could not find start!")
}

// gets the next pipe for a given point. Assumes prev point cannot be next point.
func get_next(point Point) Point {

	var new_point Point
	if len == 0 {
		new_point = find_s()
	} else {

		rune := runemat[point.y][point.x]
		fmt.Printf("Finding next for %s (%d,%d)\n", string(rune), point.x, point.y)

		switch string(rune) {
		case "S":
			//finished
			if len != 1 {
				fmt.Println((len - 1) / 2)
				return Point{-1, -1}
			}

			//hacky way to pick a direction
			new_point = Point{point.x, point.y - 1}
		default:
			//check for possible connecting pipes
			//find the one that isn't the one we just came from
			opts := get_options(point)

			if opts[0].x != prev.x || opts[0].y != prev.y {
				new_point = opts[0]
			} else if opts[1].x != prev.x || opts[1].y != prev.y {
				new_point = opts[1]
			} else {
				panic("no opts not prev!")
			}
		}
	}

	//set prev as current, set new as current and increment count
	prev = point
	len = len + 1
	fmt.Printf("Next is (%d,%d)\n", new_point.x, new_point.y)
	return new_point
}

// gets possible connecting pipes for a point
func get_options(point Point) [2]Point {
	rune := runemat[point.y][point.x]
	var opts [2]Point

	switch string(rune) {
	case "|":
		opts = [2]Point{{point.x, point.y + 1}, {point.x, point.y - 1}}
	case "-":
		opts = [2]Point{{point.x + 1, point.y}, {point.x - 1, point.y}}
	case "L":
		opts = [2]Point{{point.x, point.y - 1}, {point.x + 1, point.y}}
	case "J":
		opts = [2]Point{{point.x - 1, point.y}, {point.x, point.y - 1}}
	case "7":
		opts = [2]Point{{point.x - 1, point.y}, {point.x, point.y + 1}}
	case "F":
		opts = [2]Point{{point.x, point.y + 1}, {point.x + 1, point.y}}
	default:
		panic("this char has no options!")
	}

	return opts
}
