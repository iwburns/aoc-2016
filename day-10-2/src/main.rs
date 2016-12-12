use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Clone)]
struct Microchip {
    id: u32,
}
impl Microchip {
    fn new(id: u32) -> Microchip {
        Microchip {
            id: id,
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
#[derive(Debug)]
struct Bot {
    id: u32,
    pub low_chip: Option<Microchip>,
    pub high_chip: Option<Microchip>,
}
impl Bot {
    fn new(id: u32) -> Bot {
        Bot {
            id: id,
            low_chip: None,
            high_chip: None,
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn add_microchip(&mut self, chip: Microchip) {
        let low: Option<Microchip> = self.low_chip.take();
        let high: Option<Microchip> = self.high_chip.take();

        match (low, high) {
            (Some(_), Some(_)) => {
                panic!("adding microchip to robot that already has two microchips");
            },
            (None, Some(high)) => {
                if high.id() >= chip.id() {
                    self.high_chip = Some(high);
                    self.low_chip = Some(chip);
                } else {
                    self.high_chip = Some(chip);
                    self.low_chip = Some(high);
                }
            },
            (Some(low), None) => {
                if low.id() <= chip.id() {
                    self.low_chip = Some(low);
                    self.high_chip = Some(chip);
                } else {
                    self.low_chip = Some(chip);
                    self.high_chip = Some(low);
                }
            },
            (None, None) => {
                self.low_chip = Some(chip);
            },
        };
    }
}

#[derive(Debug, Clone)]
struct ValueInstruction {
    pub chip: Microchip,
    pub target_bot: u32,
}
#[derive(Copy, Clone)]
enum TargetType {
    Bot,
    Output,
}
struct BotInstruction {
    pub bot_id: u32,
    pub low_target: u32,
    pub low_target_type: TargetType,
    pub high_target: u32,
    pub high_target_type: TargetType,
}

struct Output {
    id: u32,
    microchips: Vec<Microchip>,
}
impl Output {
    fn new(id: u32) -> Output {
        Output {
            id: id,
            microchips: Vec::new(),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn add_microchip(&mut self, chip: Microchip) {
        self.microchips.push(chip);
    }

    fn get_microchips(&self) -> &Vec<Microchip> {
        &self.microchips
    }
}


fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let (val_instructions, mut bot_instructions) = get_instruction_sets(input.as_str());

    let mut bots: Vec<Option<Bot>> = Vec::new();
    let mut outputs: Vec<Option<Output>> = Vec::new();

    for instr in val_instructions {

        let bot_index = instr.target_bot as usize;

        if bots.get(bot_index).is_none() || bots.get(bot_index).unwrap().is_none() {
            add_new_bot(&mut bots, bot_index as u32);
        }

        let mut bot: &mut Bot = bots[bot_index].as_mut().expect("couldn't get bot that we just inserted");

        bot.add_microchip(instr.chip);
    }

    while bot_instructions.len() > 0 {

        let mut leftover_instructions: Vec<BotInstruction> = Vec::new();

        for instr in bot_instructions {

            let bot_id = instr.bot_id as usize;
            let low_target = instr.low_target as usize;
            let low_type = instr.low_target_type;
            let high_target = instr.high_target as usize;
            let high_type = instr.high_target_type;

            if bots.get(bot_id).is_none() || bots.get(bot_id).unwrap().is_none() {
                add_new_bot(&mut bots, bot_id as u32);
            }

            // default values, should never be used.
            let mut low_chip: Microchip = Microchip::new(1_000_000);
            let mut high_chip: Microchip = Microchip::new(1_000_000);

            let mut process_instruction = false;
            {
                let mut current_bot: &mut Bot = bots.get_mut(bot_id).unwrap().as_mut().expect("couldn't unwrap a bot that we just inserted");
                if current_bot.low_chip.is_some() && current_bot.high_chip.is_some() {

                    process_instruction = true;
                    low_chip = current_bot.low_chip.take().unwrap();
                    high_chip = current_bot.high_chip.take().unwrap();

                    if low_chip.id() == 17 && high_chip.id() == 61 {
                        println!("found it at bot_id: {}", current_bot.id());
                    }
                }
            }

            if process_instruction {
                match low_type {
                    TargetType::Output => {
                        if outputs.get(low_target).is_none() || outputs.get(low_target).unwrap().is_none() {
                            add_new_output(&mut outputs, low_target as u32);
                        }
                        outputs[low_target].as_mut().unwrap().add_microchip(low_chip);
                    },
                    TargetType::Bot => {
                        if bots.get(low_target).is_none() || bots.get(low_target).unwrap().is_none() {
                            add_new_bot(&mut bots, low_target as u32);
                        }
                        bots[low_target].as_mut().unwrap().add_microchip(low_chip);
                    },
                };
                match high_type {
                    TargetType::Output => {
                        if outputs.get(high_target).is_none() || outputs.get(high_target).unwrap().is_none() {
                            add_new_output(&mut outputs, high_target as u32);
                        }
                        outputs[high_target].as_mut().unwrap().add_microchip(high_chip);
                    },
                    TargetType::Bot => {
                        if bots.get(high_target).is_none() || bots.get(high_target).unwrap().is_none() {
                            add_new_bot(&mut bots, high_target as u32);
                        }
                        bots[high_target].as_mut().unwrap().add_microchip(high_chip);
                    },
                };
            } else {
                leftover_instructions.push(instr);
            }
        }

        bot_instructions = leftover_instructions;
    }

    let val_0 = outputs.get(0).unwrap().as_ref().unwrap().get_microchips().get(0).unwrap().id();
    let val_1 = outputs.get(1).unwrap().as_ref().unwrap().get_microchips().get(0).unwrap().id();
    let val_2 = outputs.get(2).unwrap().as_ref().unwrap().get_microchips().get(0).unwrap().id();

    println!("multiplied: {}", val_0 * val_1 * val_2);
}

fn add_new_output(outputs: &mut Vec<Option<Output>>, new_id: u32) {
    if (new_id + 1) > outputs.len() as u32 {
        let reserve_amt = (new_id + 1) - (outputs.len() as u32);

        for _ in 0..reserve_amt {
            outputs.push(None);
        }
    }
    let output = Output::new(new_id);
    outputs[new_id as usize] = Some(output);
}

fn add_new_bot(bots: &mut Vec<Option<Bot>>, new_id: u32) {
    if new_id + 1 > bots.len() as u32 {
        let reserve_amt = (new_id + 1) - (bots.len() as u32);

        for _ in 0..reserve_amt {
            bots.push(None);
        }
    }
    let bot = Bot::new(new_id);
    bots[new_id as usize] = Some(bot);
}

fn get_instruction_sets(string: &str) -> (Vec<ValueInstruction>, Vec<BotInstruction>) {
    let mut value_instructions = Vec::new();
    let mut bot_instructions = Vec::new();

    for line in string.lines() {
        let line_parts: Vec<&str> = line.split_whitespace().collect();

        match line_parts[0] {
            "value" => {
                let val: u32 = line_parts[1].parse().expect("error parsing instructions, couldn't parse a u32");
                let target_bot: u32 = line_parts[5].parse().expect("error parsing instructions, couldn't parse a u32");

                let instruction = ValueInstruction {
                    chip: Microchip::new(val),
                    target_bot: target_bot
                };

                value_instructions.push(instruction);
            },
            "bot" => {
                let id: u32 = line_parts[1].parse().expect("error parsing instructions, couldn't parse a u32");
                let low_target: u32 = line_parts[6].parse().expect("error parsing instructions, couldn't parse a u32");
                let low_target_type = match line_parts[5] {
                    "output" => TargetType::Output,
                    "bot" => TargetType::Bot,
                    _ => { unreachable!(); }
                };
                let high_target: u32 = line_parts[11].parse().expect("error parsing instructions, couldn't parse a u32");
                let high_target_type = match line_parts[10] {
                    "output" => TargetType::Output,
                    "bot" => TargetType::Bot,
                    _ => { unreachable!(); }
                };

                let instruction = BotInstruction {
                    bot_id: id,
                    low_target: low_target,
                    low_target_type: low_target_type,
                    high_target: high_target,
                    high_target_type: high_target_type,
                };

                bot_instructions.push(instruction);
            },
            _ => { unreachable!(); }
        }
    }

        (value_instructions, bot_instructions)
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = try!(File::open("../input/day-10.txt"));
    let mut input_string = String::new();

    try!(input_file.read_to_string(&mut input_string));

    Ok(input_string)
}

#[cfg(test)]
mod tests {
    use super::add_new_bot;
    use super::add_new_output;
    use super::Bot;
    use super::Microchip;
    use super::Output;

    #[test]
    fn test_add_new_bot() {
        let mut bots: Vec<Option<Bot>> = Vec::new();

        add_new_bot(&mut bots, 2);
        assert_eq!(bots.len(), 3);

        add_new_bot(&mut bots, 0); //shouldn't change length
        assert_eq!(bots.len(), 3);

        assert_eq!(bots[2].as_ref().unwrap().id(), 2);

        add_new_bot(&mut bots, 10);
        assert_eq!(bots.len(), 11);
    }

    #[test]
    fn test_add_new_output() {
        let mut outputs: Vec<Option<Output>> = Vec::new();

        add_new_output(&mut outputs, 2);
        assert_eq!(outputs.len(), 3);

        add_new_output(&mut outputs, 0); //shouldn't change length
        assert_eq!(outputs.len(), 3);

        add_new_output(&mut outputs, 10);
        assert_eq!(outputs.len(), 11);
    }

    #[test]
    fn test_add_new_microchip() {
        let mut bot: Bot = Bot::new(0);

        let chip_1: Microchip = Microchip::new(1);
        let chip_3: Microchip = Microchip::new(3);

        bot.add_microchip(chip_1.clone());
        bot.add_microchip(chip_3.clone());

        assert_eq!(bot.low_chip.unwrap().id(), chip_1.id());
        assert_eq!(bot.high_chip.unwrap().id(), chip_3.id());
    }

}