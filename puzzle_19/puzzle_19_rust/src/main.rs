use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use robot::{Blueprint, RobotBlueprint, RobotType};

mod robot;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut blueprint_list: Vec<Blueprint> = Vec::new();

    // parse input.txt file
    println!("--Loading Blueprints--");
    for line in reader.lines() {
        let line = line.unwrap();
        let blueprint = robot::blueprint_from_string(&line).unwrap();
        println!("{}", blueprint);
        blueprint_list.push(blueprint);
    }
    println!("--Finished Loading Blueprints--");
    let mut scores: Vec<String> = Vec::new();
    let mut total_quality = 0;
    for blueprint in blueprint_list {
        let blueprint_number = blueprint.blueprint_number;
        let solution = solve_blueprint(blueprint, 24).unwrap();
        println!("{}", solution);
        let quality = blueprint_number * solution.geode_amount;
        scores.push(format!("Blueprint {}: {}", blueprint_number, quality));
        total_quality += quality;
    }
    for score in scores {
        println!("{}", score);
    }
    println!("Total Quality: {}", total_quality);
}

pub(crate) struct TurnState {
    pub(crate) current_minute: i32,
    pub(crate) ore_amount: i32,
    pub(crate) clay_amount: i32,
    pub(crate) obsidian_amount: i32,
    pub(crate) geode_amount: i32,
    pub(crate) ore_robots: i32,
    pub(crate) clay_robots: i32,
    pub(crate) obsidian_robots: i32,
    pub(crate) geode_robots: i32,
}

impl TurnState {
    pub fn new() -> Self {
        Self {
            current_minute: 0,
            ore_amount: 0,
            clay_amount: 0,
            obsidian_amount: 0,
            geode_amount: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    /// Progress to a specific minute. Must be a minute in the future.
    pub fn progress_to_minute(&self, minute: i32) -> Self {
        //println!("progress_to_minute({}) | {}", minute, self.current_minute);
        assert!(minute >= self.current_minute);
        self.progress(minute - self.current_minute)
    }

    /// Progress a number of minutes but don't add any robots
    pub fn progress(&self, number_of_minutes: i32) -> Self {
        Self {
            current_minute: self.current_minute + number_of_minutes,
            ore_amount: self.ore_amount + self.ore_robots * number_of_minutes,
            clay_amount: self.clay_amount + self.clay_robots * number_of_minutes,
            obsidian_amount: self.obsidian_amount + self.obsidian_robots * number_of_minutes,
            geode_amount: self.geode_amount + self.geode_robots * number_of_minutes,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        }
    }

    /// Progress a number of minutes and add a robot at the end
    pub fn progress_and_add_robot(
        &self,
        number_of_minutes: i32,
        robot_to_add: &RobotBlueprint,
    ) -> Self {
        Self {
            current_minute: self.current_minute + number_of_minutes,
            ore_amount: self.ore_amount + self.ore_robots * number_of_minutes
                - robot_to_add.ore_cost,
            clay_amount: self.clay_amount + self.clay_robots * number_of_minutes
                - robot_to_add.clay_cost,
            obsidian_amount: self.obsidian_amount + self.obsidian_robots * number_of_minutes
                - robot_to_add.obsidian_cost,
            geode_amount: self.geode_amount + self.geode_robots * number_of_minutes,
            ore_robots: if matches!(robot_to_add.robot_type, RobotType::OreRobot) {
                self.ore_robots + 1
            } else {
                self.ore_robots
            },
            clay_robots: if matches!(robot_to_add.robot_type, RobotType::ClayRobot) {
                self.clay_robots + 1
            } else {
                self.clay_robots
            },
            obsidian_robots: if matches!(robot_to_add.robot_type, RobotType::ObsidianRobot) {
                self.obsidian_robots + 1
            } else {
                self.obsidian_robots
            },
            geode_robots: if matches!(robot_to_add.robot_type, RobotType::GeodeRobot) {
                self.geode_robots + 1
            } else {
                self.geode_robots
            },
        }
    }
}

impl Clone for TurnState {
    fn clone(&self) -> Self {
        Self {
            current_minute: self.current_minute.clone(),
            ore_amount: self.ore_amount.clone(),
            clay_amount: self.clay_amount.clone(),
            obsidian_amount: self.obsidian_amount.clone(),
            geode_amount: self.geode_amount.clone(),
            ore_robots: self.ore_robots.clone(),
            clay_robots: self.clay_robots.clone(),
            obsidian_robots: self.obsidian_robots.clone(),
            geode_robots: self.geode_robots.clone(),
        }
    }
}

impl fmt::Display for TurnState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TurnState(\n  current_minute: {}\n  ore_amount: {}\n  clay_amount: {}\n  obsidian_amount: {}\n  geode_amount: {}\n  ore_robots: {}\n  clay_robots: {}\n  obsidian_robots: {}\n  geode_robots: {}\n)",
            self.current_minute,
            self.ore_amount,
            self.clay_amount,
            self.obsidian_amount,
            self.geode_amount,
            self.ore_robots,
            self.clay_robots,
            self.obsidian_robots,
            self.geode_robots,
        )
    }
}

