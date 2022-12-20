## [Day 11](https://adventofcode.com/2022/day/11)

Part one went very quick. Test data for part II also. But when doing 10000 rounds, numbers started to get to big to find the solution in reasonable time.

When looking for tips, I found [this repo](https://github.com/romamik/aoc2022) which helped me a ton.

LCM (least common multiple) for given set of numbers is the smallest number which is divisible by all the numbers in the set.

Based on that set we can define the range in which we can find this number as: from biggest number in the set to product on this set.

I.e. consider set(23, 19, 13, 17). LCM for this set is somewhere between 23 and 96577.

> since we know the input data (tests that monkey do), we can define this set as (23, 19, 13, 17)

With this info, we don't have to store item as it's value after all monkey inspections. To have a proper monkey test result we need `item = item % LCM(23, 19, 13, 17)`.

Consider adding this after `operaion` and before `Test`.

```
Monkey 0:
    Starting items: 79, 98
    Operation: new = old \* 19
    --> new = new % 96577 <---
    Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
```
