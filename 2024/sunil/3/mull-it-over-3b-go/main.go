package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"
)

func main() {
	// read from file
	f, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	// non-regex version
	sum := nonRegex(string(f))
	fmt.Println(sum)
}

func nonRegex(f string) int {
	sum := 0
	// find the first dont
	index := strings.Index(string(f), "don't()")
	if index != -1 {
		fmt.Printf("initial match until dont", f[:index])
		sum += findSum(f[:index])
	} else {
		index = 0
	}

	restOfText := f[index:]
	for {
		doIndex := strings.Index(restOfText, "do()")
		if doIndex == -1 {
			break
		}
		dontIndex := strings.Index(restOfText[doIndex:], "don't()")
		if dontIndex == -1 {
			break
		}
		dontIndex += doIndex
		// fmt.Printf("doIndex: %d, dontIndex: %d\n", doIndex, dontIndex)
		sum += findSum(restOfText[doIndex:dontIndex])
		restOfText = restOfText[dontIndex+7:]
	}

	return sum
}

func findSum(f string) int {
	// fmt.Println(f)
	sum := 0
	regex := regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`)
	matches := regex.FindAllString(f, -1)
	for _, match := range matches {
		mulValue := mul(match)
		fmt.Println(match, mulValue)
		sum += mul(match)
	}
	return sum
}

func mul(s string) int {
	nums := strings.Split(s[4:len(s)-1], ",")
	return toInt(nums[0]) * toInt(nums[1])
}

func toInt(s string) int {
	num := 0
	for _, r := range s {
		num = num*10 + int(r-'0')
	}
	return num
}
