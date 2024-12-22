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
	bits := []string{}

	for scanner.Scan() {
		bits = append(bits, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	o2 := reduceBits(bits, 0, '1', '0')
	co2 := reduceBits(bits, 0, '0', '1')

	o, _ := strconv.ParseInt(o2, 2, 64)
	c, _ := strconv.ParseInt(co2, 2, 64)

	fmt.Println("O2:", o2, o)
	fmt.Println("CO2:", co2, c)

	fmt.Println("Result:", o*c)
}

func reduceBits(bits []string, index int, mostCommonBit byte, leastCommonBit byte) string {
	if len(bits) == 1 {
		return bits[0]
	}

	// find most common bit
	bitFreq := 0
	for j := 0; j < len(bits); j++ {
		if bits[j][index] == '1' {
			bitFreq += 1
		} else {
			bitFreq -= 1
		}
	}

	filteredBits := []string{}
	for b := 0; b < len(bits); b++ {
		if bitFreq >= 0 && bits[b][index] == mostCommonBit {
			filteredBits = append(filteredBits, bits[b])
		} else if bitFreq < 0 && bits[b][index] == leastCommonBit {
			filteredBits = append(filteredBits, bits[b])
		}
	}
	index++
	return reduceBits(filteredBits, index, mostCommonBit, leastCommonBit)
}
