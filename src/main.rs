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

struct Gondola(i64, u64);
struct Chair(u64, u64);

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
    // there is a better way to do the below and i will do it i promise
    println!("now {} has been on the hill {} and {} has been on the hill {}", skier.cool_name, skier.days_on_hill, skier2.cool_name, skier2.days_on_hill );

    println!(
        "The cost per meter ratio for this hill is: {}",
        sunshine.cost_per_meter()
     );

     // 5 - create new mountain and update it to resuse data from generic mountain
     let lake_louise = Mountain {
        nick_name: String::from("louie louie"),
        ..generic_mountain
     };
     
     println!("{}, {}", lake_louise.cost, lake_louise.nick_name);

    // 6 - make a gondola for sunshine 
    // there must be the origin height is -500 and the base camp is 0
    let up_to_base = Gondola(-500, 0);
    println!("sunshine gondola starts {} below the base camp", up_to_base.0);

    // 7 - make a gondola for lake louise where the bottom is 0 and the top is the top of the mountain
    let to_the_top = Gondola(0, lake_louise.height);
    println!("louise has a gondola that goes up to the top which is {} high", to_the_top.1);
    
    // 8 - make a chair for sunshine that starts at base and goes to the top of the mountain
    let angel = Chair(0, sunshine.height);
    println!("sunshine has a chair that goes up to top which is {} tall", angel.1);

    // 9 - make a chair for lake louise that starts at 800 and goes +50 above the peak (the new chair!)
    let juniper = Chair(800, lake_louise.height + 50);
    println!("louise has a chair that goes 50 above the top which is {} high total", juniper.1);
    
    // now we start to encounter abstraction problems ---
    // should the mountain have a list of chairs and a list of gondolas on it?
    // probably - lets do that in the next session    
}
