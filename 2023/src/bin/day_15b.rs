fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, &str)>> = vec![vec![]; 256];
    for s in input.split(",") {
        if s.contains("=") {
            let (label, focal_len) = s.split_once("=").unwrap();
            let b = &mut boxes[hash(label)];
            if let Some(item) = b.iter_mut().find(|(l, _)| *l == label) {
                // replace the old lens with the new lens
                item.1 = focal_len;
            } else {
                // add the lens to the box immediately behind any lenses already
                // in the box
                b.push((label, focal_len));
            }
        } else {
            // go to the relevant box and remove the lens with the given label
            // if it is present in the box.
            // Then, move any remaining lenses as far forward in the box as they
            // can go without changing their order, filling any space made by
            // removing the indicated lens.
            let label = &s[0..s.len() - 1];
            let b = &mut boxes[hash(label)];
            if let Some((idx, _)) = b.iter().enumerate().find(|(_, (l, _))| *l == label) {
                b.remove(idx);
            }
        }
    }

    // Sum the focusing powers
    // The focusing power of a single lens is the result of multiplying together:
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_num, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(idx, (_, f))| (box_num + 1) * (idx + 1) * f.parse::<usize>().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 145);
    }
}

util::read_main!();
