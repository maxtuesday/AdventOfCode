package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
)

const input_file = "input.txt"

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
	n, _ := strconv.ParseUint(b, 2, 32)
	return int(n)
}

func parsePacket(bin string, readIndex int) (index int, ver int, lit_val int) {
	var val = 0
	version := convBinToDec(bin[readIndex : readIndex+3])
	readIndex += 3
	typeID := convBinToDec(bin[readIndex : readIndex+3])
	readIndex += 3
	// fmt.Println("---- NEW PACKET ----")
	// fmt.Println("Version:", version)
	// fmt.Println("Type ID:", typeID)
	// fmt.Println("Read Index:", readIndex)

	if typeID == 4 {
		// literal value
		// fmt.Println("Lit Value packet")
		num := bin[readIndex : readIndex+5]
		num_s := bin[readIndex+1 : readIndex+5]
		readIndex += 5
		// fmt.Println("num:", num)
		for num[0] != '0' {
			// fmt.Println("num:", num)
			num = bin[readIndex : readIndex+5]
			num_s += bin[readIndex+1 : readIndex+5]
			readIndex += 5
		}
		// fmt.Println("num:", num)
		// fmt.Println("num_s:", num_s)
		val = convBinToDec(num_s)
		// fmt.Println(readIndex)
	} else {
		// operator
		// get lenTypeID
		lenTypeID := bin[readIndex]
		readIndex++
		switch lenTypeID {
		case '0':
			// next 15 bits are total len of sub-packet
			// fmt.Println("++++ Look at total len sub-packets ++++")
			L := convBinToDec(bin[readIndex : readIndex+15])
			// fmt.Println("L:", L)
			readIndex += 15
			// fmt.Println("Read Index:", readIndex)
			var n_readIndex, n_ver, _ int
			n_readIndex, n_ver, _ = parsePacket(bin, readIndex)
			version += n_ver
			// fmt.Println(n_readIndex, n_ver, l_v)
			for n_readIndex-readIndex != L {
				n_readIndex, n_ver, _ = parsePacket(bin, n_readIndex)
				version += n_ver
				// fmt.Println("Version:", n_ver)
			}
			readIndex = n_readIndex
		case '1':
			// next 11 bits are num of sub-packets following
			// fmt.Println("++++ Look at N sub-packets ++++")
			L := convBinToDec(bin[readIndex : readIndex+11])
			// fmt.Println("L:", L)
			readIndex += 11
			for i := 0; i < L; i++ {
				n_readIndex, n_ver, _ := parsePacket(bin, readIndex)
				readIndex = n_readIndex
				version += n_ver
				// fmt.Println("Version:", n_ver)
			}
		}
	}

	// fmt.Println("(Above return) Version:", version)
	return readIndex, version, val
}

func Part1(input string) {
	bin := convertHexToBin(input)
	fmt.Println(bin)
	_, ver, _ := parsePacket(bin, 0)

	fmt.Println("Version Sum:", ver)
}
