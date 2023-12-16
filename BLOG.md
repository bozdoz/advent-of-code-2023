# What Am I Learning Each Day?

### Day 14

**Difficulty: -/10 ★★★★★★☆☆☆☆**

**Time: ~- hrs**

**Run Time: ~-ms**

Doing another Vec<Vec<_>> for a grid.  Learned a bit about `Cell`, but ended up not using it. It was suggested to me to try this to update a grid from within a grid iteration:

```rust
let grid_cells = Cell::from_mut(grid.as_mut_slice()).as_slice_of_cells();
for (y, row) in grid_cells.iter().enumerate() {
  for (x, val) in row.as_slice_of_cells().iter().enumerate() {
    grid_cells[y - 1][x].set(grid_cells[y - 1][x].get() + 1);
  }
}
```

It looks like a lot.

What I ended up doing was:

```rust
let height = clone.grid.len();
let width = clone.grid[0].len();
for y in 1..height {
    for x in 0..width {
        let rock = &clone.grid[y][x];
```

### Day 13

**Difficulty: -/10 ★★★★★★☆☆☆☆**

**Time: ~- hrs**

**Run Time: ~-ms**

I really liked my part one implementation but I realize it's a pain for part two. 

I transposed a grid today:

```rust
let rows = grid.len();
let cols = grid[0].len();

let mut transposed: Vec<Vec<Item>> = vec![vec![Item::Ash; rows]; cols];

for r in 0..rows  {
    for c in 0..cols {
        // Transpose the elements
        transposed[c][r] = grid[r][c];
    }
}
```

I also had a good use of `zip`:

```rust
// pass grid or transposed to check rows or cols
fn is_reflection_at(grid: &Vec<Vec<Item>>, i: usize) -> bool {
    // expand outwards from index
    let prev = grid.iter().rev().skip(grid.len() - i);
    let next = grid.iter().skip(i + 2);

    for (p, n) in prev.zip(next) {
        if p != n {
            return false
        }
    }

    true
}
```

And `windows`:

```rust
// size 2 means we look for side-by-side matches first
for (i, row) in self.grid.windows(2).enumerate() {
    if row[0] == row[1] {
        // we have one pair; extend the search to pair everything
        if Self::is_reflection_at(&self.grid, i) {
            return Reflection::Horizontal(i + 1);
        }
    }
}
```

For part two I realize it would be easier if I were comparing bits and checking if exactly 1 bit is different.

### Day 11

**Difficulty: -/10 ★★★★★★☆☆☆☆**

**Time: ~- hrs**

**Run Time: ~-ms**

Already, I'm upset to see the grid with stars.



### Day 10

**Difficulty: 6/10 ★★★★★★☆☆☆☆**

**Time: ~2 hrs**

**Run Time: ~11.5ms**

Had a few issues trying to figure out how to do something like this:

```rust
// iterate cells somehow as a mutable variable
if let Some(v) = cells.get_mut(&starting_cell) {
    // iterate cells somehow as immutable 🦀❗️
    if let Some(v2) = cells.get(&v.neighbours[0]) {
```

But I "cannot borrow `cells` as immutable because it is also borrowed as mutable".  This is something that I could unquestionably do in any other language, and I have no idea how to go about it now.

I've decided to do this, and maybe I'm better for it?

```rust
// find starting point neighbours
let mut starting_point_neighbours: Vec<Point> = vec![];
let top = starting_cell + Point { x: 0, y: -1 };
let bottom = starting_cell + Point { x: 0, y: 1 };
let left = starting_cell + Point { x: -1, y: 0 };
let right = starting_cell + Point { x: 1, y: 0 };

for neigh in [top, bottom, left, right] {
    if let Some(v2) = cells.get(&neigh) {
        if v2.neighbours.iter().any(|x| *x == starting_cell) {
            starting_point_neighbours.push(neigh);
        }
    }
}

if let Some(v) = cells.get_mut(&starting_cell) {
    v.neighbours = starting_point_neighbours
}
```

