use bitreader::BitReader;
use itertools::FoldWhile;
use itertools::Itertools;
use std::{fs, iter};

fn handle_operator(op: u8, vals: &[u64]) -> u64 {
    match op {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => (vals[0] > vals[1]) as u64,
        6 => (vals[0] < vals[1]) as u64,
        7 => (vals[0] == vals[1]) as u64,
        _ => panic!("!kaputt!"),
    }
}

fn handle_literal(br: &mut BitReader) -> u64 {
    iter::repeat(0)
        .fold_while((1, 0u64), |(last_group, val), _| {
            if last_group != 0 {
                FoldWhile::Continue((br.read_u8(1).unwrap(), (val << 4) | br.read_u64(4).unwrap()))
            } else {
                FoldWhile::Done((0, val))
            }
        })
        .into_inner()
        .1
}

fn handle_packets_by_bit_size(br: &mut BitReader, cver: u32, op: u8) -> (u32, u64) {
    let max_pos = br.read_u16(15).unwrap() as u64 + br.position();
    let (ver, vals) = iter::repeat(0)
        .fold_while((0u32, Vec::new()), |(ver, mut vals), _| {
            if br.position() == max_pos {
                FoldWhile::Done((ver, vals))
            } else {
                let (pver, pval) = parse_packet(br);
                vals.push(pval);
                FoldWhile::Continue((ver + pver, vals))
            }
        })
        .into_inner();
    (cver + ver, handle_operator(op, &vals))
}

fn handle_packets_by_cnt(br: &mut BitReader, cver: u32, op: u8) -> (u32, u64) {
    let (ver, vals) =
        (0..br.read_u16(11).unwrap()).fold((0u32, Vec::new()), |(ver, mut vals), _| {
            let (pver, pval) = parse_packet(br);
            vals.push(pval);
            (ver + pver, vals)
        });
    (cver + ver, handle_operator(op, &vals))
}

fn parse_packet(br: &mut BitReader) -> (u32, u64) {
    let ver = br.read_u32(3).unwrap();
    match br.read_u8(3).unwrap() {
        4 => (ver, handle_literal(br)),
        op => match br.read_u8(1).unwrap() {
            0 => handle_packets_by_bit_size(br, ver, op),
            1 => handle_packets_by_cnt(br, ver, op),
            _ => panic!("!kaputt!"),
        },
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
