# What Am I Learning Each Day?

### Day 5

**Difficulty: ?/10 ★★★★★★★★☆☆☆**

**Time: ~3 hr**

**Run Time: ?µs**

For part two I was just curious to try to brute force; though, in ~30 min I somehow kept getting `0` as the answer.  Thought maybe my usize was overflowing; really not sure where I'm going wrong.

Ignoring `0`, I keep getting the same answer.

I refactored to do a reverse search, starting as low as `0` (which I know is not the correct answer), and I'm still getting the exact same answer (but it's incorrect).

```rust
fn part_two(almanac: &Almanac) -> usize {
    let smallest = usize::MAX;

    for l in 0usize.. {
        // just go crazy
        if almanac.location_to_seed(l).is_some() {
            return l;
        }
    }

    smallest
}
```

I just literally made copies of the original functions and reversed them; adding some small logic to check if the final seed is within the original range:

```rust
for seed in self.seeds.chunks(2) {
    if (seed[0]..seed[0]+seed[1]).contains(&cur) {
        return Some(cur)
    }
}

None
```

But still nada.

### Day 4

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: ~30 min**

**Run Time: 810.604µs**

Tried using `reduce`, but intellisense suggested `fold` instead.

I was originally trying to get the score of each card:

```rust
// collect points; fold is basically reduce
let points = want.iter().fold(0, |acc, n| {
    if have.contains(n) {
        if acc == 0 {
            return 1;
        }
        return acc * 2;
    }
    acc
});

Card { want, have, points }
```

But this was much easier to just count the matches, and call a `pow` function later; also I didn't need to keep track of the numbers at all:

```rust
let matches = want.iter().filter(|n| {
    have.contains(n)
}).count();

Card { matches }
```

And then part one:

```rust
fn part_one(cards: &Vec<Card>) -> usize {
    cards.iter().map(|c| {
        if c.matches == 0 {
            return 0;
        }

        usize::pow(2, (c.matches - 1) as u32)
    }).sum()
}
```

I was surprised to learn how to use a `pow` function: somehow associated with the number type.

I also tried to avoid using for loops, and instead just use vector or iterable methods. I still find this much more difficult to read, but seems easier to do.

Part 2 seemed easy to refactor; I made a vec to match the cards vec of just a count, which started at 1:

```rust
let mut counts: Vec<usize> = cards.iter().map(|_| 1).collect();
```

Then just updated the counts in a for loop:

```rust
for (i, card) in cards.iter().enumerate() {
    let j = if card.matches > len {
        len
    } else {
        card.matches + i
    };

    for k in i+1..j+1 {
        counts[k] += counts[i];
    }
}
```

I found out yesterday that you can get the actual reference to the vec items with `for card in cards`, which can be a problem in rust, since it would `move` the ownership or consume the value; however, you can convert the vec into an iterable, which returns a reference, and borrows the value (instead of move): `for card in cards.iter()`.  This is similar to go, when using a `range`:

```go
// card is reference
for i, card := range cards {
    // actual card
    actual = cards[i];
}
```

I'm using `include_str!` now for the inputs, which makes it easier to do TDD, and separate the example inputs from the actual inputs:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let cards = get_cards(EXAMPLE);
        
        let ans = part_one(&cards);

        assert_eq!(ans, 13);
    }
}
```

Today seemed incredibly easy, and I'm a bit surprised that I haven't run into any ownership issues since Day 1.  Maybe I'm getting the hang of borrowing, and iterating. I didn't even need a `Card` struct; could have just been `Vec<usize>`

### Day 3

**Difficulty: 4/10 ★★★★☆☆☆☆☆☆**

**Time: 2 hrs**

**Run Time: 2.357162ms**

I can't stop the warnings about `dead_code` in tests.  No clue how to remove them.  Also, I need to get a test to fail in order to see print or dbg calls.  Kind of annoying.

I wanted to dedicate/invest some time into getting a good grid struct with methods.  I'm not sure I accomplished that.  I did come up with what I thought was a good neighbouring cell algorithm:

```rust
struct Neighbours {
    start: Cell,
    end: Cell,
    cur: Option<Cell>,
}

