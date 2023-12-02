#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bag {
    num_blue: u64,
    num_red: u64,
    num_green: u64,
}

struct BagBuilder {
    num_blue: Option<u64>,
    num_red: Option<u64>,
    num_green: Option<u64>,
}

impl BagBuilder {
    fn new() -> Self {
        Self {
            num_blue: None,
            num_red: None,
            num_green: None,
        }
    }
    fn set_blue(self, num: u64) -> Result<Self, Error> {
        if self.num_blue.is_none() {
            Ok(BagBuilder {
                num_blue: Some(num),
                ..self
            })
        } else {
            Err(Error::MultipleColorsSameBagError)
        }
    }

    fn set_red(self, num: u64) -> Result<Self, Error> {
        if self.num_red.is_none() {
            Ok(BagBuilder {
                num_red: Some(num),
                ..self
            })
        } else {
            Err(Error::MultipleColorsSameBagError)
        }
    }

    fn set_green(self, num: u64) -> Result<Self, Error> {
        if self.num_green.is_none() {
            Ok(BagBuilder {
                num_green: Some(num),
                ..self
            })
        } else {
            Err(Error::MultipleColorsSameBagError)
        }
    }

    fn build(self) -> Bag {
        let num_blue = self.num_blue.unwrap_or(0);
        let num_red = self.num_red.unwrap_or(0);
        let num_green = self.num_green.unwrap_or(0);
        Bag {
            num_blue,
            num_red,
            num_green,
        }
    }
}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    bags: Vec<Bag>,
}

#[derive(Clone, Debug)]
enum Error {
    PartitionError(String),
    ParseGameIDError,
    MultipleColorsSameBagError,
    ParseColorNameError,
    ParseColorNumError,
}

impl Bag {
    fn can_contain(&self, other: &Bag) -> bool {
        self.num_blue >= other.num_blue
            && self.num_red >= other.num_red
            && self.num_green >= other.num_green
    }
    fn power(&self) -> u64 {
        self.num_red * self.num_green * self.num_blue
    }
}

fn split_into_two<'a, 'b>(input: &'a str, pattern: &'b str) -> Result<(&'a str, &'a str), Error> {
    let mut iter = input.split(pattern);
    if let Some(left) = iter.next() {
        if let Some(right) = iter.next() {
            if None == iter.next() {
                return Ok((left, right));
            }
        }
    }
    Err(Error::PartitionError(format!(
        "Cannot partition {} on the pattern {}.",
        input.to_string(),
        pattern.to_string()
    )))
}

fn parse_game(input: &str) -> Result<Game, Error> {
    let (game, bags) = split_into_two(input, ": ")?;
    // Parsing the game ID
    let (_, id) = split_into_two(game, " ")?;
    let id: usize = str::parse(id).map_err(|_| Error::ParseGameIDError)?;
    let mut parsed_bags = Vec::new();
    for unparsed_bag in bags.split("; ") {
        let mut bag = BagBuilder::new();
        for numbered_color in unparsed_bag.split(", ") {
            let (num, color) = split_into_two(numbered_color, " ")?;
            let num = str::parse(num).map_err(|_| Error::ParseColorNumError)?;
            match color {
                "red" => bag = bag.set_red(num)?,
                "green" => bag = bag.set_green(num)?,
                "blue" => bag = bag.set_blue(num)?,
                _ => return Err(Error::ParseColorNameError),
            }
        }
        parsed_bags.push(bag.build());
    }
    Ok(Game {
        id,
        bags: parsed_bags,
    })
}

fn sum_valid_ids(input: &[Game], master_bag: &Bag) -> usize {
    let mut sum = 0;
    for game in input.iter() {
        if game.bags.iter().all(|b| master_bag.can_contain(b)) {
            sum += game.id;
        }
    }
    sum
}

fn smallest_containing_bag(input: &Game) -> Result<Bag, Error> {
    let output = BagBuilder::new();
    let mut num_red = 0;
    let mut num_blue = 0;
    let mut num_green = 0;
    for bag in input.bags.iter() {
        num_red = u64::max(num_red, bag.num_red);
        num_blue = u64::max(num_blue, bag.num_blue);
        num_green = u64::max(num_green, bag.num_green);
    }
    let output = output.set_red(num_red)?;
    let output = output.set_blue(num_blue)?;
    let output = output.set_green(num_green)?;
    Ok(output.build())
}

fn main() {
    // Part 1
    let master_bag = Bag {
        num_red: 12,
        num_green: 13,
        num_blue: 14,
    };
    let input = std::fs::read_to_string("src/input.txt").expect("Input file not found");
    let games: Vec<Game> = input
        .lines()
        .map(|unparsed_game| parse_game(unparsed_game).unwrap())
        .collect();
    let sump1 = sum_valid_ids(&games, &master_bag);
    println!("Part 1: {}", sump1);
    let sump2: u64 = games
        .iter()
        .map(|g| smallest_containing_bag(g).unwrap().power())
        .sum();
    println!("Part 2: {}", sump2);
}