I learned about `vec.retain`:

```rust
pub fn retain<F>(&mut self, f: F)
where
    F: FnMut(&T) -> bool,

// Retains only the elements specified by the predicate
```

I had a loop for part one; for part two I realized I needed to loop again, but not for the same reason. Borrowing from the iterator I created for Day 3, I converted my loop into an iterator:

```rust
// a loop
fn walk(&self) -> usize {
    // move from starting_point
    let start = self.get_cell(&self.starting_cell);
    let mut prev = &self.starting_cell;
    let mut cur = &start.neighbours[0];
    let mut value = self.get_cell(cur);
    let mut i = 1;

    loop {
        // get neighbour that isn't previous point
        let next = value.neighbours.iter().find(|x| {
            *x != prev
        }).unwrap();

        if *next == self.starting_cell {
            return i + 1;
        }

        prev = cur;
        cur = next;
        value = self.get_cell(cur);
        
        i += 1;
    }
}
```

And here's the same code, implemented as an iterator (notice the lack of coupling with the count):

```rust
// Iterator
struct GridLoop<'a> {
    grid: &'a Grid,
    prev: Option<Point>,
    cur: Option<Point>,
}

// double lifetime
impl<'a> GridLoop<'a> {
    fn new(grid: &'a Grid) -> Self {
        GridLoop { grid, prev: None, cur: None }
    }
}

impl Iterator for GridLoop<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.grid.starting_cell;
        
        if self.prev.is_none() {
            // start with cell next to starting cell
            self.prev = Some(start);
            self.cur = Some(self.grid.get_cell(&start).neighbours[0]);

            return self.cur;
        }

        let cur = self.cur.unwrap();
        
        // finish if we reach the start
        if cur == start {
            return None;
        }

        // get next 
        let prev = self.prev.unwrap();
        let value = self.grid.get_cell(&cur);
        let next = value.neighbours.iter().find(|x| {
            // double de-reference again...
            **x != prev
        }).unwrap();

        self.prev = Some(cur);
        self.cur = Some(*next);

        self.cur
    }
}
```

And now the `walk` function can just be:

```rust
fn walk_iter(&self) -> usize {
    GridLoop::new(self).into_iter().count()
}
```

I'm realizing I need to get better at reading math.  I find it hard to recognize sum and multiples and absolute numbers in the math notation.

Oh, I also moved the Point struct to the shared lib:

```rust
use core::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

A lot of this was copied directly from the documentation; though they use generics, and I'd like to avoid public module generics as much as possible, since my frustration with using it with **go**.

Learned Shoelace Formula and Pick's Theorem.

### Day 9

**Difficulty: 1/10 ★☆☆☆☆☆☆☆☆☆**

**Time: ~40 min**

**Run Time: ~3.6ms**

Got:

```
thread 'main' panicked at day-09/src/main.rs:36:48:
is idiomatic: ParseIntError { kind: InvalidDigit }
```

Kind of funny to get that considering I read that `unwrap` is more of a TODO, and you should use `expect` instead:

This was in my parser:

```rust
fn parse(contents: &str) -> Vec<Vec<isize>> {
    contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().expect("is idiomatic"))
                .collect()
        })
        .collect()
}
```

I honestly don't really care about errors since this isn't a production app.

And:

```rust
// now get the last value of all diffs
let mut prediction = 0;

for diff in diffs.iter().rev() {
    prediction += diff.last().expect("is this what you like?");
}

