use std::collections::HashMap;
use std::fs;
use std::io::stdin;
use rand::Rng;

struct Card {
    card_name:String,
    val:i32,
}

struct Player {
    count:i32,
    games_won:i32,,
    cash:i32,
}

impl Player {

    fn new()->Self{
        Self{
            count:0,
            games_won:0,
            cas:500,
        }
    }

    fn addCard(& mut self,cVal:i32){
        self.count +=cVal
    }

    fn getVal(&self)->i32{
        self.count
    }

    fn resetScore(& mut self){
        self.count = 0
    }

    fn wonGame(&mut self){
        self.games_won+=1
    }

    fn getGamesWon(&self)->i32{
        self.games_won
    }

}



impl Card {
    
    fn new (card_name: String, cardVal: i32) -> Self{
        Self{
            card_name:card_name.to_owned(),
            val:cardVal
        }
    }

    fn getVal (&self) -> i32 {
        self.val
    }

    fn getName (&self) -> &str {
        
        &self.card_name
    }
}


fn getUserInput()->String{
    let mut userIn = String::new();

    stdin().read_line(&mut userIn).expect("unable to read input");

    userIn.trim().to_lowercase()
}

fn welcome(){
    println!("Welcome to consolejack, a blackjack ripoff");
}

fn getCardPrompt(userDone:bool){

    if userDone == false {
    println!("\nPress [H]it to get another card and [S]tay to stay. Press [e] to quit/exit\n");
    }
    else{
        println!("\nPress [H] for dealers next card. Press [e] to quit/exit\n");       
    }
}

fn loadCardData() ->Vec<Card>{

    let mut CardList: Vec<Card> = Vec::new();

    let contents = fs::read_to_string("./cards.txt").expect("Unable to read file");


    let stringParts: Vec<&str> = contents.split("\n").collect();
    
    for part in stringParts.iter(){
        let stringPart = part;
        let stringParts: Vec<&str>  = stringPart.split(":").map(|s|s.trim()).collect();

        if stringParts.len() > 1 {
            let cardName = stringParts[0].to_string();
            let cardValString:i32 = stringParts[1].parse().unwrap();

            println!("{}",cardName);
            println!("{}",cardValString);
            println!("--------");
            let tCard  = Card::new(cardName,cardValString);
            CardList.push(tCard);
        }
    }

    //print length of card data
    let cardCount = CardList.len();

    println!("Cards Loaded {}",cardCount);

    CardList
}
fn getRandomCard(count:i32)->i32{
    let randomIndex =  rand::thread_rng().gen_range(0..count);
    randomIndex
}

