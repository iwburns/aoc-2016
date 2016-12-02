enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

struct Santa {
    pub x: i32,
    pub y: i32,
    pub direction: Direction
}

impl Santa {
    fn move_forward(&mut self, amount: i32) {
        match self.direction {
            Direction::NORTH => self.y += amount,
            Direction::SOUTH => self.y -= amount,
            Direction::EAST => self.x += amount,
            Direction::WEST => self.x -= amount,
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::NORTH => self.direction = Direction::EAST,
            Direction::SOUTH => self.direction = Direction::WEST,
            Direction::EAST => self.direction = Direction::SOUTH,
            Direction::WEST => self.direction = Direction::NORTH,
        }
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::NORTH => self.direction = Direction::WEST,
            Direction::SOUTH => self.direction = Direction::EAST,
            Direction::EAST => self.direction = Direction::NORTH,
            Direction::WEST => self.direction = Direction::SOUTH,
        }
    }
}

fn main() {
    let input = "L2, L5, L5, R5, L2, L4, R1, R1, L4, R2, R1, L1, L4, R1, L4, L4, R5, R3, R1, L1, R1, L5, L1, R5, L4, R2, L5, L3, L3, R3, L3, R4, R4, L2, L5, R1, R2, L2, L1, R3, R4, L193, R3, L5, R45, L1, R4, R79, L5, L5, R5, R1, L4, R3, R3, L4, R185, L5, L3, L1, R5, L2, R1, R3, R2, L3, L4, L2, R2, L3, L2, L2, L3, L5, R3, R4, L5, R1, R2, L2, R4, R3, L4, L3, L1, R3, R2, R1, R1, L3, R4, L5, R2, R1, R3, L3, L2, L2, R2, R1, R2, R3, L3, L3, R4, L4, R4, R4, R4, L3, L1, L2, R5, R2, R2, R2, L4, L3, L4, R4, L5, L4, R2, L4, L4, R4, R1, R5, L2, L4, L5, L3, L2, L4, L4, R3, L3, L4, R1, L2, R3, L2, R1, R2, R5, L4, L2, L1, L3, R2, R3, L2, L1, L5, L2, L1, R4";
    let input = input.split(", ");

    let mut santa = Santa {
        x: 0i32,
        y: 0i32,
        direction: Direction::NORTH
    };

    for instruction in input {

        let turn: String = instruction.chars()
            .take(1)
            .collect();

        let move_amt: String = instruction.chars()
            .skip(1)
            .take(instruction.len() - 1)
            .collect();

        let move_amt: i32 = move_amt.parse().unwrap_or(0i32);

        match turn.as_str() {
            "L" => santa.turn_left(),
            "R" => santa.turn_right(),
            _ => {}
        };

        santa.move_forward(move_amt);
    }
    println!("Total distance: {}", santa.x.abs() + santa.y.abs());
}
