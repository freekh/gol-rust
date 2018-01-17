// Completed in interview (2 hours pairprogramming)
#![feature(plugin,custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::{thread, time};
use rocket::request::*;
use rocket_contrib::*;

fn print(world: &Vec<Vec<bool>>) {
  for row in world.iter() {
    for column in row.iter() {
      if *column {
        print!(" V ");
      } else {
        print!(" X ");
      }
    }
    println!("");
  }
}

const NEIGHBOURS: [(i64, i64); 8] = [
  ( 1, 1),  ( 1, 0),  ( 1, -1),
  ( 0, 1)/*,( 0, 0)*/,( 0, -1), // skip self
  (-1, 1),  (-1, 0),  (-1, -1)
];

fn count_neighbours(world: &Vec<Vec<bool>>, x: i64, y: i64) -> usize {
  NEIGHBOURS.iter().fold(0, |sum, &(nx, ny)| {
    let x_bound = nx + x;
    let y_bound = ny + y;
    if x_bound >= 0 && y_bound >= 0 {
      let &alive = world
        .get(x_bound as usize)
        .and_then(|row| { row.get(y_bound as usize) })
        .unwrap_or(&false);
      if alive {
        sum + 1
      } else {
        sum
      }
    } else {
      sum
    }
  })
}

fn next(world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  world.iter().enumerate().map(|(x, row)| {
    row.iter().enumerate().map(|(y, &alive)| {
      let live_neighbours = count_neighbours(&world, x as i64, y as i64);
      if alive && live_neighbours < 2 {
        false
      } else if alive && (live_neighbours == 2 || live_neighbours == 3) {
        true
      } else if alive && live_neighbours > 3 {
        false
      } else if !alive && live_neighbours == 3 {
        true
      } else {
        alive
      }
    }).collect()
  }).collect()
}

#[derive(Serialize, Deserialize)]
struct World {
  data: Vec<Vec<bool>>,
  generation: u64,
}

#[derive(FromForm)]
struct Qp {
  generations: u64
}

#[post("/gol?<qp>", data = "<world>")]
fn gol(world: Json<World>, qp: Qp) -> Option<Json<World>> {
  let mut data = world.data.clone();
  for _ in world.generation..qp.generations {
    data = next(data);
  }
  Some(Json(World {
    generation: world.generation + qp.generations,
    data: data,
  }))
}

fn main() {
  // Uncomment API or CLI

  // API inteface
  // rocket::ignite().mount("/", routes![gol]).launch();

  // CLI interface
  // let mut world = vec![
  //   vec![false, false, true , false, false],
  //   vec![false, false, true , false , false],
  //   vec![false, false, true , false, false],
  //   vec![false, false, false, false, false],
  //   vec![false, false, false, false, false],
  // ];

  // let sleep_time = time::Duration::from_millis(1000);
  // loop {
  //   print(&world);
  //   println!();
  //   world = next(world);
  //   thread::sleep(sleep_time);
  // }
}
