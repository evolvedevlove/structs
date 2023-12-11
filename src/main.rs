struct Skier {
    active: bool,
    cool_name: String,
    secret_hand_shake: String,
    days_on_hill: u64,
}

struct Mountain {
    open: bool,
    nick_name: String,
    cost: f32,
    height: u64,
}

impl Mountain{
    fn cost_per_meter(&self) -> f32 {
        self.cost / self.height as f32
    }
}

fn build_skier(cool_name: String, secret_hand_shake: String) -> Skier {
    Skier {
        active: true,
        cool_name: cool_name,
        secret_hand_shake: secret_hand_shake,
        days_on_hill: 1,
    }
}

fn generate_mountain() -> Mountain {
    Mountain {
        open: true,
        nick_name: String::from("temporary shred land"),
        cost: 100.23,
        height: 2500,
    }
}

fn main() {
    println!("Hello, skier!");
    // 1 - create a skier struct with four parameters
    let mut skier = Skier {
        active: true,
        cool_name: String::from("p-dawg"),
        secret_hand_shake: String::from("up left b b a"),
        days_on_hill: 69
    };
    println!("skier cool name is {}", skier.cool_name);
    skier.secret_hand_shake = String::from("be the best you can be!");

    // 2 - create a second (instance?) of the skier struct and compare it to the first
    let mut skier2 = build_skier(String::from("toodaloo"), String::from("bump slap poke"));
    println!("skier cool name is {}", skier2.cool_name);
    if skier.active &&  skier2.active{
        println!("skier has {} days on the hill and skier2 has {} days on the hill", skier.days_on_hill, skier2.days_on_hill);
    }

    // 3 - create a hill for the two skiers to shred
    let generic_mountain = generate_mountain();
    let sunshine = Mountain {
        open: generic_mountain.open,
        nick_name: String::from("sunny"),
        cost: generic_mountain.cost,
        height: 1070,
    };
    println!("the best hill is {} and it is {} tall", sunshine.nick_name, sunshine.height);

    // 4 - make both skiers go to a hill together (increment their days on hill)
    println!("yo {}, do you wanna go to {}?", skier.cool_name, sunshine.nick_name);
    println!("ya {}, i wanna shred that {}m tall mountain", skier2.cool_name, sunshine.height);
    skier.days_on_hill += 1;
    skier2.days_on_hill += 1;
    println!("{} {}", skier.days_on_hill, skier2.days_on_hill);

    println!(
        "The cost per meter ratio for this hill is: {}",
        sunshine.cost_per_meter()
     );
}