impl Neighbours {
    // use Neighbours::new to avoid passing a `cur` value
    fn new(start: Cell, end: Cell) -> Self {
        Neighbours {
            // get top-left of this cell
            start: Cell { x: start.x - 1, y: start.y - 1 },
            // get bottom-right of this cell
            end: Cell {
                x: end.x + 1,
                y: end.y + 1,
            },
            // start at None to indicate first iteration
            cur: None,
        }
    }
}

impl Iterator for Neighbours {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        // left -> right, top -> bottom
        if self.cur.is_none() {
            // start top-left!
            self.cur = Some(self.start);

            return self.cur;
        }

        // current is definitely defined
        let mut cur = self.cur.unwrap();

        // we're done if we reach the end
        if cur == self.end {
            return None;
        }

        // increase x
        cur.x += 1;

        // check if we hit the end of the row
        if cur.x > self.end.x {
            // wrap to next line
            cur.y += 1;
            cur.x = self.start.x;
        // check if we're in the middle cells (not boundary)
        } else if
            cur.x != self.start.x &&
            cur.x != self.end.y &&
            cur.y != self.start.y &&
            cur.y != self.end.y
        {
            // inside boundary
            cur.x = self.end.x;
        }

        self.cur = Some(cur);

        return self.cur;
    }
}
```

first time implementing `Iterator`.  Also I wanted to try implementing `FromStr`, which I did to convert the input string into a grid.

```rust
// so I can use .parse() on a string
impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
```

Felt a bit weird.  Not sure if I like it.  Might have just preferred a `new` function.

I felt like I nested way too much today.

I also ran into an issue for part one where I didn't account for numbers at the end of the lines.  Needed to add some logic to check if we're done of the row:

```rust
if next.is_digit(10) {
    num =
        num * 10 + (next.to_digit(10).unwrap() as i32);

    // Doh! check if we're done the loop
    if x != max {
        continue;
    } else {
        // ignore the next line
        char = '.';
    }
} else {
    char = next;
}
```

For part 2, I was able to just add a new variant to my cellvalue enum:

```rust
enum CellValue {
    Symbol(char),
    /** number, len */
    Number(i32, isize),
    /** end of a number points to the beginning (part two) */
    Pointer(Cell),
}
```

With the Pointer, I was able to reference a single cell, when the `*` symbol touched an end digit, like:

```sh
...
..*
123
```

The `*` touches 2 and 3, but I counted the cell value at the position of `1` (with a `len` of `3`).

Wasn't too much work, but maybe it made for terrible code.

For example, this is way too nested:

```rust
// iterate '*' signs and get those neighbours
for (cell, value) in grid.cells.iter() {
    if let CellValue::Symbol(sym) = value {
        if sym == &'*' {
            // get neighbouring numbers
            // but avoid adding the same cell twice
            let mut parts: HashMap<Cell, usize> = HashMap::new();

            let start = *cell;
            let end = *cell;
            let neighbours = Neighbours::new(start, end);

            for neigh in neighbours {
                if let Some(n) = grid.cells.get(&neigh) {
                    match n {
                        CellValue::Number(v, _) => {
                            parts.insert(neigh, *v as usize);
                        }
                        CellValue::Pointer(c) => {
                            let p = grid.cells.get(c).unwrap();

                            // how deep are we here?
                            if let CellValue::Number(v, _) = p {
```

### Day 2

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: 1 hr**

**Run Time: 313.363µs**

First time using `#[derive(Default)]`.  `Go` is nice for having this by default; felt like I wanted to use it for initializing a Game object:

```rust
#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

let game = Game::default();
```

Rust does have a better way to debug these structs, however, than `go`; by just adding `#[derive(Debug)]`:

```rust
#[derive(Default, Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

dbg!(Set::default());
```

First time using `panic`?  First time using `match`.  First time using `struct`.  Found out that panic accepts a format string:

```rust
n => panic!("not a color: {}", n)
```

I'm trying to avoid using `collect()`, because I feel like it's wasteful to convert to/from iterables so often:

```rust
let info: &str = line.split(": ").nth(1).unwrap();
```

I really need to learn all of the map methods for iterables, because I'm also probably unnecessarily doing far too many `for` loops.  It's day 2 and I'm already using a loop label to break out of nested loops.

```rust
'outer: for i in 0..games.len() {
    continue 'outer;    
}
```

I should be able to do a `reduce`, or a `filter_map` and `sum`.

Didn't need ChatGPT or stackoverflow today.  Far too easy of a problem.  And, rust analyzer seems good enough at this point to help me debug errors.

### Day 1

**Difficulty: 4/10 ★★★★☆☆☆☆☆☆**

**Time: 1.5 hrs**

**Run Time: 286ms**

Part one I did by creating an array, then pushing to it from another array; breaking a rule I usually have where you should just set array **B** to a map result of array **A**. I probably did this because this is my first rust script, and I still have no idea what I'm doing.

Rust analyzer and ChatGPT are helping me understand many of the errors.

I made a typical, foolish mistake of typing the numbers as `u8` originally instead of `u16` or even just `usize`, thinking it would matter at all to be concerned about optimizing this script.

TIL about `RUST_BACKTRACE=1` as I immediately ran into overflow issues with using a `u8` for summing.

Had to add a library already. 🤦‍♂️

I added the regex library to match a string value of number:

```rust
let re: Regex = Regex::new(
    r"(one|two|three|four|five|six|seven|eight|nine|\d)"
).unwrap();
```

Though maybe this is what slowed down part two so intensely:

```sh
Part one: 56108 145.58µs
Part two: 55652 285.793102ms
```

Got to use my first `Some` destructuring with an `if let`:

```rust
if let Some(first) = re.find(s) { ... }
```

I had tried doing a `replace_all`, but it was much more difficult, and included rust's [Cow enum](https://doc.rust-lang.org/std/borrow/enum.Cow.html)

I wrote my first test, and found out that rust's test output is awful:

```sh
running 1 test
test tests::test_number_parser ... FAILED

failures:

---- tests::test_number_parser stdout ----
"oneight" "1ight"
thread 'tests::test_number_parser' panicked at src/main.rs:108:9:
assertion `left == right` failed
  left: 11
 right: 18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_number_parser

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin day-01`
```

All I wanted there was `11 is not 18`; or something like that. Just far too verbose.

So one of the big issues I ran into with `replace_all` was dealing with overlapping matches, where `replace_all` states: 

> Replaces all **non-overlapping** matches in the haystack with the replacement provided. This is the same as calling replacen with limit set to 0.

A better method should have been to just iterate from the beginning until I find a number, then iterate from the end until I find a number. And stop checking the string afterward.

I ran into more issues with overflow, trying to use a `while` loop:

```rust
// get last match
let mut i: usize = s.len() - 1;
while 0 <= i {
```

I got: 

> comparison is useless due to type limits `#[warn(unused_comparisons)]` on by default

I could change the type of `usize`, or just use a `for` loop over a range instead:

```rust
// get last match
for i in (0..=s.len() - 1).rev() {
    if let Some(last) = re.find(&s[i..]) {
        two[1] = replace_numbers(last.as_str());
        break;
    }
}
```

Also a big issue was not being able to iterate over a `&Vec<&str>`, to return a `Vec<&str>`.  I had to create a `Vec<String>` first, then iterate again and map again to a `&str`; then call `collect()` to get a `Vec<&str>`.  Otherwise, the `map` variable was considered a temporary variable, and I couldn't save a reference to a temporary variable.  So, I'm doing something that's not idiomatic.

I made a boiler plate to selectively run parts.  And I might update it to add the timings.

Overall, it was a fine day, but a little over-complicated for day 1.

I think rust may have a better built-in file parser than `go`.  fs read_to_string with a `.lines()` just seems simple.  Minor downside to also have to add `.expect()` because it could be an `Error`, and `.collect()`, because it just seems more difficult to borrow iterables.
