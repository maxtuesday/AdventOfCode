package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../../input/day02.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	depth := 0
	distance := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		// process line
		lineSplit := strings.Split(scanner.Text(), " ")
		command, magnitudeStr := lineSplit[0], lineSplit[1]
		magnitude, _ := strconv.Atoi(magnitudeStr)
		switch command {
		case "forward":
			distance += magnitude
		case "down":
			depth += magnitude
		case "up":
			depth -= magnitude
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Println("Distance:", distance)
	fmt.Println("Depth:", depth)

	fmt.Println("Result:", distance*depth)
}
