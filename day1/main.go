package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func adjustDial(dial int, password int, n int) (int, int) {
	if n == 0 {
		return dial, password
	}
	nextDial := dial + n
	// Went below -100
	for nextDial <= -100 {
		nextDial += 100
		password += 1
	}
	// Landed exactly on zero
	if nextDial == 0 {
		password += 1
	}
	// went over 100
	for nextDial >= 100 {
		nextDial -= 100
		password += 1
	}
	// Check if we crossed zero without landing on it
	if nextDial*dial < 0 {
		password += 1
	}
	// Normalize negative dial positions
	if nextDial < 0 {
		nextDial += 100
	}
	return nextDial, password
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			fmt.Println("Error closing file:", err)
		}
	}(file)

	dial := 50
	password := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if len(line) < 2 {
			fmt.Println("Invalid line:", line)
			continue
		}
		dir := line[0]
		numStr := line[1:]
		n, err := strconv.Atoi(numStr)
		if err != nil {
			fmt.Println("Invalid number in line:", line)
			continue
		}
		if dir == 'L' {
			n = -n
		}
		dial, password = adjustDial(dial, password, n)
	}
	fmt.Println("Final dial position:", dial)
	fmt.Println("Password:", password)
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}
}
