fn to_u64(s: &str) -> u64 {
    u64::from_str_radix(s, 2).unwrap()
}

#[derive(Debug)]
struct Info {
    version_sum: u64,
}

impl Info {
    fn new() -> Info {
        Info { version_sum: 0 }
    }

    fn add_version(&mut self, version: &str) {
        self.version_sum += to_u64(version);
    }
}

#[derive(Debug)]
struct Parser {
    ptr: usize,
    info: Info,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            ptr: 0,
            info: Info::new(),
        }
    }

    fn parse(&mut self, packet: &str) {
        // version - 3 bits
        let version = &packet[self.ptr..self.ptr + 3];
        self.ptr += 3;
        // packet type - 3 bits
        let packet_type = &packet[self.ptr..self.ptr + 3];
        self.ptr += 3;

        self.info.add_version(version);

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

                // unused now, but probably used later
                let _literal: String = (&packet[self.ptr..=end])
                    .chars()
                    .enumerate()
                    .filter_map(|(idx, c)| if idx % 5 == 0 { None } else { Some(c) })
                    .collect();
                let _value = to_u64(&_literal);

                self.ptr = end + 1;
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
                        while self.ptr < target_length {
                            self.parse(packet);
                        }
                    }
                    // Num packets variant
                    "1" => {
                        let num_packets = &packet[self.ptr..self.ptr + 11];
                        let num_packets = to_u64(num_packets);
                        self.ptr += 11;
                        (0..num_packets).for_each(|_| {
                            self.parse(packet);
                        })
                    }
                    _ => {}
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
    parser.parse(&packet);
    parser.info.version_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"8A004A801A8002F478";
        assert_eq!(solve(input), 16);
    }

    #[test]
    fn case2() {
        let input = r"620080001611562C8802118E34";
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn case3() {
        let input = r"C0015000016115A2E0802F182340";
        assert_eq!(solve(input), 23);
    }

    #[test]
    fn case4() {
        let input = r"A0016C880162017C3686B18A3D4780";
        assert_eq!(solve(input), 31);
    }
}

util::read_main!();
