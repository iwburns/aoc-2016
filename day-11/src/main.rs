extern crate id_tree;
use id_tree::{Tree, Node, NodeId};

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
}
#[derive(Debug, Clone, PartialEq)]
enum ItemType {
    Microchip,
    Generator,
}
#[derive(Debug, Clone, PartialEq)]
struct Item {
    pub item_type: ItemType,
    pub element: Element,
}

#[derive(Debug, Clone)]
struct Floor {
    pub items: Vec<Item>,
}
impl Floor {
    fn add_generator(&mut self, e: Element) {
        self.items.push(Item { item_type: ItemType::Generator, element: e });
    }
    fn add_microchip(&mut self, e: Element) {
        self.items.push(Item { item_type: ItemType::Microchip, element: e });
    }
}

#[derive(Debug, Clone)]
struct State {
    pub floors: Vec<Floor>,
    pub elevator_pos: u32,
}

impl State {
    fn apply_move(&self, m: &Move) -> State {
        let mut floors = self.floors.clone();

        floors.get_mut(self.elevator_pos as usize).unwrap().items.retain(|x| {
            if *x == m.first_item {
                return false;
            }
            if let Some(second) = m.second_item.clone() {
                return *x != second;
            }
            true
        });

        floors.get_mut(m.next_floor as usize).unwrap().items.push(m.first_item.clone());
        if let Some(second) = m.second_item.clone() {
            floors.get_mut(m.next_floor as usize).unwrap().items.push(second);
        }

        State {
            floors: floors,
            elevator_pos: m.next_floor,
        }
    }
}

#[derive(Debug)]
struct Move {
    pub next_floor: u32,
    pub first_item: Item,
    pub second_item: Option<Item>,
}

fn main() {
    let mut floors: Vec<Floor> = setup_floors();
    let mut paths = Tree::<State>::new();

    let starting_state = State{ floors: floors.clone(), elevator_pos: 0 };

    let root_id = paths.set_root(Node::new(starting_state.clone()));

    for move_opt in get_possible_moves(&starting_state).iter() {

        let state = starting_state.apply_move(move_opt);

//        println!("{:?}", move_opt);
//        println!("{:?}", state);
        for move_opt_2 in get_possible_moves(&state) {

            println!("{:?}", move_opt_2);
        }
        println!("=================");

    }

    //try all possible moves and add them as children,
    //when checking children, don't progress after states where any chip is fried
    // break out as soon as we find the first child where everything is on floor 4
    // keep a depth counter as you go.

    /*
        start on floor 1, get everything to floor 4.
        must move at least one thing to move the elevator.
        cannot move more than two things.
        M must not be on a level with a different G, unless it has it's corresponding G
        get all parts up to level 4
    */
}

fn get_possible_moves(state: &State) -> Vec<Move> {
    let mut next_floor_options = Vec::new();
    if state.elevator_pos == 0 {
        // can only move up.
        next_floor_options.push(state.elevator_pos + 1);
    } else if state.elevator_pos == 3 {
        // can only move down.
        next_floor_options.push(state.elevator_pos - 1);
    } else {
        // can move up or down.
        next_floor_options.push(state.elevator_pos + 1);
        next_floor_options.push(state.elevator_pos - 1);
    }
    let next_floor_options = next_floor_options;

    let mut item_options: Vec<(Item, Option<Item>)> = Vec::new();
    let current_floor: &Floor = state.floors.get(state.elevator_pos as usize).unwrap();

    for item in current_floor.items.iter().cloned() {
        item_options.push((item.clone(), None));
    }

    for (i, item) in current_floor.items.iter().enumerate() {
        for other_item in current_floor.items.iter().skip(i + 1) {
            item_options.push( (item.clone(), Some(other_item.clone())) );
        }
    }
    let item_options = item_options;

    let mut move_options = Vec::new();

    for floor_option in next_floor_options {
        for item_option in item_options.iter().cloned() {
            let move_option = Move {
                next_floor: floor_option,
                first_item: item_option.0,
                second_item: item_option.1,
            };
            move_options.push(move_option);
        }
    }

    move_options
}

fn setup_floors() -> Vec<Floor> {
    let mut floors = Vec::new();

    for i in 0..4 {
        let mut floor = Floor {
            items: Vec::new(),
        };

        match i {
            0 => {
                floor.add_generator(Element::Thulium);
                floor.add_generator(Element::Plutonium);
                floor.add_generator(Element::Strontium);
                floor.add_microchip(Element::Thulium);
            },
            1 => {
                floor.add_microchip(Element::Plutonium);
                floor.add_microchip(Element::Strontium);
            },
            2 => {
                floor.add_generator(Element::Promethium);
                floor.add_generator(Element::Ruthenium);
                floor.add_microchip(Element::Promethium);
                floor.add_microchip(Element::Ruthenium);
            },
            3 => {}, //4th floor is empty
            _ => { unreachable!(); },
        }

        floors.push(floor);
    }

    floors
}

#[cfg(test)]
mod tests {

}