fn game_power(game: &str) -> i32 {
    let mut num_red = 0;
    let mut num_green = 0;
    let mut num_blue = 0;

    let mut game = game.split(": ");
    let mut game_id = game.next().unwrap().split("Game ");
    game_id.next();
    let id = game_id.next().unwrap();
    let id = id.parse::<i32>().unwrap();

    let all_rounds = game.next().unwrap();
    for round in all_rounds.split("; ").into_iter() {
        for cube_color in round.split(", ").into_iter() {
            let mut cube_color = cube_color.split(" ");
            let num_cubes = cube_color.next().unwrap();
            let num_cubes = num_cubes.parse::<i32>().unwrap();
            let color = cube_color.next().unwrap();
            if color == "red" {
                num_red = num_red.max(num_cubes);
            }
            if color == "green" {
                num_green = num_green.max(num_cubes);
            }
            if color == "blue" {
                num_blue = num_blue.max(num_cubes);
            }
        }
    }

    let power = num_red * num_green * num_blue;
    println!("Game {}: {}", id, power);
    power
}

fn main() {
    let mut sum_power = 0;
    for line in std::io::stdin().lines() {
        sum_power += game_power(line.unwrap().trim());
    }
    println!("Power: {}", sum_power);
}
