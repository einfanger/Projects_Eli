use std::io::{self, Write};

#[derive(Clone)]
struct Player {
    hp: i32,
    inv: Vec<String>,
    visited: Visited,
    has_excalibur: bool,
}

#[derive(Clone)]
struct Visited {
    entrance: bool,
    snake_crypt: bool,
    haunted_chapel: bool,
    river_tunnel: bool,
    sword_chamber: bool,
}

#[derive(Clone, Copy)]
enum Room {
    EntranceHall,
    SnakeCrypt,
    HauntedChapel,
    RiverTunnel,
    SwordChamber,
}

fn main() {
    println!("=== Indiana Jones and the Quest for Excalibur ===");
    println!("Rain hammers the stone ruins above you.");
    println!("Your boots splash through mud as you descend into the unknown.\n");

let name = "Indiana Jones".to_string();

println!("\nYou are {}.", name);
println!("Archaeologist. Explorer. Professional bad-decision maker.");
println!("Your flashlight clicks on… then flickers. Classic.");
println!("You tighten your grip on the whip at your belt.\n");

    println!("\nAlright, {}.", name);
    println!("Your flashlight clicks on… then flickers. Classic.");
    println!("You tighten your grip on the whip at your belt.\n");

    let mut player = Player {
        hp: 100,
        inv: vec!["Whip".to_string(), "Medkit".to_string()],
        visited: Visited {
            entrance: false,
            snake_crypt: false,
            haunted_chapel: false,
            river_tunnel: false,
            sword_chamber: false,
        },
        has_excalibur: false,
    };

    let mut room = Room::EntranceHall;

    loop {
        if player.hp <= 0 {
            println!("\nYour vision blurs. Your knees hit stone.");
            println!("The ruins don’t care who you are.");
            println!("Excalibur remains lost… and the darkness closes in.");
            break;
        }
        if player.has_excalibur {
            println!("\n🏆 YOU WON 🏆");
            println!("Excalibur is in your hands—cold, heavy, *real*.");
            println!("For a moment, the entire ruin feels… quiet. Like it’s honoring you.");
            println!("You climb back into the rain as a legend.");
            break;
        }

        show_status(&player);

        room = match room {
            Room::EntranceHall => entrance_hall(&name, &mut player),
            Room::SnakeCrypt => snake_crypt(&mut player),
            Room::HauntedChapel => haunted_chapel(&mut player),
            Room::RiverTunnel => river_tunnel(&mut player),
            Room::SwordChamber => sword_chamber(&name, &mut player),
        };
    }
}


fn entrance_hall(name: &str, p: &mut Player) -> Room {
    if !p.visited.entrance {
        p.visited.entrance = true;
        println!("\n--- ENTRANCE HALL ---");
        println!("A massive stone doorway yawns behind you, now swallowed by shadow.");
        println!("In front: an ancient corridor lined with cracked statues.");
        println!("One statue’s face looks almost… disappointed.");
        println!("A draft whispers through the hall like someone breathing.\n");
        println!("{} mutters: \"This better be worth it.\"", name);
    } else {
        println!("\n--- ENTRANCE HALL ---");
        println!("You’re back where the air is cold but familiar.");
        println!("The statues watch you like security cameras with trauma.");
    }

    println!("\nWhere do you go?");
    println!("1) Left passage (faint hissing sounds)");
    println!("2) Right passage (old chapel bells… but underground?)");
    println!("3) Down the stairs (you hear water)");
    println!("4) Use an item");
    println!("5) Quit (live to excavate another day)");

    match read("> ").as_str() {
        "1" => Room::SnakeCrypt,
        "2" => Room::HauntedChapel,
        "3" => Room::RiverTunnel,
        "4" => { use_item(p); Room::EntranceHall }
        "5" => {
            println!("\nYou turn back.");
            println!("Some treasures aren’t worth dying for.");
            std::process::exit(0);
        }
        _ => {
            println!("Your indecision echoes. The ruin waits.");
            Room::EntranceHall
        }
    }
}

