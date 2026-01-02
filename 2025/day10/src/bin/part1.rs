use day10::{get_manual, InputType, Machine, Manual};

const INF: u32 = 1_000_000_000;


fn button_to_bitmask(button: &Vec<usize>) ->usize {
    button.iter().fold(0, |acc, &idx| acc | (1 << idx))
}

fn light_to_bitmask(lights: &Vec<bool>) -> usize {
    lights.iter().enumerate().fold(0, |acc, (idx, &state)| {
        if state {
            acc | (1 << idx)
        } else {
            acc
        }
    })
}

fn solve_machine(machine: &Machine) -> u32 {
    let lights = machine.lights.len();
    let buttons = machine.buttons.len();
    let mut dp = vec![vec![INF; 1 << lights]; buttons];

    dp[0][0] = 0;
    dp[0][button_to_bitmask(&machine.buttons[0])] = 1;

    for i in 1..buttons {
        let button_mask = button_to_bitmask(&machine.buttons[i]);
        for light_state in 0..(1 << lights) {
            dp[i][light_state] = dp[i - 1][light_state].min(dp[i - 1][light_state ^ button_mask] + 1);
        }
    }

    dp[buttons - 1][light_to_bitmask(&machine.lights)]
}

fn main() {
    let manual = get_manual(InputType::Sample).expect("Failed to get manual");


    let res = manual.machines.iter().fold(0, |sum, machine| {
        sum + solve_machine(machine)
    });

    println!("Result: {}", res);
}
