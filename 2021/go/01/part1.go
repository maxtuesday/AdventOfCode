package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	b, err := os.ReadFile("../../input/day01.txt")
	if err != nil {
		log.Fatal(err)
	}

	data := string(b)
	levels := strings.Split(data, "\n")

	count := 0
	prevVal, err := strconv.Atoi(levels[0])
	if err != nil {
		log.Fatal(err)
	}

	for _, level := range levels {
		c, err := strconv.Atoi(level)
		if err != nil {
			fmt.Println(err)
			break
		}
		if c > prevVal {
			count++
		}
		prevVal = c
	}

	fmt.Println(count)
}
