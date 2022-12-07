use std::collections::HashMap;

fn solve(input: &str) -> usize {
    const ROOT_NODE: (&str, usize) = ("/", 0);
    let mut child_to_parent: HashMap<(&str, usize), (&str, usize)> = HashMap::new();
    let mut dir_to_size: HashMap<(&str, usize), usize> = HashMap::new();

    // root dir
    child_to_parent.insert(ROOT_NODE, ("", 0));
    dir_to_size.insert(ROOT_NODE, 0);
    let mut cur_dir = ROOT_NODE;

    for line in input
        .trim()
        .lines()
        .skip(2)
        .filter(|l| !l.starts_with("$ ls"))
    {
        // cd
        if &line[0..1] == "$" {
            let mut parts = line.split(" ").skip(2);
            let new_dir_name = parts.next().unwrap();

            match new_dir_name {
                // Up a level
                ".." => {
                    // Add self's size parent
                    let parent = *child_to_parent.get(&cur_dir).unwrap();
                    let cur_dir_size = *dir_to_size.get(&cur_dir).unwrap();
                    let v = dir_to_size.entry(parent).or_default();
                    *v += cur_dir_size;
                    cur_dir = parent;
                }
                // Down a level
                _ => {
                    let new_dir = (new_dir_name, child_to_parent.len());
                    child_to_parent.insert(new_dir, cur_dir);
                    cur_dir = new_dir;
                }
            }
            continue;
        }

        // ls results
        let (op, _) = line.split_once(' ').unwrap();
        if op == "dir" {
            continue;
        }
        let v = dir_to_size.entry(cur_dir).or_default();
        *v += op.parse::<usize>().unwrap();
    }

    // Final leaf - add size to all ancestors
    let mut iter_dir = cur_dir;
    loop {
        // finished processing root
        if iter_dir == ("", 0) {
            break;
        }

        let iter_dir_size = *dir_to_size.get(&iter_dir).unwrap();
        iter_dir = *child_to_parent.get(&iter_dir).unwrap();
        let v = dir_to_size.entry(iter_dir).or_insert(0);
        *v += iter_dir_size;
    }

    let unused_space = 70000000 - dir_to_size.get(&ROOT_NODE).unwrap();
    let target_size = 30000000 - unused_space;

    dir_to_size
        .into_values()
        .filter(|&v| v >= target_size)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solve(input), 24933642);
    }
}

util::read_main!();