fn main() {

    let mut done:bool = false;
    let mut userDone:bool = false;
    let mut state = 0;                    
    let cards: Vec<Card> = loadCardData();
    let count = cards.len();

    let mut cardIndexesUsed: Vec<i32> = Vec::new();

    //make player
    let mut player = Player::new();

    //make dealer
    let mut dealer = Player::new();

    loop {
        if done {
            break
        }else{

            let mut userInput = String::new();

            match state {
                0 => {
                    //welcome
                    welcome();
                    println!("Press any key to continue");
                    userInput = getUserInput();
                    state = 1;
                },
                1 => {
                    
                //beginner of game loop
                //initial card deal
                getCardPrompt(userDone);
                userInput = getUserInput();

                //get if hit or stay
                //if hit draw another and update score for user
                //if stay update score for dealer
                //check for win or loss condition
                

                    match userInput.as_str() {

                        "h" | "H" => {

                            let mut randomCardIndex = 0;
                            let mut validIndex = false;
                            
                            if userDone == false {
                                println!("Player selected hit!");

                                loop{
                                    randomCardIndex = getRandomCard(count as i32) as i32;

                                    //check for first card
                                    if cardIndexesUsed.len()<1{
                                        cardIndexesUsed.push(randomCardIndex as i32);
                                        validIndex = true;
                                        break;
                                    }
                                    else{
                                        for i in 0 ..cardIndexesUsed.len(){
                                            if randomCardIndex as i32 != cardIndexesUsed[i] {
                                                cardIndexesUsed.push(randomCardIndex as i32);
                                                validIndex =true;
                                                break;
                                            }
                                        }
                                    }
                                    if validIndex{
                                        break;
                                    }
                                }

                            let tCard = &cards[randomCardIndex as usize];
                            
                            //show card to player
                            println!("Plater Card Dealt: {}",tCard.getName());
                            player.addCard(tCard.getVal());

                            }
                            
                            //getting deaer card
                            randomCardIndex = 0;
                            validIndex = false;

                            loop{
                                randomCardIndex = getRandomCard(count as i32) as i32;
                                //println!("rng {}",randomCardIndex);

                                //check for first card
                                if cardIndexesUsed.len()<1{
                                    cardIndexesUsed.push(randomCardIndex as i32);
                                    validIndex = true;
                                    break;
                                }
                                else{
                                    for i in 0 ..cardIndexesUsed.len(){
                                        if randomCardIndex as i32 != cardIndexesUsed[i] {
                                            cardIndexesUsed.push(randomCardIndex as i32);
                                            validIndex =true;
                                            break;
                                        }
                                    }
                                }
                                if validIndex{
                                    break;
                                }
                            }

                            //got random card index time to get card from desk
                            
                            let tCard = &cards[randomCardIndex as usize];
                            
                            //show card to player
                            println!("Player Card Dealt: {}",tCard.getName());
                            dealer.addCard(tCard.getVal());

                        
                        },
                        "s" | "s" => {
                            println!("Player selected stay!");
                            let mut randomCardIndex = 0;
                            let mut validIndex = false;

                            loop{
                                randomCardIndex = getRandomCard(count as i32) as i32;
                                //println!("rng {}",randomCardIndex);

                                //check for first card
                                if cardIndexesUsed.len()<1{
                                    cardIndexesUsed.push(randomCardIndex as i32);
                                    validIndex = true;
                                    break;
                                }
                                else{
                                    for i in 0 ..cardIndexesUsed.len(){
                                        if randomCardIndex as i32 != cardIndexesUsed[i] {
                                            cardIndexesUsed.push(randomCardIndex as i32);
                                            validIndex =true;
                                            break;
                                        }
                                    }
                                }
                                if validIndex{
                                    break;
                                }
                            }

                            //got random card index time to get card from desk                            
                            let tCard = &cards[randomCardIndex as usize];
                            
                            //show card to player
                            println!("Dealer Card Dealt: {}",tCard.getName());
                            dealer.addCard(tCard.getVal());
                            userDone=true;
                        },
                        "e" | "E" => {
                            state = 5
                        },
                        _ => {
                            println!("Unknown command");
                        }


                    }

                    //check win lost condition
                    let playerVal = player.getVal();
                    let dealerVal = dealer.getVal();

                    println!("player count: {}",playerVal);
                    println!("dealer count: {}",dealerVal);

                    if playerVal >=21 || dealerVal >= 21 {

                        //check if tied
                        if playerVal == dealerVal {
                            state = 4;
                        }

                        //if player has 21
                        else if playerVal == 21 {
                            state = 2
                        
                        }

                        //if dealer has 21
                        else if dealerVal == 21 {
                            state = 3
                        }
                        // player busts
                        else if playerVal > 21 {
                            state = 3;
                        }
                        //dealer busts
                        else if dealerVal > 21 {
                            state = 2
                        }
                        //player busts
                        else if playerVal > 21 {
                            state  = 3 
                        }

                        //player has higher hand
                        else if playerVal > dealerVal {
                            state = 2;
                        }
                        //dealer has higher hand
                        else {
                            state = 3;
                        }
                    }
                },

                2 => {

                    //player won
                    println!("You Won!");
                    println!("Press any key to play again or [e] to exit");

                    let userInput = getUserInput();

                    player.wonGame();

                        match userInput.as_str(){


                            "e" => {
                                state = 5;
                            },
                            _ =>{
                                done = false;
                                player.resetScore();
                                
                                dealer.resetScore();
                                
                                cardIndexesUsed = Vec::new();
                                userDone = false;
                                state = 1;
                            }
                        }
                },
                3 =>{

                    //dealer won
                    println!("Game Over. Dealer Won!");
                    println!("Press any key to play again or [e] to exit");
                    userInput = getUserInput();

                    dealer.wonGame();

                        match userInput.as_str(){
                            "e" => {
                                state = 5;
                            },
                            _ => {
                                done = false;
                                player.resetScore();
                                dealer.resetScore();
                                cardIndexesUsed = Vec::new();
                                userDone = false;
                                state = 1;
                            }
                        }
                },
                4 =>{

                    //tie
                    println!("Game tied!");
                    println!("Press any key to play again or [e] to exit");
                    userInput = getUserInput();

                    match userInput.as_str(){
                        "e" => {
                            state = 5;
                        },
                        _ => {
                            done = false;
                            player.resetScore();
                            dealer.resetScore();
                            cardIndexesUsed = Vec::new();
                            userDone = false;
                            state = 1;
                        }
                    }


                },
                5 =>{
                    println!("Goodbye");

                    println!("-----------------------");
                    println!("Games won: {}",player.getGamesWon());
                    println!("-----------------------");
                    println!("Games lost: {}",dealer.getGamesWon());
                    println!("------------------------");

                    done=true;
                    break;
                    }
                _ =>{
                    println!("unknown input");
                }

            }
            }

            //check scores for game over!

        }
}
