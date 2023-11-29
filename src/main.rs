struct Skier {
    active: bool,
    cool_name: String,
    secret_hand_shake: String,
    days_on_hill: u64,
}

fn build_skier(cool_name: String, secret_hand_shake: String) -> Skier {
    Skier {
        active: true,
        cool_name: cool_name,
        secret_hand_shake: secret_hand_shake,
        days_on_hill: 1,
    }
}

fn main() {
    println!("Hello, world!");
    let mut skier = Skier {
        active: true,
        cool_name: String::from("p-dawg"),
        secret_hand_shake: String::from("up left b b a"),
        days_on_hill: 69
    };
    println!("skier cool name is {}", skier.cool_name);
    skier.secret_hand_shake = String::from("be the best you can be!");

    let skier2 = build_skier(String::from("toodaloo"), String::from("bump slap poke"));
    println!("skier cool name is {}", skier2.cool_name);
    if skier.active &&  skier2.active{
        println!("skier has {} days on the hill and skier2 has {} days on the hill", skier.days_on_hill, skier2.days_on_hill);
    }
    
}
