fn to_u64(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

fn operate(operator: &str, values: Vec<u64>) -> u64 {
    match operator {
        "000" => values.into_iter().sum(),
        "001" => values.into_iter().product(),
        "010" => values.into_iter().min().unwrap(),
        "011" => values.into_iter().max().unwrap(),
        "101" => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        "110" => {
            if values[1] > values[0] {
                1
            } else {
                0
            }
        }
        "111" => {
            if values[1] == values[0] {
                1
            } else {
                0
            }
        }
        _ => panic!("unknown operator"),
    }
}

#[derive(Debug)]
struct Parser {
    ptr: usize,
    version_sum: u64,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            ptr: 0,
            version_sum: 0,
        }
    }

    fn add_version(&mut self, version: &str) {
        self.version_sum += to_u64(version);
    }

    fn parse(&mut self, packet: &str) -> u64 {
        // version - 3 bits
        let version = &packet[self.ptr..self.ptr + 3];
        self.ptr += 3;
        // packet type - 3 bits
        let packet_type = &packet[self.ptr..self.ptr + 3];
        self.ptr += 3;

        self.add_version(version);

        match packet_type {
            // Literal - 4
            "100" => {
                let mut end = self.ptr;
                loop {
                    if &packet[end..=end] == "0" {
                        end += 4;
                        break;
                    }
                    end += 5;
                }

                let literal: String = (&packet[self.ptr..=end])
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, c)| if idx % 5 == 0 { None } else { Some(c) })
                    .collect();
                let value = to_u64(&literal);

                self.ptr = end + 1;
                return value;
            }
            // Everything else - operator
            _ => {
                let length_type = &packet[self.ptr..=self.ptr];
                self.ptr += 1;
                match length_type {
                    // Total length variant
                    "0" => {
                        let total_length = &packet[self.ptr..self.ptr + 15];
                        let total_length = to_u64(total_length);
                        self.ptr += 15;
                        let target_length = self.ptr + total_length as usize;
                        let mut values = vec![];
                        while self.ptr < target_length {
                            values.push(self.parse(packet));
                        }
                        return operate(packet_type, values);
                    }
                    // Num packets variant
                    "1" => {
                        let num_packets = &packet[self.ptr..self.ptr + 11];
                        let num_packets = to_u64(num_packets);
                        self.ptr += 11;
                        let values = (0..num_packets).map(|_| self.parse(packet)).collect();
                        return operate(packet_type, values);
                    }
                    _ => unreachable!("should not be here"),
                }
            }
        }
    }
}

fn solve(input: &str) -> u64 {
    let packet: String = input
        .trim()
        .chars()
        .flat_map(|c| format!("{:04b}", c.to_digit(16).unwrap()).into_bytes())
        .map(|b| b as char)
        .collect();

    let mut parser = Parser::new();
    parser.parse(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"C200B40A82";
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn case2() {
        let input = r"04005AC33890";
        assert_eq!(solve(input), 54);
    }

    #[test]
    fn case3() {
        let input = r"880086C3E88112";
        assert_eq!(solve(input), 7);
    }

    #[test]
    fn case4() {
        let input = r"CE00C43D881120";
        assert_eq!(solve(input), 9);
    }

    #[test]
    fn case5() {
        let input = r"D8005AC2A8F0";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case6() {
        let input = r"F600BC2D8F";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case7() {
        let input = r"9C005AC2F8F0";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case8() {
        let input = r"9C0141080250320F1802104A08";
        assert_eq!(solve(input), 1);
    }
}

util::read_main!();
