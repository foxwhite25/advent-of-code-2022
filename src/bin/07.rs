use indextree::{Arena, NodeId};

#[derive(Clone)]
pub struct Folder<'a> {
    name: &'a str,
    size: u32,
}

type Input<'a> = Arena<Folder<'a>>;

fn parse(input: &str) -> Input {
    let mut file_tree = Arena::new();
    let mut current_file = file_tree.new_node(Folder { name: "/", size: 0 });
    input.split("$ ").skip(2).for_each(|pair| {
        let (cmd_part, result_part) = pair.split_at(2);
        let trimmed = result_part.trim();
        match (cmd_part, trimmed) {
            ("cd", "..") => current_file = file_tree.get(current_file).unwrap().parent().unwrap(),
            ("cd", _) => current_file = current_file.children(&file_tree).find(|&x| {
                file_tree.get(x).unwrap().get().name == trimmed
            }).unwrap(),
            ("ls", _) => {
                trimmed.lines().for_each(|line| {
                    let (size, name) = line.split_once(" ").unwrap();
                    if size == "dir" {
                        current_file.append(file_tree.new_node(Folder { name, size: 0 }), &mut file_tree);
                    } else {
                        current_file
                            .ancestors(&file_tree)
                            .collect::<Vec<NodeId>>()
                            .into_iter()
                            .for_each(|x| {
                                file_tree.get_mut(x).unwrap().get_mut().size += size.parse::<u32>().unwrap()
                            })
                    }
                })
            }
            _ => unreachable!(),
        }
    });
    file_tree
}

pub fn part_one(input: Input) -> Option<u32> {
    Some(
        input
            .iter()
            .map(|node| node.get().size)
            .filter(|&folder| folder <= 100000)
            .sum()
    )
}

pub fn part_two(input: Input) -> Option<u32> {
    let mut file_iter = input.iter().map(|node| node.get().size);
    let space_required = 30000000 - (70000000 - file_iter.next().unwrap());

    file_iter.filter(|&file| file >= space_required).min()
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", 7)));
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", 7)));
        assert_eq!(result, Some(24933642));
    }
}
