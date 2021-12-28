fn main() {
    let mut state = [0; 9];
    include_str!("input.txt").trim().split(",").for_each(|s| {
        state[s.parse::<usize>().unwrap()] += 1;
    });

    for _ in 0..80 {
        run(&mut state);
    }
    println!("state: {:?}, ans: {}", state, state.iter().sum::<i32>())
}

fn run(state: &mut [i32; 9]) {
    let mut new_state = [0; 9];
    new_state[8] = state[0];
    new_state[6] = state[0];
    for i in 0..8 {
        new_state[i] += state[i + 1];
    }
    state.swap_with_slice(&mut new_state[0..9]);
}
