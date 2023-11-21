use bitreader::BitReader;
use std::fs;

fn handle_operator(op: u8, vals: &[u64]) -> u64 {
    match op {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => {
            if vals[0] > vals[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if vals[0] < vals[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if vals[0] == vals[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("!kaputt!"),
    }
}

fn parse_packet(br: &mut BitReader) -> (u32, u64) {
    let mut ver = br.read_u32(3).unwrap();
    let type_id = br.read_u8(3).unwrap();
    match type_id {
        4 => {
            // literal
            let mut val = 0u64;
            loop {
                let last_group = br.read_u8(1).unwrap();
                val <<= 4;
                val |= br.read_u64(4).unwrap();
                if last_group == 0 {
                    return (ver, val);
                }
            }
        }
        op => {
            // operator
            let length_type_id = br.read_u8(1).unwrap();
            let mut vals: Vec<u64> = Vec::new();
            match length_type_id {
                0 => {
                    let total_bit_len = br.read_u16(15).unwrap();
                    let cpos = br.position();
                    loop {
                        let (pver, pval) = parse_packet(br);
                        ver += pver;
                        vals.push(pval);
                        if br.position() == cpos + total_bit_len as u64 {
                            break;
                        }
                    }
                }
                1 => {
                    let num_sub_packets = br.read_u16(11).unwrap();
                    (0..num_sub_packets).for_each(|_| {
                        let (pver, pval) = parse_packet(br);
                        ver += pver;
                        vals.push(pval);
                    })
                }
                _ => panic!("!kaputt!"),
            }
            (ver, handle_operator(op, &vals))
        }
    }
}

fn solution(input: &str) -> (u32, u64) {
    let data = hex::decode(input).unwrap();
    let mut br = BitReader::new(&data);
    parse_packet(&mut br)
}

#[test]
fn test_run() {
    assert_eq!(solution("8A004A801A8002F478"), (16, 15));
    assert_eq!(solution("620080001611562C8802118E34"), (12, 46));
    assert_eq!(solution("C0015000016115A2E0802F182340"), (23, 46));
    assert_eq!(solution("A0016C880162017C3686B18A3D4780"), (31, 54));
    let input = fs::read_to_string("src/inputs/aoc_16.input").unwrap();
    assert_eq!(solution(&input), (873, 402817863665));
}

pub fn run() -> (String, String) {
    let input = fs::read_to_string("src/inputs/aoc_16.input").unwrap();
    let (p1, p2) = solution(&input);
    (p1.to_string(), p2.to_string())
}
