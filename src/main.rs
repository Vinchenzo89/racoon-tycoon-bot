#[derive(Default)]
struct Town {
    comodity_purchase_type: Comodity,
    comodity_purchase_amount: u32,
    victory_points: u32
}

#[derive(Default)]
struct Railroad {
    price: u32,
    victory_points: u32
}

#[derive(Default)]
struct Building {
    price: u32,
    victory_points: u32
}

#[derive(Copy, Clone)]
enum Comodity {
    Any,
    Coal,
    Goods,
    Iron,
    Luxury,
    Wheat,
    Wood,
    // Keep this last
    Count
}

impl Comodity {
    fn from_name(name: &str) -> Option<Comodity> {
        for c in ComodityNameMap {
            if c.0 == name {
                return Some(c.1);
            }
        }
        None
    }
}

const ComodityNameMap: [(&'static str, Comodity); Comodity::Count as usize] = [
    ("Any", Comodity::Any),
    ("Coal", Comodity::Coal),
    ("Goods", Comodity::Goods),
    ("Iron", Comodity::Iron),
    ("Luxury", Comodity::Luxury),
    ("Wheat", Comodity::Wheat),
    ("Wood", Comodity::Wood),
];

impl Default for Comodity {
    fn default() -> Self { Comodity::Any }
}

#[derive(Default, Clone, Copy)]
struct ComodityPrice {
    pub min: u32,
    pub max: u32,
    pub current: u32,
}

impl ComodityPrice {
    fn new(min: u32, max: u32) -> Self {
        Self {
            min: min,
            max: max,
            current: min,
        }
    }

    fn inflate(&mut self, amount: u32) {
        let result = (self.current + amount).max(self.min).min(self.max);
        self.current = result;
    }

    fn deflate(&mut self, amount: u32) {
        let result = (self.current - amount).max(self.min).min(self.max);
        self.current = result;
    }
}

// Card has 6 productions 6 inflations
#[derive(Default)]
struct ProductionCard {
    pub produce: Vec<Production>, 
    pub inflate: Vec<Production>,
}

#[derive(Copy, Clone)]
struct Production(Comodity, u32);

#[derive(Default)]
struct Player {
    pub name: String,
    pub money: u32,
    pub comodities: [u32; Comodity::Count as usize],
    pub production: Vec<ProductionCard>,
    pub buildings: Vec<Building>,
    pub railroads: Vec<Railroad>,
    pub towns: Vec<Town>,
}

impl Player {
    fn new(name: String) -> Self {
        let mut result = Self::default();
        result.name = name;
        result
    }
}

#[derive(Default)]
struct GameState {
    pub market_place: [ComodityPrice; Comodity::Count as usize],
    pub town_deck: Vec<Town>,
    pub building_deck: Vec<Building>,
    pub railroad_deck: Vec<Railroad>,
    pub players: Vec<Player>,
    current_player_tern: usize,
}

fn game_comodity_price(game: &GameState, c: Comodity) -> u32 {
    let result = game.market_place[c as usize];
    result.current
}

fn game_current_player(game: &mut GameState) -> &mut Player {
    &mut game.players[game.current_player_tern]
}

fn game_action_produce(game: &mut GameState, prod: &ProductionCard) {
    let player = game_current_player(game);
    for p in prod.produce.iter() {
        player.comodities[p.0 as usize] += p.1;
    }
    for p in prod.inflate.iter() {
        game.market_place[p.0 as usize].inflate(p.1);
    }
}

fn init_market_place(game: &mut GameState) {
    // These are in the game board order from left to right
    game.market_place[Comodity::Wheat as usize] = ComodityPrice::new(1, 12);
    game.market_place[Comodity::Wood as usize] = ComodityPrice::new(1, 12);
    game.market_place[Comodity::Iron as usize] = ComodityPrice::new(2, 13);
    game.market_place[Comodity::Coal as usize] = ComodityPrice::new(2, 13);
    game.market_place[Comodity::Goods as usize] = ComodityPrice::new(3, 14);
    game.market_place[Comodity::Luxury as usize] = ComodityPrice::new(3, 14);
    game.market_place[Comodity::Any as usize] = ComodityPrice::default();
}

fn init_players(game: &mut GameState) {
    println!("Enter each player's name in the order they will take turns.");

    input_loop(
        game, 
        |game| {
            let next_player = game.players.len() + 1;
            println!("Enter Player {} name or 'done': ", next_player);
        }, 
        |game, input| {
            match input {
                "done" => true,
                _ => {
                    game.players.push(Player::new(String::from(input)));
                    false
                }
            }
        }
    );

    // add the bot
    game.players.push(Player::new(String::from("Racoon Bot")));
}

fn exec_player_turn(game: &mut GameState) {
    let player = game_current_player(game);
    
    println!("Its {}'s turn", player.name);
    println!("========================");
    println!(" What action was taken?");
    println!("========================");
    println!("1 Produce Comodities");
    println!("2 Sell Comodities");
    println!("3 Buy Building");
    println!("4 Buy Railroad");
    println!("5 Auction a Railroad");

    let mut input = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        let input = input.trim();
        match input {
            "1" => {game_action_produce(game, &player_produce_comodities())},
            "2" => {},
            "3" => {},
            "4" => {},
            "5" => {},
            _ => {},
        }
        break;
    }

    game.current_player_tern = (1 + game.current_player_tern) % game.players.len();
}

