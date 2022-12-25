## [Day 14](https://adventofcode.com/2022/day/14)

The trick in part II was to filter out as most candidates to check as I can. Since we can assume that there is only one beacon to be found, we can assume that it is right next to some area. To obtain candidates, for each sensor I took points that are r (radius) + 1 distance from sensor center. Then filter out those below 0 and above 400000. Those were candidates to be checked.
