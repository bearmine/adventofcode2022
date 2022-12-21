use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct RobotType {
    robot_type: String,
    ore_cost: i32,
    clay_cost: i32,
    obsidian_cost: i32,
}

impl RobotType {
    pub fn new(robot_type: String, ore_cost: i32, clay_cost: i32, obsidian_cost: i32) -> Self {
        Self {
            robot_type,
            ore_cost,
            clay_cost,
            obsidian_cost,
        }
    }
}

impl fmt::Display for RobotType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Simple pipe delimited formatting
        write!(
            f,
            "{}|{}|{}|{}",
            self.robot_type, self.ore_cost, self.clay_cost, self.obsidian_cost
        )
    }
}

struct Blueprint {
    blueprint_number: i32,
    ore_robot: RobotType,
    clay_robot: RobotType,
    obsidian_robot: RobotType,
    geode_robot: RobotType,
}

impl Blueprint {
    pub fn new(
        blueprint_number: i32,
        ore_robot: RobotType,
        clay_robot: RobotType,
        obsidian_robot: RobotType,
        geode_robot: RobotType,
    ) -> Self {
        Self {
            blueprint_number,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Simple pipe delimited formatting
        write!(
            f,
            "Blueprint {}:\n  {}\n  {}\n  {}\n  {}",
            self.blueprint_number,
            self.ore_robot,
            self.clay_robot,
            self.obsidian_robot,
            self.geode_robot
        )
    }
}

fn blueprint_from_string(blueprint_string: &str) -> Result<Blueprint, &str> {
    // Create regex for matching groups
    let blueprint_regex: Regex = Regex::new(
        "Blueprint ([0-9]+): \
        Each ore robot costs ([0-9]+) ore. \
        Each clay robot costs ([0-9]+) ore. \
        Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. \
        Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.",
    )
    .unwrap();

    // Attempt to match against given string
    let regex_match = blueprint_regex.captures(blueprint_string);
    return match regex_match {
        Some(m) => {
            // We matched, so build a new blueprint
            Ok(Blueprint::new(
                m.get(1).unwrap().as_str().parse().unwrap(),
                RobotType::new(
                    "Ore".to_owned(),
                    m.get(2).unwrap().as_str().parse().unwrap(),
                    0,
                    0,
                ),
                RobotType::new(
                    "Clay".to_owned(),
                    m.get(3).unwrap().as_str().parse().unwrap(),
                    0,
                    0,
                ),
                RobotType::new(
                    "Obsidian".to_owned(),
                    m.get(4).unwrap().as_str().parse().unwrap(),
                    m.get(5).unwrap().as_str().parse().unwrap(),
                    0,
                ),
                RobotType::new(
                    "Geode".to_owned(),
                    m.get(6).unwrap().as_str().parse().unwrap(),
                    0,
                    m.get(7).unwrap().as_str().parse().unwrap(),
                ),
            ))
        }
        // If we didn't match, throw and Err
        None => Err("Blueprint string doesn't match"),
    };
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let blueprint = blueprint_from_string(&line).unwrap();
        println!("{}", blueprint)
    }
}
