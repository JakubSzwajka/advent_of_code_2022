## [Day 13](https://adventofcode.com/2022/day/13)

A few new things which was this day about:

- Parsing data from string. Used `nom` crate here. Short description below at `Packet Parser`.
- Enums and variants.
- Comparing enums and variants with `Itertools::EitherOrBoth`.

## Enums

Enums in Rust can have methods implemented for them. In Rust, an enum is a type that represents a group of related values, which are called variants. Each variant can have its own methods, and you can also define methods that apply to all variants of the enum.

To define a method on an enum, you use the same syntax as defining a method on a struct, but you add the impl keyword and the name of the enum before the method definition. Here's an example of an enum with a method implemented for it:

```
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}
```

You can then call this method on an instance of the Direction enum like this:

```
let d = Direction::North;
let opposite = d.opposite(); // opposite will be Direction::South
```

Note that the self parameter in the method definition is a reference to the enum variant on which the method is being called. This is similar to how methods on structs work in Rust.

## Packet Parser

Define an enum called Packet with two variants: List and Int. The List variant has a single field of type `Vec<Packet>`, which means it can hold a list of Packet values. The Int variant has a single field of type Int, which is an alias for i32.

The parse function is the main entry point for parsing packets, and it uses the `alt` combinator from the `nom` crate to try parsing the input string as either an integer or a list. If the input string starts with a digit, it will be parsed as an integer using the `parse_int` function. If it starts with an open square bracket [, it will be parsed as a list using the `parse_list` function.

The `parse_int` function uses the `map` combinator to convert the result of parsing a string of digits into an Int value, and then wraps it in a `Packet::Int` variant. The `parse_list` function uses the delimited and `separated_list0` combinators to parse a list of packets surrounded by square brackets and separated by commas. It then wraps the resulting list of packets in a `Packet::List` variant.

The digit1 combinator parses a string of one or more digits, and the map_res combinator applies a function to the resulting string to convert it into a numeric value. In this case, the function passed to map_res is |s: &str| s.parse(), which parses the string as a number of the type specified in the Int alias (i32 in this case).

The `IResult` type is an alias for a result type that is returned by the parsing functions. It indicates whether the parsing was successful and, if so, gives the remaining input and the parsed value.
