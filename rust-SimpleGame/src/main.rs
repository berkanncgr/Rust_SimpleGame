#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![warn(unused_parens)]

use std::io::{stdin ,Write, stdout}; // for read input.

const PLAYER:&str="🔻";
const FOOD:&str="🍔";
const EMPTY:&str="⚫";

const ROW:u8=11;
const COLOMN:u8=11;

#[derive(Copy,Clone)]
enum Kind
{
    Food,
    Player,
    Empty,
}

enum Position
{
    Left, Right, Up, Down // for player
}

type Area=[[Kind;11];11];

struct Entity
{
    x:u8,
    y:u8,
}

struct Game
{
    area:Area,
    player:Entity,
    food:Entity,
    score:u8,
}

trait GameTrait
{
    fn Init(&mut self);      // Call on start.
    fn Draw (&self);         //Draw the map.
    fn ReadKey(&mut self);   //Read key for the actio.
    fn Update(&mut self);    //
    fn UpdatePlayerPosition(&mut self, position:Position);
}

impl GameTrait for Game
{
    fn Init(&mut self)
    {
        self.area[self.player.y as usize][self.player.x as usize]=Kind::Player; // Set player loc.
        self.area[self.food.y as usize][self.food.x as usize]=Kind::Food;       // Set food loc.
        self.Update();
    }

    fn Draw(&self) //doesnt need to be mut
    {
        print!("\x1B[2J\x1B[1;1H]");
        print!("\n");
        for y in 0..self.area.len()
        {
            for x in 0..self.area[y].len()
            {
                match self.area[y][x]
                {
                    Kind::Food=>print!("{}",FOOD),
                    Kind::Player=>print!("{}",PLAYER),
                    Kind::Empty=>print!("{}",EMPTY),
                }
            }
            print!("\n");
        }
        println!("SCORE: {}",self.score); // print the score.
    }

    fn ReadKey(&mut self)
    {
        print!("\n Bir deger bekleniyor (a, s, d, w)");
        stdout().flush();
        let mut input=String::new();
        if let Ok(_)=stdin().read_line(&mut input)
        {
            input=String::from(input.trim());
            match input.as_str()
            {
                "a"=>
                {
                    println!("A");
                    self.UpdatePlayerPosition(Position::Left );
                    self.Update();
                },
                
                "d"=>
                {
                    println!("D");
                    self.UpdatePlayerPosition(Position::Right);
                    self.Update();
                },
                
                "w"=>
                {
                    println!("W");
                    self.UpdatePlayerPosition(Position::Up);
                    self.Update();
                },
                
                "s"=>
                {
                    println!("S");
                    self.UpdatePlayerPosition(Position::Down);
                    self.Update(); }
                
                "e"=> {},
              _=>self.ReadKey(),

            }
        }
    }


    fn UpdatePlayerPosition(&mut self,position:Position)
    {
        let (cx,cy) =(self.player.x,self.player.y);
        self.area[cy as usize][cx as usize]=Kind::Empty; // Empty the ol location ...

        match position //... and move to next position.
        {
            Position::Down =>
            {   
                let ny:u8 =if cy!=(ROW-1) { cy+1 } else {0};   
                self.player.y=ny; 
                self.area[ny as usize][cx as usize]=Kind::Player;
            }
            Position::Up =>
            {
                let ny:u8=if cx!=0 { cx-1 } else {COLOMN-1};  
                self.player.y=ny;
                self.area[ny as usize][cx as usize]=Kind::Player;
            }
            Position::Left =>
            {
                let nx:u8=if cx!=0 { cx-1 } else {COLOMN-1};  
                self.player.x=nx;
                self.area[cy as usize][nx as usize]=Kind::Player;
            }
            Position::Right =>
            {
                let nx:u8=cx+1;
                self.player.x=nx;
                self.area[cy as usize][nx as usize]=Kind::Player;
            }
        }
        if (self.player.x==self.food.x && self.player.y==self.food.y)
        {
            self.score+=1;
        }
    }

    fn Update(&mut self)
    {
        self.Draw();
        self.ReadKey();

    }

}

fn main() {
  
    let mut game:Game=Game
    {
        area:[[Kind::Empty;11];11],
        player:Entity{x:5,y:5},
        food:Entity{x:8,y:3},
        score:0,
    }; // Set player, food loc. value.
        
    game.Init();
    game.Draw();
    game.ReadKey();
  
}
