use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    convert::Infallible,
    str::FromStr,
};

use Packet::{Int, List};

fn main() {
    let input = include_str!("../input/day13.txt");

    let mut part1 = 0;
    let mut packets = vec![];
    for (pair, idx) in input.split("\n\n").zip(1..) {
        let (left_str, right_str) = pair.split_once('\n').unwrap();
        let left: Packet = left_str.parse().unwrap();
        let right: Packet = right_str.parse().unwrap();
        if left < right {
            part1 += idx;
        }
        packets.push(left);
        packets.push(right);
    }
    // part 1
    assert_eq!(part1, 6076);

    // part 2
    let dividers = ["[[2]]", "[[6]]"].map(|line| line.parse().unwrap());
    packets.extend_from_slice(&dividers);
    packets.sort_unstable();

    let part2 = dividers
        .iter()
        .map(|div| packets.iter().position(|packet| packet == div).unwrap() + 1)
        .product::<usize>();

    assert_eq!(part2, 24805);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Int(u8),
}

impl Packet {
    fn push_back(&mut self, packet: Packet) -> Option<&mut Packet> {
        match self {
            List(lst) => {
                lst.push(packet);
                lst.last_mut()
            }
            Int(_) => None,
        }
    }
}

impl FromStr for Packet {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut packet = Packet::List(vec![]);
        inner(
            &mut packet,
            &mut line[1..line.len() - 1].as_bytes().iter().copied(),
        );

        fn inner(packet: &mut Packet, instructions: &mut (impl Iterator<Item = u8> + Clone)) {
            while let Some(inst) = instructions.next() {
                match inst {
                    b'[' => {
                        let new_packet = packet.push_back(List(vec![])).unwrap();
                        inner(new_packet, instructions)
                    }
                    b']' => return,
                    b',' => {}
                    _ => {
                        if matches!(instructions.clone().next(), Some(b']') | Some(b',') | None) {
                            packet.push_back(Int(inst - 48));
                        } else {
                            packet.push_back(Int(10));
                            instructions.next();
                        }
                    }
                }
            }
        }

        Ok(packet)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (List(v_left), List(v_right)) => {
                let mut idx = 0;
                loop {
                    match (v_left.get(idx), v_right.get(idx)) {
                        (None, None) => return Equal,
                        (None, Some(_)) => return Less,
                        (Some(_), None) => return Greater,
                        (Some(left), Some(right)) => {
                            if let ret @ (Less | Greater) = left.cmp(right) {
                                return ret;
                            }
                        }
                    }
                    idx += 1;
                }
            }
            (Int(i_left), Int(i_right)) => i_left.cmp(i_right),
            (p @ List(_), &Int(n)) => p.cmp(&Packet::List(vec![Packet::Int(n)])),
            (&Int(n), p @ List(_)) => Packet::List(vec![Packet::Int(n)]).cmp(p),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
