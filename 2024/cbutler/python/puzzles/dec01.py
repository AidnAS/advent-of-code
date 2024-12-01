from typing import List
from utils.file_utils import data_file_reader



def solve_part1(date: str, use_test_data: bool = False):
	print("Day 01, Part 1")

	lines = data_file_reader(date = date, use_test_data = use_test_data, part = 1)

	#  create two lists of Ids
	firstIds: List[int] = []
	secondIds: List[int] = []

	for line in lines:
		parts = line.split()
		firstIds.append(int(parts[0]))
		secondIds.append(int(parts[1]))

	# sort both lists
	firstIds.sort()
	secondIds.sort()

	# print('First Ids')
	# for id in firstIds:
	# 	print(id)
	
	# print('Second Ids')
	# for id in secondIds:
	# 	print(id)

	if (len(firstIds) != len(secondIds)):
		print('Lists are not the same size')
		return
	
	# compare the two lists
	diffs: List[int] = []

	for i in range(len(firstIds)):
		if (firstIds[i] >= secondIds[i]):
			diffs.append(firstIds[i] - secondIds[i])
		else:
			diffs.append(secondIds[i] - firstIds[i])
	
	# print the differences
	# print('Differences')
	# for diff in diffs:
	# 	print(diff)

	# count the total
	total: int = 0

	for i in range(len(diffs)):
		total += diffs[i]
	
	outputString = ""
	if use_test_data:
		outputString = "Total [using test data]"
	else:
		outputString = "Total [using puzzle data]"
	
	print(f"{outputString}: {total}")



def solve_part2(date, use_test_data = False):
	print("Day 01, Part 2")

	lines = data_file_reader(date = date, use_test_data = use_test_data, part = 1)

	#  create two lists of Ids
	firstIds: List[int] = []
	secondIds: List[int] = []

	for line in lines:
		parts = line.split()
		firstIds.append(int(parts[0]))
		secondIds.append(int(parts[1]))

	if (len(firstIds) != len(secondIds)):
		print('Lists are not the same size')
		return

	# find the similarity scores for each id in the first list
	scores: List[int] = []

	for i in range(len(firstIds)):
		count = 0
		for j in range(len(secondIds)):
			if (firstIds[i] == secondIds[j]):
				count += 1
		scores.append(firstIds[i] * count)
	
	# count the total
	total: int = 0

	for i in range(len(scores)):
		total += scores[i]
	
	outputString = ""
	if use_test_data:
		outputString = "Total [using test data]"
	else:
		outputString = "Total [using puzzle data]"
	
	print(f"{outputString}: {total}")