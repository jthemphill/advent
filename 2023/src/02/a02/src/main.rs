fn legal_game_id(game: &str) -> Option<i32> {
    let num_red = 12;
    let num_green = 13;
    let num_blue = 14;

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
            if color == "red" && num_cubes > num_red {
                return None;
            }
            if color == "green" && num_cubes > num_green {
                return None;
            }
            if color == "blue" && num_cubes > num_blue {
                return None;
            }
        }
    }

    println!("Game {}", id);
    Some(id)
}

fn main() {
    let mut sum_legal_games = 0;
    for line in std::io::stdin().lines() {
        if let Some(id) = legal_game_id(line.unwrap().trim()) {
            sum_legal_games += id;
        }
    }
    println!("Legal: {}", sum_legal_games);
}
