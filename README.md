# Advent-Code
I will post my solutions for [Advent of Code](https://adventofcode.com) in this repository.

## Problems
You can find my code solutions for every problem of the Advent of Code in this section.

* [Day 1](code/day1) (Solved using Go language)
	* Count the number of times a depth measurement increases from the previous measurement. (There is no measurement before the first measurement).

	* Considering a sum of a three-measurement sliding window, count the number of items the sum of measurements in this sliding window increases.

* [Day 2](code/day2) (Solved using Rust language)
	* Calculate the horizontal position and depth you would have after following the planned course. **What do you get if you multiply your final horizontal position by your final depth?**
	Consider the following instructions:
		* ```forward X``` --> Increases the horizontal position by X units.
		* ```down X``` --> **Increases** the dept by X units.
		* ```up X``` --> **Decreases** the depth by X units.
		
	* In addition to horizontal position and depth, you'll also need to track a third value, **aim**, which also starts at **0**. Calculate the position and depth you would have after following the planned course.**What do you get if you multiply your final horizontal position by your final depth?** Use the new interpretation:
		* ```forward X``` --> Does two things:
			* It increases your horizontal position by X units.
			* It increases your depth by your aim **multiplied by** X.
		* ```down X``` --> **Increases** your aim by X units.
		* ```up X``` --> **Decreases** your aim by X units.
