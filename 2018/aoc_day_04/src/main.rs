use std::time::Instant;
use std::collections::HashMap;
use aoc_util::aoc_util::*;

fn main() -> AocResult<()> {
    let problem_input = get_problem_input()?;
    let start = Instant::now();
    println!("1: Coded value: {}", part_1(&problem_input)?);
    let after_1 = Instant::now();
    println!("2: Coded value: {:?}", part_2(&problem_input)?);
    let after_2 = Instant::now();
    println!("1: {:?}  2: {:?}", after_1 - start, after_2 - after_1 );

    Ok(())
}

/// strategy one: find guard with most asleep minutes, then find the minute most asleep
fn part_1(input: &str) -> AocResult<u32> {
    let entries = parse_input(input)?;

    let guard_detail = fill_guard_details(&entries)?;

    let mut sleepiest_minute = 0;
    let mut sleepiest_minute_value = 0;
    let mut sleepiest_total_minutes = 0;
    let mut sleepiest_guard = 0;
    //find the guard with the most sleeping minutes
    for (guard, times) in &guard_detail {
        let mut total_time = 0;
        for t in 0..60 {
            total_time += times[t];
        }

        if total_time > sleepiest_total_minutes {
            sleepiest_total_minutes = total_time;
            sleepiest_guard = *guard;
        }
    }

    //find the most-slept minute for the guard selected above
    for t in 0..60 {
        if guard_detail[&sleepiest_guard][t] > sleepiest_minute_value {
            sleepiest_minute = t as u32; 
            sleepiest_minute_value = guard_detail[&sleepiest_guard][t];
        }
    }

    println!("1: G:{} T:{} M:{} V:{}", sleepiest_guard, sleepiest_total_minutes, sleepiest_minute, sleepiest_minute_value);

    Ok(sleepiest_minute * sleepiest_guard)
}

/// strategy two: find the particular minute with the most sleep and the guard that sleeps on that minute
fn part_2(input: &str) -> AocResult<u32> {
    let entries = parse_input(input)?;

    let guard_detail = fill_guard_details(&entries)?;

    let mut sleepiest_minute = 0;
    let mut sleepiest_minute_value = 0;
    let mut sleepiest_guard = 0;
    //find the guard with the most sleeping minutes
    for (guard, times) in &guard_detail {
        for m in 0..60 {
            if times[m] > sleepiest_minute_value {
                sleepiest_minute = m as u32;
                sleepiest_minute_value = times[m];
                sleepiest_guard = *guard;
            }
        }
    }

    println!("2: G:{} M:{} V:{}", sleepiest_guard, sleepiest_minute, sleepiest_minute_value);

    Ok(sleepiest_minute * sleepiest_guard)
}

#[derive(Debug)]
enum EntryDetail {
    GuardShiftBegin(u32),
    FallAsleep,
    WakeUp,
    Invalid
}
struct LogEntry {
    hour: u32,
    minute: u32,
    timecode: u64,
    detail: EntryDetail
}

/// build a detailed map of each guard's midnight hour
fn fill_guard_details(entries: &Vec<LogEntry>) -> AocResult<HashMap<u32, [u32;60]>>{
    let mut current_guard = 0;
    enum State {
        NoGuard,
        Asleep(u32),
        Awake(u32)
    };
    let mut state = State::NoGuard;
    let mut guard_detail: HashMap<u32, [u32;60]> = HashMap::new();

    //walk through the sorted entries, building a picture of the midnight hour for each guard
    for e in entries {
        match e.detail {
            EntryDetail::GuardShiftBegin(g) => {
                current_guard = g;
                state = State::Awake (if e.hour != 0 {
                    0
                }
                else {
                    e.minute
                });
            },
            EntryDetail::FallAsleep => {
                match state {
                    State::Awake(_) => (),
                    _ => panic!("Falling asleep from non-awake state")
                }
                state = State::Asleep(e.minute);
            },
            EntryDetail::WakeUp => {
                match state {
                    State::Asleep(time) => {
                        for m in time..e.minute {
                            guard_detail.entry(current_guard).or_insert([0; 60])[m as usize] += 1;
                        }
                    },
                    _ => panic!("Waking up from non-asleep state")
                }
                state = State::Awake(e.minute);
            },
            EntryDetail::Invalid => {
                panic!("Invalid Entry");
            }
        }
    }

    Ok(guard_detail)
}

///Parse and chronologically sort the input.txt file
fn parse_input(input: &str) -> AocResult<Vec<LogEntry>> {
    let mut entries = Vec::new();

    for ln in input.lines() {
        //Shortest line format: [1518-11-07 00:52] wakes up
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
            hour, minute, timecode, detail
        });
    }

    entries.sort_by_key(|e| e.timecode);
    
    Ok(entries)
}