prediction
```

Today I only really got tripped up by the parser (above), where I forgot the inner `.collect()`.  I also had to remember that `.rev()` is to reverse an iterable, but `.reverse()` is to reverse a vector.

Also, I'm unsure of how performant this is to remove the last item from an iterable (double rev):

```rust
// rev'ing twice to skip last
for (i, v) in cur.iter().rev().skip(1).rev().enumerate() {
```

### Day 8

**Difficulty: 3/10 ★★★☆☆☆☆☆☆☆**

**Time: ~2 hr**

**Run Time: ~5.4ms**

I totally forgot about lcm (least common multiplier), and I'm sure I've solved a puzzle like this in previous years (though I can't find my implementation).

I copied over some functions I found for implementing lcm, and part 2 seemed to work just fine.

I thought today's parsing was pretty simple:

```rust
let mut elements = HashMap::new();

for line in lines.skip(1) {
    let key = &line[..3];
    let left = &line[7..10];
    let right = &line[12..15];

    elements.insert(key, (left, right));
}
```

First time using a view(?) on a slice(?).

I was tempted to save the start_keys to the struct, but it worked a lot better to just supply that to the `count_path` function.

I find it odd to get the last character of a &str:

```rust
if elem.0.chars().nth(2).unwrap() == 'A' {
    Some(*elem.0)
} else {
    None
}
```

I also found it frustrating to determine how and when to add the `mut` keyword, in the count_path function:

```rust
fn count_path(&self, key: &str) -> usize {
    // iterate instructions, return # of iterations
    let mut key = key;
    let mut i = 0;
    let len = self.instructions.len();

    loop {
        let (l, r) = self.elements.get(key).unwrap();

        key = match self.instructions[i % len] {
            Dir::L => { l }
            Dir::R => { r }
        };

        i += 1;

        if key.chars().nth(2).unwrap() == 'Z' {
            return i;
        }
    }
}
```

That `let mut key = key;` looks awfully strange to me, but the tests pass, so I guess it's great.

Probably my first time using a `loop`.

I did successfully, finally use lifetimes in the Network struct:

```rust
struct Network<'a> {
    instructions: Vec<Dir>,
    elements: HashMap<&'a str, (&'a str, &'a str)>,
}

impl Network<'_> {
    fn new(contents: &str) -> Network<'_> {
        // ...
