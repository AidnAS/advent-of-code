


def print_output(val: str, use_test_data: bool = False):
	outputString = ""
	if use_test_data:
		outputString = "Total [using test data]"
	else:
		outputString = "Total [using puzzle data]"
	
	print(f"{outputString}: {val}")