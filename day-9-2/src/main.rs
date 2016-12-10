use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Marker {
    pub length: u64,
    pub repetitions: u64,
}

fn main() {
    let input = get_input_string().unwrap_or(String::new());

    let sum = calc_num_characters(input);

    println!("{}", sum);

//    let mut current_string = input;
//    let mut sum: u64 = 0;
//    loop {
//
//
//
//        if let (Some(start), Some(end)) = find_next_marker(current_string.as_str()) {
//
//            let marker = parse_marker(current_string.as_str(), start, end);
//            let territory_end = end + 1 + marker.length as usize;
//
//            let full_marker_string = get_sub_string(current_string.as_str(), start, territory_end).to_string();
//
//            println!("full_marker_string: {}", full_marker_string);
//
//            sum += calc_num_characters(full_marker_string);
//            println!("sum: {}", sum);
//
//            if territory_end == current_string.len() {
//                break;
//            }
//
//            let copy = current_string.clone();
//            let (_, next) = copy.split_at(territory_end);
//
//            current_string = next.to_string();
//        } else {
//            break;
//        }
//    }
//
//    println!("count: {}", sum);
}

fn calc_num_characters(string: String) -> u64 {

    if string.contains('(') {
        if !string.starts_with('(') {

            let (pre, post) = string.split_at(string.find('(').unwrap());
            return pre.len() as u64 + calc_num_characters(post.to_string());

        } else {

            //we know start = index 0
            let start: usize = 0;
            let end: usize = string.find(')').unwrap();

            let marker = parse_marker(string.as_str(), start, end);
            let marker_territory = get_marker_territory(string.as_str(), &marker, end);

            println!("territory: {}", marker_territory);

            let mut territory_char_count = 0;

            if marker_territory.contains('(') {
                //territory contains another marker
                territory_char_count = marker.repetitions * calc_num_characters(marker_territory.clone());
            } else {
                territory_char_count = marker.repetitions * marker.length;
            }

            if end + marker_territory.len() != string.len() - 1 {
                //there's more shit after this one.

                let (pre, post) = string.split_at(end + 1 + marker_territory.len());
                return territory_char_count + calc_num_characters(post.to_string());

            } else {
                //we're on the last one
                return territory_char_count;
            }
        }
    }
    string.len() as u64
}

fn get_marker_territory(string: &str, marker: &Marker, marker_end: usize) -> String {
    get_sub_string(string.clone(), marker_end + 1, marker_end + 1 + marker.length as usize).to_string()
}

fn find_next_marker(string: &str) -> (Option<usize>, Option<usize>) {
    (string.find("("), string.find(")"))
}

fn parse_marker(string: &str, first: usize, second: usize) -> Marker {
    let (temp, _) = string.split_at(second);
    let (_, temp) = temp.split_at(first + 1);

    let numbers: Vec<u64> = temp.split('x')
        .map(|x| {
            x.parse().unwrap_or(1u64)
        }).collect();

    Marker {
        length: numbers[0],
        repetitions: numbers[1],
    }
}

fn get_sub_string(string: &str, start: usize, end: usize) -> &str {
    let (temp, _) = string.split_at(end);
    let (_, sub_string) = temp.split_at(start);
    sub_string
}

fn get_input_string() -> Result<String, std::io::Error> {
    let mut input_file = try!(File::open("../input/day-9.txt"));
    let mut input_string = String::new();

    try!(input_file.read_to_string(&mut input_string));

    Ok(input_string)
}

#[cfg(test)]
mod tests {
    use super::Marker;
    use super::get_sub_string;
    use super::find_next_marker;
    use super::parse_marker;
    use super::calc_num_characters;
    use super::get_marker_territory;

    #[test]
    fn test_get_sub_string() {
        let string = "abcdefghij";

        let sub_str = get_sub_string(string, 0, 5);

        assert_eq!("abcde", sub_str);
    }

    #[test]
    fn test_find_next_marker() {
        let string = "abcdefgh(2x10)ij";

        let (start, end) = find_next_marker(string);

        assert_eq!(start, Some(8));
        assert_eq!(end, Some(13));
    }

    #[test]
    fn test_parse_marker() {
        let string = "abcdefgh(2x10)ij";

        let marker = parse_marker(string, 8, 13);

        assert_eq!(marker.length, 2);
        assert_eq!(marker.repetitions, 10);
    }

    #[test]
    fn test_get_marker_territory() {
        let string = "abcdefgh(2x10)ij";

        let marker = parse_marker(string, 8, 13);
        let territory = get_marker_territory(string, &marker, 13);

        assert_eq!(territory, "ij");

        let string = "(2x10)ij";

        let marker = parse_marker(string, 0, 5);
        let territory = get_marker_territory(string, &marker, 5);

        assert_eq!(territory, "ij");
        assert_eq!(5 + territory.len(), string.len() - 1);
    }

    #[test]
    fn test_calc_num_characters() {
        let string = "ab".to_string();
        let sum = calc_num_characters(string);
        assert_eq!(sum, 2);

        let string = "ab(1x2)a".to_string();
        let sum = calc_num_characters(string);
        assert_eq!(sum, 4);

        let string = "ab(1x2)a(2x3)ab".to_string();
        let sum = calc_num_characters(string);
        assert_eq!(sum, 10);

        let string = "(9x2)(1x2)abcde".to_string();
        let sum = calc_num_characters(string);
        assert_eq!(sum, 11);
    }

}