fn player_produce_comodities() -> ProductionCard {
    println!("========================");
    println!("      Production");
    println!("========================");
    let mut production = ProductionCard::default();
    
    // get all comodities produced
    println!("Enter each comodity produced as Comodity-Amount:");
    input_loop(
        &mut production, 
        |p| {
            let mut buf = String::new();
            for prod in p.produce.iter() {
                buf.push_str(&format!("{}-{} ", ComodityNameMap[prod.0 as usize].0, prod.1));
            }
            println!("{}", buf);
        }, 
        |p, input| {
            let mut result = false;
            if input == "done" {
                result = true;
            } else {
                let input: Vec<&str> = input.split('-').collect();
                if input.len() == 2 {
                    let name = input[0];
                    let amount = input[1];
                    if let Some(value) = Comodity::from_name(name) {
                        if let Ok(amount) = amount.parse::<u32>() {
                            p.produce.push(Production(value, amount));
                        }
                        else {
                            println!("{} is not a number", input[1]);
                        }
                    } else {
                        println!("{} is not a comodity", input[0]);
                    }
                } else {
                    println!("Example: Coal-2");
                }
            }
            result
        }
    );

    production
}

fn new_game() {
    println!("\nNew Game");

    let mut game = GameState::default();
    init_market_place(&mut game);
    init_players(&mut game);

    println!("Press Enter to start the game...");
    input_loop(
        &mut game, 
        |_| {}, 
        |game, input| {
            match input {
                "end" => true,
                "show" => { show_game(&game); false },
                _ => { exec_player_turn(game); false },
            }
        }
    );
}

fn show_game(game: &GameState) {
    println!("Market Place");
    println!("------------");
    println!("Wheat  Wood  Iron  Coal  Goods  Luxury");
    println!("${}     ${}    ${}    ${}    ${}     ${}",
        game_comodity_price(&game, Comodity::Wheat),
        game_comodity_price(&game, Comodity::Wood),
        game_comodity_price(&game, Comodity::Iron),
        game_comodity_price(&game, Comodity::Coal),
        game_comodity_price(&game, Comodity::Goods),
        game_comodity_price(&game, Comodity::Luxury)
    );
    println!();
    println!("Players");
    println!("-------");
    for p in game.players.iter() {
        println!("{}", p.name);
        println!("=========");
        println!("    Money ${}   Buildings {}   Railroads {} ", p.money, p.buildings.len(), p.railroads.len());
        let mut buf = String::new();
        for c in 0..(p.comodities.len()) {
            buf.push_str(&format!("{}-{} ", ComodityNameMap[c].0, p.comodities[c]));
        }
        println!("    {}", buf);
        println!("    Victory Points: {}", 0);
    }
    println!();
}

fn main() {
    input_loop(
        &mut Some(0),
        |_| {
            println!("Racoon Tycoon Bot");
            println!("-----------------");
            println!("new  - Make a new game");
            println!("show - Show current state of the game");
            println!("end  - Close app");
            println!();
        }, 
        |_, input| {
            match input {
                "end" => true,
                "new" => { new_game(); false },
                _ => { println!("Not an option"); false },
            }
        });
}

fn input_loop<T>(
    ctx: &mut T,
    before_input: fn(&T),
    handle_input: fn(&mut T, &str) -> bool
) {
    let stdin = std::io::stdin();
    loop {
        before_input(ctx);
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let result = handle_input(ctx, input.trim());
                if result { break; }
            },
            _ => {println!("Error reading input.")}
        }
    }
}
