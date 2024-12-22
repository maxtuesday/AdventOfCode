package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("../../input/day03.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	bitlen := 12
	bits := make([]int, bitlen)

	for scanner.Scan() {
		line := scanner.Text()
		for i := 0; i < bitlen; i++ {
			if line[i] == '1' {
				bits[i] += 1
			} else {
				bits[i] -= 1
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	gamma, epsilon := "", ""
	for _, bit := range bits {
		if bit > 0 {
			gamma += "1"
			epsilon += "0"
		} else {
			gamma += "0"
			epsilon += "1"
		}
	}

	g, _ := strconv.ParseInt(gamma, 2, 64)
	e, _ := strconv.ParseInt(epsilon, 2, 64)

	fmt.Println("Gamma:", gamma, g)
	fmt.Println("Epsilon:", epsilon, e)

	fmt.Println("Result:", g*e)
}
