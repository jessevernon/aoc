use std::fs;

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn main() {
    let moves = parse_moves();
    let mut stacks = parse_stacks();

    // reverse all the stacks
    for stack in &mut stacks {
        stack.reverse();
    }

    for move_ in moves {
        println!("");
        println!("Move {} from {} to {}", move_.quantity, move_.from, move_.to);

        let mut stack = stacks[move_.from - 1].clone();
        let mut stack2 = stacks[move_.to - 1].clone();

        for _ in 0..move_.quantity {
            stack2.push(stack.pop().unwrap());
        }

        stacks[move_.from - 1] = stack;
        stacks[move_.to - 1] = stack2;

        let mut i = 1;
        for stack in &stacks {
            print!("{}: ", i);
            i += 1;
            for c in stack {
                print!("{} ", c);
            }
            println!("");
        }
    }

    // print out top of each stack
    println!("");
    for stack in &stacks {
        print!("{}", stack[stack.len() - 1]);
    }
}

fn parse_stacks() -> Vec<Vec<char>> {
    // read file moves
    let contents = fs::read_to_string("stacks").expect("Failed to read file");

    // split into lines
    let mut lines = contents.lines();

    // parse moves
    let mut stacks: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let mut i = 0;
        let mut col = 0;

        // format is [A-Z] or "   "
        for c in line.chars() {
            if (i % 4) == 0 {
                col += 1;

                if stacks.len() < col {
                    stacks.push(Vec::new());
                }
            }

            // see if char is between A and Z
            if c >= 'A' && c <= 'Z' {
                stacks[col - 1].push(c);
            }

            i += 1;
        }
    }

    // print moves
    /*println!("Stacks:");
    for stack in &stacks {
        for c in stack {
            print!("{} ", c);
        }
        println!("");
    }*/

    return stacks;
}

fn parse_moves() -> Vec<Move> {
    // read file moves
    let contents = fs::read_to_string("moves").expect("Failed to read file");

    // split into lines
    let mut lines = contents.lines();

    // parse moves
    let mut moves: Vec<Move> = Vec::new();

    while let Some(line) = lines.next() {
        // format is moves [quantity] from [from] to [to]
        let mut parts = line.split(" ");
        let quantity = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
        let to = parts.nth(1).unwrap().parse::<usize>().unwrap();

        moves.push(Move {
            quantity: quantity,
            from: from,
            to: to,
        });
    }

    // print moves
    /*println!("Moves:");
    for move_ in &moves {
        println!("Move {} from {} to {}", move_.quantity, move_.from, move_.to);
    }*/

    return moves;
}
