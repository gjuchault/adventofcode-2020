use crate::utils;
use num_bigint::BigInt;
use std::time::SystemTime;

fn part1(minimum_timestamp: u32, all_bus: &Vec<(u32, i32)>) {
    let now = SystemTime::now();

    let mut result_id = 0;
    let mut result_timestamp = minimum_timestamp * 2;

    for (bus, _) in all_bus {
        let ratio = minimum_timestamp as f32 / *bus as f32;
        let bus_timestamp = ratio.ceil() as u32 * bus;
        if bus_timestamp < result_timestamp {
            result_id = *bus;
            result_timestamp = bus_timestamp;
        }
    }

    let result = result_id * (result_timestamp - minimum_timestamp);

    println!("Part 1: {}", result);
    println!("Part 1 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn part2(all_bus: &Vec<(u32, i32)>) {
    let now = SystemTime::now();

    let mut result = BigInt::from(0);

    // 7,13,x,x,59,x,31,19
    // x = 0 mod 7
    // x = -1 mod 13 = 12 mod 13
    // x = -3 mod 59 = 56 mod 59
    // x = -2 mod 31 = 29 mod 31
    // x = -1 mod 19 = 18 mod 31
    let mut m = BigInt::from(1);

    // https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    for (bus, _) in all_bus {
        m = m * bus;
    }

    for (bus, idx) in all_bus {
        let rest = negative_mod_to_mod(*idx as i32, *bus as i32);
        // println!("x = {} mod {}", rest, bus);

        let m_i = m.clone() / bus;
        let y_i = egcd(m_i.clone(), BigInt::from(*bus as i32));

        result = result.clone() + rest * m_i.clone() * y_i.clone();

        // println!("  Mi={} M={} y_i={}", m_i, m, y_i);
        // println!(
        //     "  partial result {}*{}*{} = {}",
        //     rest,
        //     m_i,
        //     y_i,
        //     rest * m_i.clone() * y_i.clone()
        // );
        // println!("  partial result sum {}", result);
    }

    println!("Part 2: {}", result % m);
    println!("Part 2 took: {}ms", now.elapsed().unwrap().as_millis());
}

fn negative_mod_to_mod(a: i32, b: i32) -> i32 {
    let mut result = a;

    while result < 0 {
        result += b;
    }

    while result > b {
        result -= b;
    }

    return result;
}

// n = Mi, b = bus
fn egcd(b: BigInt, n: BigInt) -> BigInt {
    let mut n0 = n.clone();
    let mut b0 = b.clone();
    let mut t0 = BigInt::from(0);
    let mut t = BigInt::from(1);
    let mut q = n0.clone() / b0.clone();
    let mut r = n0.clone() - q.clone() * b0.clone();
    while r > BigInt::from(0) {
        let mut temp = t0 - q.clone() * t.clone();

        if temp >= BigInt::from(0) {
            temp = temp.clone() % n.clone();
        } else {
            temp = n.clone() - (-1 * temp.clone() % n.clone())
        };
        t0 = t.clone();
        t = temp.clone();
        n0 = b0.clone();
        b0 = r.clone();
        q = n0.clone() / b0.clone();
        r = n0.clone() - q.clone() * b0.clone();
    }
    return t;
}

pub fn run() {
    println!("Running day13");

    let now = SystemTime::now();
    let input: String = utils::input::read("day13");
    let input_lines: Vec<&str> = input.split("\n").collect();

    let minimum_timestamp: u32 = input_lines[0].parse().unwrap();
    let all_bus_str: Vec<&str> = input_lines[1].split(",").collect();
    let mut all_bus: Vec<(u32, i32)> = Vec::with_capacity(all_bus_str.len());

    let mut i = 0;

    for bus_str in all_bus_str {
        if bus_str == "x" {
            i -= 1;
            continue;
        }

        let bus = bus_str.parse().unwrap();

        all_bus.push((bus, i));

        // reset x counter
        i -= 1;
    }

    println!("Parsing took: {}ms", now.elapsed().unwrap().as_millis());

    part1(minimum_timestamp, &all_bus);
    part2(&all_bus);
}