fn snake_crypt(p: &mut Player) -> Room {
    println!("\n--- SNAKE CRYPT ---");

    if !p.visited.snake_crypt {
        p.visited.snake_crypt = true;
        println!("The passage narrows until the walls nearly brush your shoulders.");
        println!("Your flashlight beam lands on something moving.");
        println!("Not one thing.");
        println!("Many things.\n");
        println!("Snakes. Coiled. Layered. Breathing like a living carpet.");
        println!("The smell is ancient dust and fear.\n");
        println!("A stone pedestal sits beyond them, holding a rusted TORCH.");
    } else {
        println!("You return to the crypt. The hissing starts immediately.");
        println!("The snakes remember you. Unfortunately.");
    }

    if !has(&p.inv, "Torch") {
        println!("\nOptions:");
        println!("1) Grab the torch (risky)");
        println!("2) Fight through with the whip");
        println!("3) Back away slowly");
    } else {
        println!("\nOptions:");
        println!("1) Use torch to scare them back");
        println!("2) Fight through with the whip");
        println!("3) Back away slowly");
    }

    match read("> ").as_str() {
        "1" => {
            if !has(&p.inv, "Torch") {
                println!("\nYou step forward, carefully…");
                println!("A snake strikes like a spring-loaded nightmare.");
                p.hp -= 12;
                println!("You yank your hand back, grab the TORCH anyway. (-12 HP)");
                add(&mut p.inv, "Torch");
            } else {
                println!("\nYou raise the torch. The flame snaps and dances.");
                println!("The snakes recoil like you just turned on the world’s worst music.");
                println!("A path opens.");
            }
            Room::EntranceHall
        }
        "2" => {
            println!("\nYou crack the whip. The sound explodes in the tight tunnel.");
            println!("Snakes scatter. Some don’t.");
            p.hp -= 15;
            println!("You push through, bitten and annoyed. (-15 HP)");
            Room::EntranceHall
        }
        "3" => {
            println!("You back out. Quietly. Respectfully. Like a person who likes being alive.");
            Room::EntranceHall
        }
        _ => {
            println!("You hesitate. The snakes do not.");
            p.hp -= 8;
            println!("You get tagged. (-8 HP)");
            Room::EntranceHall
        }
    }
}

fn haunted_chapel(p: &mut Player) -> Room {
    println!("\n--- HAUNTED CHAPEL ---");

    if !p.visited.haunted_chapel {
        p.visited.haunted_chapel = true;
        println!("You step into a ruined chapel carved beneath the earth.");
        println!("As your boot touches the stone… candles ignite on their own.");
        println!("A low hum fills the air, like a choir in another room.\n");
        println!("A ghostly knight forms from mist, armor clanking with no body inside.");
        println!("It points a spectral blade at you.");
    } else {
        println!("The air is colder here. The candles relight as if expecting you.");
        println!("The ghost knight returns—patient, offended, and very stabby.");
    }

    println!("\nWhat do you do?");
    println!("1) Fight (whip)");
    println!("2) Use Holy Charm (if you have it)");
    println!("3) Run back");

    match read("> ").as_str() {
        "1" => {
            println!("\nYou swing the whip—");
            println!("It passes through the ghost like smoke, but the force disrupts it.");
            println!("The knight retaliates. Cold pain slams into your chest.");
            p.hp -= 18;
            println!("You grit your teeth and push forward. (-18 HP)");

            if !has(&p.inv, "Holy Charm") {
                println!("A small charm drops from the altar, glowing faintly.");
                add(&mut p.inv, "Holy Charm");
            }
            Room::EntranceHall
        }
        "2" => {
            if has(&p.inv, "Holy Charm") {
                println!("\nYou hold up the charm.");
                println!("Light blooms across the chapel.");
                println!("The ghost freezes… then fractures into mist, like it’s finally exhaling.");
                println!("Behind the altar, a stone door clicks—somewhere deeper, something opened.");
                Room::EntranceHall
            } else {
                println!("\nYou reach for a charm you don’t have.");
                println!("The ghost bonks your soul. Unfair.");
                p.hp -= 10;
                println!("(-10 HP)");
                Room::EntranceHall
            }
        }
        "3" => {
            println!("You retreat. The ghost does not chase you—just watches.");
            p.hp -= 5;
            println!("You trip on the way out because the ruin is petty. (-5 HP)");
            Room::EntranceHall
        }
        _ => {
            println!("You freeze.");
            println!("The ghost takes that personally.");
            p.hp -= 12;
            println!("(-12 HP)");
            Room::EntranceHall
        }
    }
}

fn river_tunnel(p: &mut Player) -> Room {
    println!("\n--- RIVER TUNNEL ---");

    if !p.visited.river_tunnel {
        p.visited.river_tunnel = true;
        println!("Stone steps descend into a tunnel where water runs like whispers.");
        println!("The floor is slick. Your flashlight reflects off black water.");
        println!("A rope bridge crosses a narrow underground river.");
        println!("On the far side: a heavy door etched with a sword-in-stone symbol.");
    } else {
        println!("You’re back at the underground river.");
        println!("The rope bridge sways like it’s nervous.");
    }

    println!("\nWhat do you do?");
    println!("1) Cross the bridge");
    println!("2) Search near the water");
    println!("3) Go back");

    match read("> ").as_str() {
        "1" => {
            println!("\nYou step onto the bridge.");
            println!("The rope groans.");
            println!("Halfway across… the bridge snaps a strand.");
            if has(&p.inv, "Whip") {
                println!("You snap the whip around a stone pillar and swing the rest of the way.");
                println!("Your heart is doing parkour, but you make it.");
                Room::SwordChamber
            } else {
                println!("You try to grab the rope—too late.");
                p.hp -= 30;
                println!("You slam into shallow water and crawl out, soaked. (-30 HP)");
                Room::EntranceHall
            }
        }
        "2" => {
            println!("\nYou kneel by the water, scanning the stones.");
            if !has(&p.inv, "Old Coin") {
                println!("You find an OLD COIN with a knight’s crest.");
                add(&mut p.inv, "Old Coin");
            } else {
                println!("You find… more wet rocks. Nice.");
            }
            Room::RiverTunnel
        }
        "3" => Room::EntranceHall,
        _ => {
            println!("You stand there and listen to water drip like a countdown.");
            Room::RiverTunnel
        }
    }
}

