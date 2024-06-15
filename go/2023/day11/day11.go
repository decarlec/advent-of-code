package day11

import (
	//"fmt"
	//"io"
	"bufio"
	"os"
	//"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Point struct {
	x, y int
}

func Day11() {
	var runemat = read_runemat()
	var expanded = expand_vertical(runemat)
	var transformed = transform(expanded)

	for _, line := range expanded {
		println("expanded")
		println(string(line))
	}

	for _, line := range transformed {
		println("transformed")
		println(string(line))
	}
}

// initialize matrix
func read_runemat() [][]rune {
	var runemat [][]rune
	file, err := os.Open("./input/day11")
	check(err)

	defer file.Close()
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		runes := []rune(scanner.Text())
		runemat = append(runemat, runes)
	}

	return runemat
}

func check_empty(line string) bool {
	for _, t := range []rune(line) {
		if t != '.' {
			return false
		}
	}
	return true
}

func expand_vertical(runemat [][]rune) [][]rune {
	var newMat [][]rune

	for _, row := range runemat {
		if check_empty(string(row)) {
			newMat = append(newMat, row)
		}
		newMat = append(newMat, row)
	}

	return newMat
}

func transform(runemat [][]rune) [][]rune {
	x := len(runemat[0])
	y := len(runemat)

	newMat := make([][]rune, x, y)

	println(len(runemat))
	println(len(newMat))

	for i, row := range runemat {
		println(i)
		newMat[i] = make([]rune, y)
		for j := range row {
			println(j)
			newMat[j][i] = runemat[i][j]
		}
	}
	return newMat
}