fn solve_blueprint(blueprint: Blueprint, max_depth_minutes: i32) -> Option<TurnState> {
    println!("Solving {}", blueprint);
    let start_turn = TurnState::new();
    let result = solver_branch(&blueprint, max_depth_minutes, &start_turn);
    return result;
}

fn solver_branch(
    blueprint: &Blueprint,
    max_depth_minutes: i32,
    state: &TurnState,
) -> Option<TurnState> {

    let ore_result = solve_turn(&blueprint, max_depth_minutes, &state, RobotType::OreRobot);
    let clay_result = solve_turn(&blueprint, max_depth_minutes, &state, RobotType::ClayRobot);
    let obsidian_result = solve_turn(
        &blueprint,
        max_depth_minutes,
        &state,
        RobotType::ObsidianRobot,
    );
    let geode_result = solve_turn(&blueprint, max_depth_minutes, &state, RobotType::GeodeRobot);

    // Add all result to a vec if they aren't None
    let mut result_list: Vec<TurnState> = Vec::new();
    if let Some(r) = ore_result {
        result_list.push(r)
    }

    if let Some(r) = clay_result {
        result_list.push(r)
    }

    if let Some(r) = obsidian_result {
        result_list.push(r)
    }

    if let Some(r) = geode_result {
        result_list.push(r)
    }

    let mut best_result: Option<TurnState> = None;
    for r in result_list.iter() {
        if let Some(br) = &best_result {
            if r.geode_amount > br.geode_amount {
                best_result = Some(r.clone());
            }
        } else {
            best_result = Some(r.clone());
        }
    }

    return best_result;
}

fn solve_turn(
    blueprint: &Blueprint,
    max_depth_minutes: i32,
    state: &TurnState,
    focus: RobotType,
) -> Option<TurnState> {
    // Apply focus
    let turn_progress = match focus {
        RobotType::OreRobot => {
            solve_for_robot(blueprint, max_depth_minutes, state, &blueprint.ore_robot)
        }
        RobotType::ClayRobot => {
            solve_for_robot(blueprint, max_depth_minutes, state, &blueprint.clay_robot)
        }
        RobotType::ObsidianRobot => solve_for_robot(
            blueprint,
            max_depth_minutes,
            state,
            &blueprint.obsidian_robot,
        ),
        RobotType::GeodeRobot => {
            solve_for_robot(blueprint, max_depth_minutes, state, &blueprint.geode_robot)
        }
    };

    match turn_progress {
        Some(turn) => {
            if turn.current_minute > max_depth_minutes {
                // We hit our depth, so backup and progress to end depth
                Some(state.progress_to_minute(max_depth_minutes))
            } else {
                // Back to branching and a new rounds of turns
                solver_branch(&blueprint, max_depth_minutes, &turn)
            }
        }
        // Impossible to make focused rebot to end chain
        None => None,
    }
}

fn time_to_robot(current_state: &TurnState, robot_to_build: &RobotBlueprint) -> Option<i32> {
    let ore_needed = robot_to_build.ore_cost - current_state.ore_amount;
    let clay_needed = robot_to_build.clay_cost - current_state.clay_amount;
    let obsidian_needed = robot_to_build.obsidian_cost - current_state.obsidian_amount;

    let ore_num_turns = if ore_needed > 0 {
        if current_state.ore_robots > 0 {
            // Calc how many turns to get enough ore
            (ore_needed as f32 / current_state.ore_robots as f32).ceil() as i32
        } else {
            // Impossible to get enough ore
            return None;
        }
    } else {
        0
    };

    let clay_num_turns = if clay_needed > 0 {
        if current_state.clay_robots > 0 {
            // calc how many turns to get enough clay
            (clay_needed as f32 / current_state.clay_robots as f32).ceil() as i32
        } else {
            // Impossible to get enough clay
            return None;
        }
    } else {
        0
    };

    let obsidian_num_turns = if obsidian_needed > 0 {
        if current_state.obsidian_robots > 0 {
            // calc how many turns to get enough obsidian
            (obsidian_needed as f32 / current_state.obsidian_robots as f32).ceil() as i32
        } else {
            // Impossible to get enough obsidian
            return None;
        }
    } else {
        0
    };

    // return the max number of turns needed
    let turns_to_progress = max(max(ore_num_turns, clay_num_turns), obsidian_num_turns);
    //println!("Turns to Build Robot: {}", turns_to_progress);
    if turns_to_progress == 0 {
        // We should always progress 1 turn
        return Some(1);
    }
    return Some(turns_to_progress);
}

fn solve_for_robot(
    blueprint: &Blueprint,
    max_depth_minutes: i32,
    current_state: &TurnState,
    robot: &RobotBlueprint,
) -> Option<TurnState> {
    let number_of_minutes = time_to_robot(current_state, robot);
    match number_of_minutes {
        Some(n) => Some(current_state.progress_and_add_robot(n, robot)),
        None => None,
    }
}
