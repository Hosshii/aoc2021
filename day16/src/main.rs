fn main() -> MyResult<()> {
    let input = include_str!("../input/input");
    let input = parse(input);
    let (ipt, packet) = parse_packet(&input)?;
    println!("{:?}", packet);
    println!("{}", packet.version_sum());
    Ok(())
}

fn parse(ipt: &str) -> String {
    ipt.chars().map(to_string).collect()
}

fn to_string(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unimplemented!(),
    }
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Packet {
    version: u8,
    type_id: u8,
    kind: PacketKind,
}

impl Packet {
    pub fn version_sum(&self) -> u32 {
        let child_version = match &self.kind {
            PacketKind::Literal(_) => 0,
            PacketKind::SubPackets(sub_packets) => sub_packets
                .iter()
                .fold(0, |acc, cur| acc + cur.version_sum()),
        };

        self.version as u32 + child_version
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum PacketKind {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

fn to_dec<'a>(chars: impl DoubleEndedIterator<Item = &'a char>) -> u64 {
    chars.rev().enumerate().fold(
        0,
        |acc, (idx, c)| {
            if *c == '1' {
                acc + (1 << idx)
            } else {
                acc
            }
        },
    )
}

fn parse_digit3(ipt: &str) -> MyResult<(&str, u8)> {
    let mut ipt_iter = ipt.char_indices().peekable();
    let (a, b, (_, c)) = (
        ipt_iter.next().unwrap().1,
        ipt_iter.next().unwrap().1,
        ipt_iter.next().unwrap(),
    );

    let idx = ipt_iter.peek().unwrap().0;

    let (_, ret) = ipt.split_at(idx);

    Ok((ret, to_dec([a, b, c].iter()) as u8))
}

fn parse_digit1(ipt: &str) -> MyResult<(&str, u8)> {
    let mut ipt_iter = ipt.char_indices().peekable();
    let (_, a) = ipt_iter.next().ok_or("empty")?;

    let idx = ipt_iter.peek().unwrap().0;
    let (_, ret) = ipt.split_at(idx);

    Ok((ret, to_dec([a].iter()) as u8))
}

fn parse_digit_n(ipt: &str, n: usize) -> MyResult<(&str, u64)> {
    let mut ipt_iter = ipt.char_indices().peekable();
    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        let c = ipt_iter.next().ok_or("empty")?.1;
        v.push(c);
    }

    let idx = ipt_iter.peek().unwrap().0;

    let (_, ret) = ipt.split_at(idx);

    Ok((ret, to_dec(v.iter())))
}

fn parse_version(ipt: &str) -> MyResult<(&str, u8)> {
    let (ipt, version) = parse_digit3(ipt)?;
    Ok((ipt, version))
}

fn parse_type_id(ipt: &str) -> MyResult<(&str, u8)> {
    let (ipt, type_id) = parse_digit3(ipt)?;
    Ok((ipt, type_id))
}

fn parse_length_type_id(ipt: &str) -> MyResult<(&str, u8)> {
    let (ipt, length_type_id) = parse_digit1(ipt)?;
    Ok((ipt, length_type_id))
}

fn parse_literal(ipt: &str) -> MyResult<(&str, u64)> {
    let mut ipt_iter = ipt.char_indices().peekable();
    let mut v = Vec::new();
    loop {
        let is_end = ipt_iter.next().ok_or("Invalid literal")?.1 == '0';
        for _ in 0..4 {
            let (_, c) = ipt_iter.next().ok_or("Invalid literal")?;
            v.push(c);
        }
        let (idx, _) = ipt_iter.peek().ok_or("Invalid literal")?;

        if is_end {
            let (_, ret) = ipt.split_at(*idx);

            break Ok((ret, to_dec(v.iter())));
        }
    }
}

fn parse_operator(ipt: &str) -> MyResult<(&str, Vec<Packet>)> {
    let (ipt, l_type_id) = parse_length_type_id(ipt)?;
    let mut packets = Vec::new();
    match l_type_id {
        0 => {
            let (ipt, total_bit) = parse_digit_n(ipt, 15)?;
            let last = &ipt[total_bit as usize..];
            let mut ipt = ipt;
            loop {
                let (_ipt, packet) = parse_packet(ipt)?;
                ipt = _ipt;
                packets.push(packet);
                if ipt.is_empty() || ipt == last {
                    break Ok((last, packets));
                }
            }
        }
        1 => {
            let (mut ipt, total_packet) = parse_digit_n(ipt, 11)?;
            for _ in 0..total_packet {
                let (_ipt, packet) = parse_packet(ipt)?;
                ipt = _ipt;
                packets.push(packet);
            }
            Ok((ipt, packets))
        }
        _ => unimplemented!(),
    }
}

fn parse_packet(ipt: &str) -> MyResult<(&str, Packet)> {
    let (ipt, version) = parse_version(ipt)?;
    let (ipt, type_id) = parse_type_id(ipt)?;
    let (ipt, kind) = match type_id {
        4 => {
            let (ipt, literal) = parse_literal(ipt)?;
            (ipt, PacketKind::Literal(literal))
        }
        _ => {
            let (ipt, sub_packets) = parse_operator(ipt)?;
            (ipt, PacketKind::SubPackets(sub_packets))
        }
    };
    Ok((
        ipt,
        Packet {
            version,
            type_id,
            kind,
        },
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_dec() {
        let ipt = [
            (vec!['1'], 1),
            (vec!['1', '0', '1'], 5),
            (vec!['1', '0', '1', '0', '1'], 21),
            (vec!['1', '0', '1', '0', '1', '0', '1'], 85),
        ];

        for (ipt, ret) in ipt.iter() {
            assert_eq!(super::to_dec(ipt.iter()), *ret);
        }
    }
}
