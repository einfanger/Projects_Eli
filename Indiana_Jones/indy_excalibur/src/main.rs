use std::io::{self, Write};

struct Player {
    hp: i32,
    inv: Vec<String>,
    ghosts_beaten: bool,
    snakes_beaten: bool,
    has_excalibur: bool,
}

fn main() {
    println!("=== Indiana Jones and the Quest for Excalibur ===");
    println!("Fedora on. Whip ready. Trauma pending.\n");

    let mut p = Player {
        hp: 100,
        inv: vec!["Whip".to_string(), "Medkit".to_string()],
        ghosts_beaten: false,
        snakes_beaten: false,
        has_excalibur: false,
    };

    loop {
        println!("\nHP: {} | Inventory: {}", p.hp, p.inv.len());
        println!("1) Enter the chamber");
        println!("2) Check inventory");
        println!("3) Use item");
        println!("4) Quit");

        match read("> ").as_str() {
            "1" => chamber(&mut p),
            "2" => show_inv(&p.inv),
            "3" => use_item(&mut p),
            "4" => { println!("You leave. Excalibur stays stuck. Honestly fair."); break; }
            _ => println!("Pick 1–4. Indy can’t parse chaos."),
        }

        if p.hp <= 0 {
            println!("\nYou drop to the floor. The dungeon wins. The snakes are thrilled.");
            break;
        }
        if p.has_excalibur {
            println!("\n🏆 YOU GOT EXCALIBUR 🏆");
            println!("You walk out a legend. Somewhere, a museum curator screams.");
            break;
        }
    }
}

fn chamber(p: &mut Player) {
    println!("\n--- THE CHAMBER ---");
    println!("A hallway splits into three ominous vibes:");
    println!("1) Snake tunnel");
    println!("2) Haunted chapel");
    println!("3) Sword-in-stone room (final)");
    println!("4) Back");

    match read("> ").as_str() {
        "1" => snakes(p),
        "2" => ghosts(p),
        "3" => finale(p),
        "4" => println!("You back out slowly. Respect."),
        _ => println!("The dungeon sighs. Try again."),
    }
}

fn snakes(p: &mut Player) {
    println!("\nYou hear a hiss. Then 400 more hisses.");
    println!("Snakes. So many snakes.");
    println!("1) Fight with whip");
    println!("2) Run");

    match read("> ").as_str() {
        "1" => {
            println!("You go full whip-mode. It works, but you get tagged.");
            p.hp -= 15;
            p.snakes_beaten = true;
            if !has(&p.inv, "Holy Charm") {
                println!("You find a Holy Charm on a cracked statue.");
                add(&mut p.inv, "Holy Charm");
            }
        }
        "2" => {
            println!("You run. Fast. Heroic? No. Effective? Yes.");
            p.hp -= 5;
        }
        _ => println!("You freeze. The snakes judge you for it."),
    }
}

fn ghosts(p: &mut Player) {
    println!("\nCandles light on their own. That’s never a good sign.");
    println!("A ghost knight appears and points a sword at your soul.");
    println!("1) Fight");
    println!("2) Use Holy Charm (if you have it)");
    println!("3) Run");

    match read("> ").as_str() {
        "1" => {
            println!("You fight the ghost. That sentence is already bad.");
            p.hp -= 20;
            p.ghosts_beaten = true;
        }
        "2" => {
            if has(&p.inv, "Holy Charm") {
                println!("The charm flares. The ghost evaporates like it saw your browser history.");
                p.ghosts_beaten = true;
            } else {
                println!("You reach for a charm you don’t have. The ghost bonks you.");
                p.hp -= 10;
            }
        }
        "3" => {
            println!("You sprint out. The ghost laughs. You hate that.");
            p.hp -= 5;
        }
        _ => println!("The ghost waits. Menacingly."),
    }
}

fn finale(p: &mut Player) {
    println!("\n--- THE SWORD-IN-STONE ROOM ---");
    println!("A stone pedestal. A blade hilt. The air feels… judgey.");
    println!("A voice whispers: \"Prove you’re worthy.\"");

    println!("\nA rival treasure hunter steps out of the shadows.");
    println!("1) Fight");
    println!("2) Run (coward route, but valid)");
    let c = read("> ");
    if c == "1" {
        println!("You scrap. It’s messy. You win, but take a hit.");
        p.hp -= 15;
    } else if c == "2" {
        println!("You dodge past him like a football highlight.");
        p.hp -= 5;
    } else {
        println!("You hesitate. He punches you. Classic.");
        p.hp -= 10;
    }

    if p.hp <= 0 { return; }

    println!("\nNow the choice:");
    println!("1) Pull the sword");
    println!("2) Step back");

    match read("> ").as_str() {
        "1" => {
            if p.ghosts_beaten || (p.snakes_beaten && has(&p.inv, "Whip")) {
                println!("The stone loosens… the blade slides free.");
                add(&mut p.inv, "Excalibur");
                p.has_excalibur = true;
            } else {
                println!("The stone refuses. The room is like: \"nah.\"");
                p.hp -= 10;
                println!("A backlash hits you. (-10 HP)");
            }
        }
        "2" => println!("You step away. The dungeon approves your humility."),
        _ => println!("The sword does not respond to nonsense."),
    }
}

fn use_item(p: &mut Player) {
    show_inv(&p.inv);
    let item = read("Type item name (or back): ");
    if item.eq_ignore_ascii_case("back") { return; }

    if !has(&p.inv, &item) {
        println!("You don’t have that.");
        return;
    }

    match item.as_str() {
        "Medkit" => {
            println!("You patch yourself up. You are now 20% less doomed.");
            p.hp += 25;
            if p.hp > 100 { p.hp = 100; }
            remove(&mut p.inv, "Medkit");
        }
        "Holy Charm" => println!("It glows faintly. Ghosts probably hate this."),
        "Whip" => println!("You twirl the whip. Style points: max."),
        _ => println!("You mess with it. Nothing happens."),
    }
}

fn show_inv(inv: &Vec<String>) {
    println!("\nInventory:");
    if inv.is_empty() { println!("(empty)"); }
    else { for i in inv { println!("- {}", i); } }
}

fn add(inv: &mut Vec<String>, item: &str) {
    inv.push(item.to_string());
    println!("✅ Added: {}", item);
}

fn has(inv: &Vec<String>, item: &str) -> bool {
    inv.iter().any(|x| x == item)
}

fn remove(inv: &mut Vec<String>, item: &str) {
    if let Some(i) = inv.iter().position(|x| x == item) {
        inv.remove(i);
    }
}

fn read(prompt: &str) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}