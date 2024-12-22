package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

const input_file = "../../input/day16.txt"

func LoadData(filename string) string {
	content, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}

func main() {
	input := LoadData(input_file)
	Part1(input)
	Part2(input)
}

func convertHexToBin(hex string) string {
	s := ""
	for _, c := range hex {
		d := c - '0'
		if d > 9 {
			d = c - 'A' + 10
		}
		b := fmt.Sprintf("%04b", d)
		s += b
	}
	return s
}

func convBinToDec(b string) int {
	n, _ := strconv.ParseUint(b, 2, 64)
	return int(n)
}

func parsePacket(bin string, readIndex int) (index int, ver int, lit_val_ret int) {
	version := convBinToDec(bin[readIndex : readIndex+3])
	readIndex += 3
	typeID := convBinToDec(bin[readIndex : readIndex+3])
	readIndex += 3

	if typeID == 4 { // literal value (this is the end of recursion)
		num := bin[readIndex : readIndex+5]
		num_s := bin[readIndex+1 : readIndex+5]
		readIndex += 5
		for num[0] != '0' {
			num = bin[readIndex : readIndex+5]
			num_s += bin[readIndex+1 : readIndex+5]
			readIndex += 5
		}
		val := convBinToDec(num_s)
		return readIndex, version, val
	} else { // operator
		lenTypeID := bin[readIndex]
		readIndex++

		lit_vals := []int{}
		switch lenTypeID {
		case '0':
			// next 15 bits are total len of sub-packet
			L := convBinToDec(bin[readIndex : readIndex+15])
			readIndex += 15
			n_readIndex, n_ver, lit_val := parsePacket(bin, readIndex)
			lit_vals = append(lit_vals, lit_val)
			version += n_ver
			for n_readIndex-readIndex != L {
				n_readIndex, n_ver, lit_val = parsePacket(bin, n_readIndex)
				lit_vals = append(lit_vals, lit_val)
				version += n_ver
			}
			readIndex = n_readIndex
		case '1':
			// next 11 bits are num of sub-packets following
			L := convBinToDec(bin[readIndex : readIndex+11])
			readIndex += 11
			for i := 0; i < L; i++ {
				n_readIndex, n_ver, lit_val := parsePacket(bin, readIndex)
				lit_vals = append(lit_vals, lit_val)
				readIndex = n_readIndex
				version += n_ver
			}
		}

		// Perform action on lit_vals based on typeID
		var val int
		switch typeID {
		case 0: // sum
			val = Sum(lit_vals)
		case 1: // product
			val = Prod(lit_vals)
		case 2: // minimum
			val = Min(lit_vals)
		case 3: // maximum
			val = Max(lit_vals)
		case 5: // greater than
			if lit_vals[0] > lit_vals[1] {
				val = 1
			} else {
				val = 0
			}
		case 6: // less than
			if lit_vals[0] < lit_vals[1] {
				val = 1
			} else {
				val = 0
			}
		case 7: // equal to
			if lit_vals[0] == lit_vals[1] {
				val = 1
			} else {
				val = 0
			}
		}
		return readIndex, version, val
	}
}

func Sum(a []int) int {
	sum := 0
	for _, e := range a {
		sum += e
	}
	return sum
}

func Prod(a []int) int {
	prod := 1
	for _, e := range a {
		prod *= e
	}
	return prod
}

func Min(a []int) int {
	sort.Ints(a)
	return a[0]
}

func Max(a []int) int {
	sort.Ints(a)
	return a[len(a)-1]
}

func Part1(input string) {
	bin := convertHexToBin(input)
	_, ver, _ := parsePacket(bin, 0)
	fmt.Println("Version Sum:", ver)
}

func Part2(input string) {
	bin := convertHexToBin(input)
	_, _, val := parsePacket(bin, 0)
	fmt.Println("Val Expr:", val)
}
