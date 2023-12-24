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
    println!("2: Non-overlapping extents: {:?}", part_2(&problem_input)?);
    let after_2 = Instant::now();
    println!("1a: {:?}  1b: {:?}  2: {:?}", after_a - start, after_b - after_a, after_2 - after_b);

    Ok(())
}

///HashMap implementation. Looks nice but is actually considerably slower
/// Likely saves memory, though.
fn part_1a(input: &String) -> AocResult<i32> {
    let extents = parse_input(&input)?;

    let mut overlaps: HashMap<(u32,u32), u32> = HashMap::new();
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

///2D array implementation
/// Fast, preallocates a fair bit of memory
fn part_1b(input: &String) -> AocResult<i32> {
    let extents = parse_input(&input)?;

    //nope, stack buster
    //let mut overlaps: [[u32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut overlaps: Vec<[u32; 1000]> = vec![[0; 1000]; 1000];

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

/// return a vector of owner_id where each entry is a non-overlapping claim
fn part_2(input: &String) -> AocResult<Vec<u32>> {
    let extents = parse_input(&input)?;

    let mut non_overlaps: Vec<u32> = Vec::new();

    //test each extent against every other, checking for overlaps
    //optimization potential: keep a list, so if e overlaps with f, f is also tagged on the first pass
    'outer: for e in &extents {
        for f in &extents {
            //skip ourself
            if e.owner_id == f.owner_id{
                continue;
            }
            //short-circuit if we find an overlap
            if overlap(&e, &f) {
               continue 'outer; 
            }
        }
        //e has no overlaps if we get here, add it to the list
        non_overlaps.push(e.owner_id);
    }

    Ok(non_overlaps)
}

struct Extent {
    pub raw: String,
    pub owner_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32
}

/// Parse input.txt of the form
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


/// check if two extents overlap
/// 
fn overlap(e1: &Extent, e2: &Extent) -> bool {
    // for readability
    let e1x1 = e1.x;
    let e1x2 = e1.x + e1.w - 1;
    let e1y1 = e1.y;
    let e1y2 = e1.y + e1.h - 1;
    let e2x1 = e2.x;
    let e2x2 = e2.x + e2.w - 1;
    let e2y1 = e2.y;
    let e2y2 = e2.y + e2.h - 1;

    //bounding box test
    e1x1 <= e2x2 && e1x2 >= e2x1 && e1y1 <= e2y2 && e1y2 >= e2y1  
}
