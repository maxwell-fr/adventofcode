use std::collections::HashMap;
use std::time::Instant;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    let start = Instant::now();
    println!("1a: Number of multiowner cells: {}", part_1a(&problem_input)?);
    let after_a = Instant::now();
    println!("1b: Number of multiowner cells: {}", part_1b(&problem_input)?);
    let after_b = Instant::now();
    println!("1a: {:?}  1b: {:?}", after_a - start, after_b - after_a);

    Ok(())
}

fn part_1a(input: &String) -> AocResult<i32> {
    let extents = parse_input(&input)?;
    //println!("1a: {} entries found.", extents.len());

    let mut overlaps: HashMap<(u32,u32), u32> = HashMap::with_capacity(250000);
    let mut count = 0;
    for e in extents {
        for x in e.x..e.x+e.w {
            for y in e.y..e.y+e.h {
                let cell = overlaps.entry((x,y)).or_insert(0);
                *cell += 1;
                if *cell == 2 {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn part_1b(input: &String) -> AocResult<i32> {
    let extents = parse_input(&input)?;
    println!("1b: {} entries found.", extents.len());

    //nope, stack buster
    //let mut overlaps: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut overlaps: Vec<[u32; 1000]> = Vec::new();
    for i in 0..1000{
        overlaps.push([0;1000]);
    }

    let mut count = 0;
    for e in extents {
        for x in e.x..e.x+e.w {
            for y in e.y..e.y+e.h {
                overlaps [x as usize][y as usize] += 1;
                if overlaps[x as usize ][y as usize] == 2 {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

struct Extent {
    pub raw: String,
    pub owner_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32
}

/// Parse input of the form
/// #22 @ 121,2: 18x27
/// ID^   X^  ^Y W^  ^H
/// into a vector of Extents
fn parse_input(input: &String) -> AocResult<Vec::<Extent>> {
    let mut parsed: Vec::<Extent> = Vec::new();
    for ln in input.lines() {
        let pieces: Vec<&str> = ln.split(|c: char|!c.is_numeric()).filter(|p| p.len() != 0).collect();
        
        if pieces.len() != 5 {
            continue;
        }

        parsed.push(Extent {
            raw: ln.to_owned(), 
            owner_id: pieces[0].parse()?,
            x: pieces[1].parse()?, y: pieces[2].parse()?,
            w: pieces[3].parse()?, h: pieces[4].parse()?
        }
        );
    }
               
    Ok(parsed)
}
