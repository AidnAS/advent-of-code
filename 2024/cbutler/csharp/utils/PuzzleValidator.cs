using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace csharp.utils;

public class PuzzleValidator
{
	public static string Validate(int expected, int actual, string puzzleName) {
		if (expected == actual) {
			return $"{puzzleName} - PASSED \u2705 [{actual}]";
		} else {
			return $"{puzzleName} - FAILED \u274c [Expected: {expected}, Actual: {actual}]";
		}
	}
}
