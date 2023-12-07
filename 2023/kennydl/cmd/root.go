/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"os"
	"strconv"

	"advent.of.code.2023/cmd/solver/day1"
	"advent.of.code.2023/cmd/solver/day2"
	"advent.of.code.2023/cmd/solver/day3"
	"advent.of.code.2023/cmd/solver/day4"
	"advent.of.code.2023/cmd/solver/day5"
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "advent.of.code.2023",
	Short: "Advent of Code 2023",
	Long: `Advent of Code 2023 is a set of small programming puzzles for a variety of skill levels.`,
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			fmt.Println("Please provide a day number or numbers.")
			return
		}

		for i, dayValue := range args {
			day, err := strconv.Atoi(dayValue)
			if err != nil || day < 1 || day > 25 {
				fmt.Println("The input day \"" + dayValue + "\" is not valid.")
				continue
			}

			fmt.Println("Solve for day " + dayValue)
			runDay(day)
			if i < len(args) - 1 {
				fmt.Println()
			}
		}
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
}

func runDay(day int) {
	switch day {
		case 1:
			day1.Run()
		case 2:
			day2.Run()
		case 3:
			day3.Run()
		case 4:
			day4.Run()
		case 5:
			day5.Run()
		default:
			fmt.Println(" - Not solved yet! Coming soon...")
	}
}
