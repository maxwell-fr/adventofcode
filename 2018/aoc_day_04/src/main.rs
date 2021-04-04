use std::time::Instant;
use std::collections::HashMap;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    let start = Instant::now();
   // println!("1a: Number of multiowner cells: {}", part_1a(&problem_input)?);
    let after_a = Instant::now();
   // println!("1b: Number of multiowner cells: {}", part_1b(&problem_input)?);
    let after_b = Instant::now();
   // println!("2: Non-overlapping extents: {:?}", part_2(&problem_input)?);
   let i = parse_input(&problem_input)?;
   for e in i {
       println!("{} {:?}", e.timecode, e.detail);
   }
    let after_2 = Instant::now();
    println!("1a: {:?}  1b: {:?}  2: {:?}", after_a - start, after_b - after_a, after_2 - after_b);

    Ok(())
}

#[derive(Debug)]
enum EntryDetail {
    GuardShiftBegin(u32),
    FallAsleep,
    WakeUp,
    Invalid
}
struct LogEntry {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    timecode: u64,
    detail: EntryDetail
}

fn parse_input(input: &str) -> AocResult<Vec<LogEntry>> {
    let mut entries = Vec::new();

    for ln in input.lines() {
        // Shortest line format: [1518-11-07 00:52] wakes up
        //basic formatting check
        if !ln.contains(&['[', ']', '-', ':'][..]) || ln.len() < 27 {
            continue;
        }
        let year = ln[1..5].parse::<u32>()?;
        let month = ln[6..8].parse::<u32>()?;
        let day = ln[9..11].parse::<u32>()?;
        let hour = ln[12..14].parse::<u32>()?;
        let minute = ln[15..17].parse::<u32>()?;
        let timecode: u64 = (minute + hour*100 + day * 10_000 + month * 1_000_000) as u64+ (year as u64 * 100_000_000);
        let detail_str = &ln[19..];
        
        let detail = 
        if detail_str.contains("wakes") {
            EntryDetail::WakeUp
        }
        else if detail_str.contains("asleep") {
            EntryDetail::FallAsleep
        }
        else if detail_str.contains("Guard #") && detail_str.contains("begins shift"){
            let id: u32 = detail_str[detail_str.find('#').unwrap() + 1 .. detail_str.find(" begins").unwrap()].parse()?;
            EntryDetail::GuardShiftBegin(id)
        }
        else {
            EntryDetail::Invalid
        };


        entries.push(LogEntry{ 
            year, month, day, hour, minute, timecode, detail
        });
    }

    entries.sort_by_key(|e| e.timecode);
    
    Ok(entries)
}
