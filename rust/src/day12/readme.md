## [Day 12](https://adventofcode.com/2022/day/12)

This day was about pathfinding algorithm. After quick research I decided to go with [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) implementation.

## Rules

As an input we have a map and a few information about it:

- `S` is a starting point with height eq `a`.
- `E` is a target point with height eq `z`.
- Every other point is represented by its height where `a` is the lowest one and `Z` is the highest.

```
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
```

Rules of moving:

- We can go only up, down, left or right.
- We can go only one height unit up.
- We can go as many height units down as we want.

First thing to note is that we can change letter height representation from `char` to `usize` based on [ASCII table](https://pl.wikipedia.org/wiki/ASCII).

> In ASCII, capital letters are first ( a = 97 and A = 65 ). This means that if we want to keep proper order, we need to decrease every letter above 97 by 58. This way z = 64 and A = 65 ).

## Part I and II difference

Difference between Part I and Part II is that in part II we can have a multiple starting points based on heigh and we need to find the shortest path. In Part I the starting point is declared as the one with `S`.

To make solution more generic I'm injecting the condition which decides if given point can be a starting point.

Then count the shortest path for every point and return the shortest one from all the points. In Part one, `starting_points` array will be a single element array.

## Dijkstra's algorithm

Implementation was quite straithforward since the distance to next point was always 1.

- Every point has a distance propery which initially should be declared as infinity. I declared it as 1 since I will overwrite it later.
- Declare `spt` (shortest path three) vector. This vector will be holding the next points to be checked. On the beginit there is starting point only in it.

while any on `next_squares` is not a `target_square`:

1. Sort `spt` by distance.
2. `current_square` is the first element from `spt` (the one with the smallest distance)
3. Mark `current_square` as `visited=true`.
4. Get all possible `next_squares` from `current_square` (left, right, up, down) but not those that has already been visited. Here you can implement the rules about walls etc.
5. Update distance of each from `next_squares`, as `current_square.distance + 1`.
6. Add them to `spt`

> If two first elements in spt have the same distance, we can choose any on them.

## Note on structs

Quick explanation on structs and dependencies. Why is code devided this way not another.

- From input we construct the `Map`.
- When creating `Traveller` we give him a `Map` since every traveller needs to have one.
- The `Map` is a matrix of points along which `Traveller can move.
- `Traveller` is taking some notes on points when travelling. Do I visited this point? How far is it from my starting point.
- `Map` also has some additional info. When traveller looks at the map, based on his rules, he can mark some possible starting points on the map. Those points are then marked as `starting_point=true`
