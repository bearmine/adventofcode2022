use regex::Regex;
use std::fmt;

pub(crate) enum RobotType {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot
}

impl RobotType {
    pub fn get_name(&self) -> &str {
        match self {
            RobotType::OreRobot => "Ore Robot",
            RobotType::ClayRobot => "Clay Robot",
            RobotType::ObsidianRobot => "Obsidian Robot",
            RobotType::GeodeRobot => "Geode Robot",
        }
    }
}

pub(crate) struct RobotBlueprint {
    pub(crate) robot_type: RobotType,
    pub(crate) ore_cost: i32,
    pub(crate) clay_cost: i32,
    pub(crate) obsidian_cost: i32,
}

impl RobotBlueprint {
    pub fn new(robot_type: RobotType, ore_cost: i32, clay_cost: i32, obsidian_cost: i32) -> Self {
        Self {
            robot_type,
            ore_cost,
            clay_cost,
            obsidian_cost,
        }
    }
}

impl fmt::Display for RobotBlueprint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Simple pipe delimited formatting
        write!(
            f,
            "{}|{}|{}|{}",
            self.robot_type.get_name(), self.ore_cost, self.clay_cost, self.obsidian_cost
        )
    }
}

pub(crate) struct Blueprint {
    pub(crate) blueprint_number: i32,
    pub(crate) ore_robot: RobotBlueprint,
    pub(crate) clay_robot: RobotBlueprint,
    pub(crate) obsidian_robot: RobotBlueprint,
    pub(crate) geode_robot: RobotBlueprint,
}

impl Blueprint {
    pub fn new(
        blueprint_number: i32,
        ore_robot: RobotBlueprint,
        clay_robot: RobotBlueprint,
        obsidian_robot: RobotBlueprint,
        geode_robot: RobotBlueprint,
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

pub(crate) fn blueprint_from_string(blueprint_string: &str) -> Result<Blueprint, &str> {
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
                RobotBlueprint::new(
                    RobotType::OreRobot,
                    m.get(2).unwrap().as_str().parse().unwrap(),
                    0,
                    0,
                ),
                RobotBlueprint::new(
                    RobotType::ClayRobot,
                    m.get(3).unwrap().as_str().parse().unwrap(),
                    0,
                    0,
                ),
                RobotBlueprint::new(
                    RobotType::ObsidianRobot,
                    m.get(4).unwrap().as_str().parse().unwrap(),
                    m.get(5).unwrap().as_str().parse().unwrap(),
                    0,
                ),
                RobotBlueprint::new(
                    RobotType::GeodeRobot,
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
