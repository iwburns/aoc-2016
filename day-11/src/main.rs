extern crate id_tree;
use id_tree::{Tree, Node, NodeId};


enum Element {
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
}
enum ItemType {
    Microchip,
    Generator,
}
#[derive(Clone)]
struct Item {
    pub item_type: ItemType,
    pub element: Element,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct State {
    pub floors: Vec<Floor>,
    pub elevator: u32,
}



fn main() {
    let mut floors: Vec<Floor> = setup_floors();
    let mut paths = Tree::<State>::new();

    let starting_state = State{ floors: floors.clone(), elevator: 0 };

    let root_id = paths.set_root(starting_state.clone());

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