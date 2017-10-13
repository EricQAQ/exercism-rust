use std::convert::Into;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Team {
    name: String,
    win: u32,
    draw: u32,
    loss: u32
}

impl Team {
    fn new<T: Into<String>>(name: T, win: u32, draw: u32, loss: u32) -> Self {
        Team { name: name.into(), win: win, draw: draw, loss: loss }
    }

    fn parse_line(line: &str, target: &mut HashMap<String, Self>) {
        let list = line.split(";")
            .map(|slice| slice.to_string())
            .collect::<Vec<String>>();
        if list.len() != 3 { return }
        match &*list[2] {
            "win" => {
                target.entry(list[0].clone())
                        .or_insert(Team::new(list[0].clone(), 0, 0, 0)).win += 1;
                target.entry(list[1].clone())
                        .or_insert(Team::new(list[1].clone(), 0, 0, 0)).loss += 1;
            },
            "draw" => {
                target.entry(list[0].clone()).or_insert(Team::new(list[0].clone(), 0, 0, 0)).draw += 1;
                target.entry(list[1].clone()).or_insert(Team::new(list[1].clone(), 0, 0, 0)).draw += 1;
            }
            "loss" => {
                target.entry(list[0].clone()).or_insert(Team::new(list[0].clone(), 0, 0, 0)).loss += 1;
                target.entry(list[1].clone()).or_insert(Team::new(list[1].clone(), 0, 0, 0)).win += 1;
            }
            _ => {},
        }
    }
}

fn convert(target: &HashMap<String, Team>) -> String {
    let mut res = format!("{:30} | MP |  W |  D |  L |  P\n", "Team");
    let mut list = target.values().cloned().collect::<Vec<Team>>();
    list.sort_by(|x, y| match (x.win * 3 + x.draw).cmp(&(y.win * 3 + y.draw)).reverse() {
        Ordering::Equal => x.name.cmp(&(y.name)),
        other => other
    });
    for team in list.iter() {
        let line = format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}\n",
            team.name,
            team.win + team.draw + team.loss,
            team.win,
            team.draw,
            team.loss,
            team.win * 3 + team.draw
        );
        res.push_str(&line);
    }
    res.trim().to_string()
}

pub fn tally(input: &str) -> String {
    let list = input.split("\n").collect::<Vec<&str>>();
    let mut res: HashMap<String, Team> = HashMap::new();
    for &line in list.iter() {
        Team::parse_line(line, &mut res);
    }
    convert(&res)
}