fn sword_chamber(name: &str, p: &mut Player) -> Room {
    println!("\n--- SWORD CHAMBER ---");

    if !p.visited.sword_chamber {
        p.visited.sword_chamber = true;
        println!("The heavy door scrapes open.");
        println!("A circular chamber breathes cold air.");
        println!("At the center: stone pedestal. Sword hilt. Silence.\n");
        println!("Then a voice: \"Only the worthy may claim the blade.\"");
    } else {
        println!("The chamber is still. Waiting.");
    }

    println!("\nA rival treasure hunter steps out of the shadows.");
    println!("He grins like he rehearsed it.");
    println!("\"You’re late,\" he says.\n");
    println!("1) Fight him");
    println!("2) Bluff (talk your way past)");
    println!("3) Run back");

    match read("> ").as_str() {
        "1" => {
            println!("\nYou lunge. He swings. You both stumble like action-movie professionals.");
            p.hp -= 12;
            println!("You win the scuffle, but take a hit. (-12 HP)");
        }
        "2" => {
            println!("\nYou talk fast: artifacts, curses, bad luck, ‘this place is rigged.’");
            println!("He hesitates just long enough for you to shoulder past.");
            p.hp -= 5;
            println!("He clips you on the way. (-5 HP)");
        }
        "3" => {
            println!("You retreat. The chamber’s silence follows you like a stare.");
            return Room::RiverTunnel;
        }
        _ => {
            println!("You pause. He punches you. Timing is everything.");
            p.hp -= 10;
        }
    }

    if p.hp <= 0 {
        return Room::SwordChamber;
    }

    println!("\nNow the pedestal.");
    println!("Your hand hovers over the sword hilt.");
    println!("You feel the weight of every bad decision you’ve ever made.\n");

    println!("Choose:");
    println!("1) Pull the sword");
    println!("2) Use an item first");
    println!("3) Step away");

    match read("> ").as_str() {
        "1" => attempt_excalibur(name, p),
        "2" => { use_item(p); Room::SwordChamber }
        "3" => {
            println!("You step away. Sometimes the bravest move is not dying today.");
            Room::RiverTunnel
        }
        _ => {
            println!("The sword does not respond to chaos.");
            Room::SwordChamber
        }
    }
}

fn attempt_excalibur(name: &str, p: &mut Player) -> Room {
    let worthy = has(&p.inv, "Holy Charm") || has(&p.inv, "Old Coin");

    if worthy {
        println!("\nThe stone *shifts*.");
        println!("A low rumble travels through the chamber like thunder underground.");
        println!("The sword slides free—smooth, effortless, like it chose you.\n");
        println!("{} whispers: \"…It’s real.\"", name);
        add(&mut p.inv, "Excalibur");
        p.has_excalibur = true;
        Room::SwordChamber
    } else {
        println!("\nYou pull. The sword doesn’t move.");
        println!("The chamber rejects you like a bouncer with standards.");
        println!("A shockwave hits your chest.");
        p.hp -= 15;
        println!("(-15 HP)");
        println!("Maybe you need proof of honor… something holy, or a knight’s sign.");
        Room::SwordChamber
    }
}

fn show_status(p: &Player) {
    println!("\n------------------------------");
    println!("HP: {}", p.hp);
    println!("Inventory: {}", if p.inv.is_empty() { "(empty)".to_string() } else { format!("{:?}", p.inv) });
    println!("------------------------------");
}

fn use_item(p: &mut Player) {
    println!("\nInventory:");
    if p.inv.is_empty() {
        println!("(empty)");
        return;
    }
    for item in &p.inv {
        println!("- {}", item);
    }
    let item = read("Type item name to use (or back): ");
    if item.eq_ignore_ascii_case("back") { return; }

    if !has(&p.inv, &item) {
        println!("You don’t have that.");
        return;
    }

    match item.as_str() {
        "Medkit" => {
            println!("You wrap bandages and breathe through the pain.");
            p.hp += 25;
            if p.hp > 100 { p.hp = 100; }
            println!("(+25 HP, max 100)");
            remove(&mut p.inv, "Medkit");
        }
        "Torch" => println!("You light the torch. The shadows back up a step."),
        "Holy Charm" => println!("The charm warms in your palm. Ghosts probably hate this."),
        "Old Coin" => println!("You rub the coin. It feels… significant. Like proof."),
        _ => println!("You use it. Nothing obvious happens."),
    }
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