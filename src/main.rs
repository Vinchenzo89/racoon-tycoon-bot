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
        match COMODITY_NAME_MAP.iter().find(|c| c.0 == name) {
            Some((_, c)) => Some(*c),
            _ => None
        }
    }

    fn parse_from_input(input: &str) -> Result<(Self, u32), String> {
        let input: Vec<&str> = input.split('-').collect();
        if input.len() == 2 {
            let name = input[0];
            let amount = input[1];
            if let Some(value) = Comodity::from_name(name) {
                if let Ok(amount) = amount.parse::<u32>() {
                    return Ok((value, amount));
                } else {
                    return Err(format!("{} is not a number", input[1]));
                }
            } else {
                return Err(format!("{} is not a comodity", input[0]));
            }
        } else {
            return Err(String::from("Example: Iron-3"));
        }
    }

    #[inline]
    fn index(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn name(&self) -> &str {
        COMODITY_NAME_MAP[*self as usize].0
    }
}

impl Default for Comodity {
    fn default() -> Self { Comodity::Any }
}

const COMODITY_NAME_MAP: [(&'static str, Comodity); Comodity::Count as usize] = [
    ("Any", Comodity::Any),
    ("Coal", Comodity::Coal),
    ("Goods", Comodity::Goods),
    ("Iron", Comodity::Iron),
    ("Luxury", Comodity::Luxury),
    ("Wheat", Comodity::Wheat),
    ("Wood", Comodity::Wood),
];

#[derive(Default, Clone, Copy)]
struct ComoditySale {
    pub comodity: Comodity,
    pub amount: u32,
    pub market_price: u32,
}

#[derive(Default, Clone, Copy)]
struct ComodityPrice {
    pub min: u32,
    pub max: u32,
    pub price: u32,
}

impl ComodityPrice {
    fn new(min: u32, max: u32) -> Self {
        Self {
            min: min,
            max: max,
            price: min,
        }
    }

    fn inflate(&mut self, amount: u32) {
        let result = match self.price.checked_add(amount) {
            Some(r) => u32::min(r, self.max),
            None => self.max,
        };
        self.price = result;
    }

    fn deflate(&mut self, amount: u32) {
        let result = match self.price.checked_sub(amount) {
            Some(r) => u32::max(r, self.min),
            None => self.min
        };
        self.price = result;
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

    fn comodity_supply(&self, c: Comodity) -> u32 {
        self.comodities[c.index()]
    }
}

#[derive(Default)]
struct GameState {
    pub market_place: [ComodityPrice; Comodity::Count as usize],
    pub town_deck: Vec<Town>,
    pub building_deck: Vec<Building>,
    pub railroad_deck: Vec<Railroad>,
    pub players: Vec<Player>,
    current_player_turn: usize,
}

fn game_comodity_price(game: &GameState, c: Comodity) -> u32 {
    let result = game.market_place[c as usize];
    result.price
}

fn game_current_player(game: &GameState) -> &Player {
    &game.players[game.current_player_turn]
}

fn game_current_player_mut(game: &mut GameState) -> &mut Player {
    &mut game.players[game.current_player_turn]
}

fn init_market_place(game: &mut GameState) {
    // These are in the game board order from left to right
    game.market_place[Comodity::Wheat  as usize] = ComodityPrice::new(1, 12);
    game.market_place[Comodity::Wood   as usize] = ComodityPrice::new(1, 12);
    game.market_place[Comodity::Iron   as usize] = ComodityPrice::new(2, 13);
    game.market_place[Comodity::Coal   as usize] = ComodityPrice::new(2, 13);
    game.market_place[Comodity::Goods  as usize] = ComodityPrice::new(3, 14);
    game.market_place[Comodity::Luxury as usize] = ComodityPrice::new(3, 14);
    game.market_place[Comodity::Any    as usize] = ComodityPrice::default();
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

fn option_not_implemented() {
    println!("Option not implemented yet. :(");
}

fn invalid_opition() {
    println!("That's not an option. Try again");
}

fn game_action_produce(game: &mut GameState, prod: &ProductionCard) {
    let player = game_current_player_mut(game);
    for p in prod.produce.iter() {
        player.comodities[p.0.index()] += p.1;
    }
    for p in prod.inflate.iter() {
        game.market_place[p.0.index()].inflate(p.1);
    }
}

fn game_action_sell(game: &mut GameState, sale: &ComoditySale) {
    let market_price = game_comodity_price(game, sale.comodity);
    let player = game_current_player_mut(game);
    let supply = player.comodities[sale.comodity.index()];

    // if the sale is for more than the player has,
    // we just execute the sale for whatever they have and wipe out their supply.
    let new_supply = match supply.checked_sub(sale.amount) {
        Some(s) => s,
        None => 0,
    };
    let actual_amount = supply - new_supply;

    player.money += actual_amount * market_price;
    player.comodities[sale.comodity.index()] = new_supply;
    game.market_place[sale.comodity.index()].deflate(sale.amount);
}

fn exec_player_turn(game: &mut GameState) {
    let player = game_current_player(game);
    
    println!("Its {}'s turn", player.name);
    println!("========================");
    println!(" What action did the player take?");
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
            "2" => {game_action_sell(game, &player_sell_comodities(game));},
            "3" => {option_not_implemented();},
            "4" => {option_not_implemented();},
            "5" => {option_not_implemented();},
            _ => {invalid_opition();},
        }
        break;
    }

    game.current_player_turn = (1 + game.current_player_turn) % game.players.len();
}

fn player_produce_comodities() -> ProductionCard {
    println!("========================");
    println!("     Production Card");
    println!("========================");
    let mut production = ProductionCard::default();
    
    // get all comodities to be produced
    println!("Enter each comodity produced in format: Comodity-Amount:");
    println!("Example: Enter \"Iron-3\" if card produces 3 Iron");
    println!("Enter \"done\" when finished");
    input_loop(
        &mut production, 
        |_| {}, 
        |p, input| {
            let mut result = false;
            if input == "done" {
                result = true;
            } else {
                match Comodity::parse_from_input(input) {
                    Ok((comodity, amount)) => p.produce.push(Production(comodity, amount)),
                    Err(e) => println!("{}", e),
                }
            }
            result
        }
    );

    // get all comodities to be inflated in price
    println!("Enter each comodity and amount to inflate price in format: Comodity-Amount:");
    println!("Example: Enter \"Iron-3\" if card inflates the price of Iron by $3");
    println!("Enter \"done\" when finished");
    input_loop(
        &mut production, 
        |_| {}, 
        |p, input| {
            let mut result = false;
            if input == "done" {
                result = true;
            } else {
                match Comodity::parse_from_input(input) {
                    Ok((comodity, amount)) => p.inflate.push(Production(comodity, amount)),
                    Err(e) => println!("{}", e),
                }
            }
            result
        }
    );

    println!("Production card will produce:");
    for p in production.produce.iter() {
        println!("{} by {}", p.0.name(), p.1);
    }
    println!("Production card will inflate:");
    for p in production.inflate.iter() {
        println!("{} by ${}", p.1, p.0.name());
    }

    production
}

fn player_sell_comodities(game: &GameState) -> ComoditySale {
    println!("========================");
    println!("     Sell Comodities");
    println!("========================");
    let mut sale = ComoditySale::default();
    let player = game_current_player(game);

    // get all comodities produced
    println!("Enter the comodity being sold and number of units in the format: Comodity-Amount:");
    println!("Example: Enter \"Iron-3\" if selling 3 units of Iron");
    println!("Enter \"done\" when finished");
    input_loop(
        &mut (player, &mut sale), 
        |_| {}, 
        |(player, sale), input| {
            let mut done = false;
            if input == "done" {
                done = true;
            } else {
                match Comodity::parse_from_input(input) {
                    Ok((comodity, amount)) => {
                        let supply = player.comodity_supply(comodity);
                        done = amount <= supply;
                        if done {
                            sale.comodity = comodity;
                            sale.amount = amount;
                        } else {
                            println!("Player {} only has {} of {}", player.name, supply, comodity.name());
                        }
                    },
                    Err(e) => println!("{}", e),
                }
            }
            done
        }
    );

    sale
}

fn new_game() {
    println!("\nNew Game");

    let mut game = GameState::default();
    init_market_place(&mut game);
    init_players(&mut game);

    input_loop(
        &mut game, 
        |_| {
            println!("================");
            println!("show - Show current state of the game");
            println!("Press Enter to start the next turn...");
        }, 
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
            buf.push_str(&format!("{}-{} ", COMODITY_NAME_MAP[c].0, p.comodities[c]));
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
