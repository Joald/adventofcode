use std::fs;

use anyhow::*;
// use itertools::Itertools;

type Stack = Vec<Vec<char>>;

fn parse_stack(lines: &[&str]) -> Result<Stack, Error> {
    // [].to_vec()
    let cols = lines.get(lines.len() - 1).ok_or(anyhow!("parse_stack(): lines.get()"))?.trim().split("  ").count();
    let mut stack = vec!(Vec::new(); cols);
    
    for line in lines.iter().rev().skip(1) {
        // let mut line = line.to_owned();
        let mut index = usize::MIN;
        let mut index2 = usize::MIN;
        // while !line.is_empty() {
        for group in line.split(|_c| {index += 1; if index == 4 {index = 0; true} else { false }}) {
            if group.starts_with('[') {
                stack[index2].push(group.chars().nth(1).unwrap_or(' '));
            }
            index2 += 1;
            // line = (&line[4..]).to_owned();
        }
    }
    // println!("[{}]", stack.iter().enumerate().map(|(i, v)| format!("{}: {}", i, v.iter().collect::<String>())).collect::<Vec<_>>().join("\n"));
    Ok(stack)
}

fn stack_str(stack: &Stack) -> String {
    stack.iter().enumerate().map(|(i, v)| format!("{}: {}", i, v.iter().collect::<String>())).collect::<Vec<_>>().join("\n")
}


fn move_elem(stack: &mut Stack, from: usize , to: usize) -> Result<(), Error> {
    let elem = stack[from].pop().ok_or(anyhow!("move_elem: {} to {}", from, to))?;
    stack[to].push(elem);
    Ok(())
}

fn part12(lines: Vec<&str>, part: bool) -> Result<String, Error> {
    let mut index: usize = 0;
    while !lines[index].is_empty() {
        index += 1;
    }
    let (stack, lines) = lines.split_at(index);
    let mut stack = parse_stack(stack)?;
    if part {
        stack.push(Vec::new());
    }
    let last = stack.len() - 1;
    for line in lines.iter().skip(1) {
        let info: Vec<_> = line.split(' ').collect();
        let how_many = info[1].parse::<i32>()?;
        let from = info[3].parse::<usize>()? - 1;
        let to = info[5].parse::<usize>()? - 1;
        for _ in 0..how_many {
            // println!("Moving {} to {} on stack: \n{}\n\n", from, to, stack_str(&stack));
            move_elem(&mut stack, from, if part { last } else { to })?;
        }
        if part {
            for _ in 0..how_many {
                move_elem(&mut stack, last, to)?;
            }
        }
    }
    
    Ok(stack.iter().map(|s| s.get((s.len() as i32 - 1) as usize).map(char::to_string).unwrap_or(String::new())).collect::<String>())
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap_or("".to_owned());
    let lines = contents.lines().collect();
    println!("{}", part12(lines, true).unwrap_or_else(|err| format!("{:?}", err)));
}
