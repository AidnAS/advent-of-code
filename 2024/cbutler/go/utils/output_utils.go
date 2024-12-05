package utils

import (
	"fmt"
)

func PrintOutput(val string, useTestData bool) {
	var outputString = "Total [using puzzle data]: "

	if useTestData {
		outputString = "Total [using test data]: "
	}

	fmt.Println(outputString, val, "\n\n")
}
