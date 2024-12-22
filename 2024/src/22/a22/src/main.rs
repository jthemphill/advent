use std::io::Read;

fn mix(a: usize, b: usize) -> usize {
    a ^ b
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn evolve(secret: usize) -> usize {
    let secret = mix(secret, secret * 64);
    let secret = prune(secret);

    let secret = mix(secret, secret / 32);
    let secret = prune(secret);

    let secret = mix(secret, secret * 2048);
    let secret = prune(secret);

    secret
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let secrets: Vec<usize> = input
        .split('\n')
        .map(|secret| secret.parse().unwrap())
        .collect();

    let mut total = 0;
    for secret in secrets {
        let mut evolved = secret;
        for _ in 0..2000 {
            evolved = evolve(evolved);
        }
        total += evolved;
        println!("{secret}: {evolved}");
    }
    println!("{total}");
}