```

I *think* what it means is that all the `&str` references should live as long as the Network instance.

### Day 7

**Difficulty: 4/10 ★★★★☆☆☆☆☆☆**

**Time: ~2 hr**

**Run Time: ~2.3ms**

Today I thought, let's use an enum.  

I am not entirely clear what each `derive` trait is doing; I thought I had to make a custom `PartialOrd`, but turns out I can just derive it.

I tried desperately to get `self.hand == other.hand` to work, but I can't figure out how to match enum variants that hold different data; other than:

```rust
// I can't believe I have to do this
match (&self.hand, &other.hand) {
    | (CamelType::HighCard(a), CamelType::HighCard(b))
    | (CamelType::Pair(a), CamelType::Pair(b))
    | (CamelType::TwoPair(a), CamelType::TwoPair(b))
    | (CamelType::Three(a), CamelType::Three(b))
    | (CamelType::FullHouse(a), CamelType::FullHouse(b))
    | (CamelType::Four(a), CamelType::Four(b))
    | (CamelType::Five(a), CamelType::Five(b)) => {
        // check the cards (vec<u8> has `.cmp()`)
        return a.cmp(&b);
    }
}
```

That's crazy, and makes me think this was a bad idea.

**However**, enums are comparable naturally by just adding the `PartialEq` and `PartialOrd` traits, and **ordering** the enum variants in **ASC** order:

```rust
#[derive(PartialEq, PartialOrd)]
enum Hand {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn main() {
    assert!(Hand::HighCard < Hand::Pair);
    assert!(Hand::Pair == Hand::Pair);
    assert!(Hand::Three > Hand::Pair);
}
```

I think I may never use lifetimes...  I thought at first to make the hands a `&[u8]`, but that required a lifetime like `&'a [u8]`.  Anyway, a vector seems better.

Made a nice create/update hashmap:

```rust
let mut counts: HashMap<&u8, i32> = HashMap::new();

for card in hand.iter() {
    if let Some(x) = counts.get_mut(card) {
        *x += 1;
    } else {
        counts.insert(card, 1);
    }
}
```

Today's solution for me was purely sorting.  For part 2 I just swapped the joker value: 

```rust
match c {
    'A' => { 14 }
    'K' => { 13 }
    'Q' => { 12 }
    'J' => { if part == 1 { 11 } else { 1 } }
    'T' => { 10 }
    n => n.to_digit(10).unwrap() as u8,
}
```

And that's always included in the sorting comparison, due to the enum including data.

I didn't like adding the `part: u8` flag everywhere.

The logic for finding the best hand with jokers seemed simple to me: give the jokers to the card that has the highest count: if you have three of a kind, make it four.  Otherwise my categorizing logic wouldn't work:

```rust
// deduce the hand from the count grou
match counts.len() {
    1 => { CamelType::Five(hand) }
    2 => {
        if counts.values().any(|x| *x == 2) {
            return CamelType::FullHouse(hand);
        }
        CamelType::Four(hand)
    }
    3 => {
        if counts.values().any(|x| *x == 2) {
            return CamelType::TwoPair(hand);
        }
        CamelType::Three(hand)
    }
    4 => { CamelType::Pair(hand) }
    _ => { CamelType::HighCard(hand) }
}
```

The only real questions there are FullHouse/Four and TwoPair/Three, which all depend on knowing if there's a card with a count of 2.

I had way too many false positives in testing the sorting too.  It was a bummer.

### Day 6

**Difficulty: 2/10 ★★☆☆☆☆☆☆☆☆**

**Time: ~1 hr**

**Run Time: ~594ms**

I spent most of today trying to figure out the best way to parse the input.  I find this a bit intense:

```rust
let lines: Vec<_> = contents
    .lines()
    .map(|l| {
        l.split_once(":")
            .unwrap()
            .1.split_whitespace()
            .map(|x| { x.parse::<usize>().unwrap() })
            .collect()
    })
    .collect();
let time: &Vec<usize> = &lines[0];
let dist: &Vec<usize> = &lines[1];

let mut data = vec![];

for (i, t) in time.iter().enumerate() {
    data.push(Race { time: *t, dist: dist[i] });
}
```

Maybe the first time I listened to the IDE and just used `Vec<_>` to infer(?); I'm still not sure why it's necessary; it's clearly a vec, and not clearly anything else, but seems to parse fine.

I kind of wanted to use `zip` to merge the two iterables, but I can't imagine that would make it better.

For part two, the answer was again mostly todo with parsing. I just ran the same function on it afterwards.

One unknown is how I can get so many references here:

```rust
fn part_one(races: Vec<Race>) -> usize {
    races.iter().map(|r: &Race| {
        get_dist_for_hold(r.time).iter().filter(|x: &&usize| {
            **x > r.dist
        }).count()
    }).product()
}
```

`Race` is a reference, I get that, due to `.iter()`; I'm not sure how the inner `Vec<usize>` turns into a `&&usize` in the `.filter()`, or how to make the `**x` less silly.  The way I'm understanding this: it's a pointer to a pointer to a number value.

### Day 5

**Difficulty: 7/10 ★★★★★★★☆☆☆**

**Time: ~3 hr**

**Run Time: 1.3s**

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

Found it.  It was a `>=` instead of a `>`:

```diff
fn y_to_x_map(n: usize, mapping: &Mapping) -> Option<usize> {
    let (x, y, len) = *mapping;

-    if n < x || n > x + len {
+    if n < x || n >= x + len {
        return None;
    }
```

Maybe worth doing the refactor?  Had I spotted this earlier I could have done this challenge in less than an hour.  Due to waiting 30 min for brute forcing, I had to refactor to do a reverse search, which brought the time down to 1.3s.

I used a custom type today to cut down on duplication:

```rust
/** (from, to, range len) */
type Mapping = (usize, usize, usize);
```

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
