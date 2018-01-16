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

fn next(world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
  world.iter().enumerate().map(|(x, row)| {
    row.iter().enumerate().map(|(y, cell)| {
      let mut live_neighbours = 0;
      for neighbour_x in 0..3 {
        for neighbour_y in 0..3 {
          let x_bound: i32 = (neighbour_x as i32) + (x as i32) - 1;
          let y_bound: i32 = (neighbour_y as i32) + (y as i32) - 1;
          if !(neighbour_x == 1 && neighbour_y == 1) && // 1 is really 0 ;)
            x_bound >= 0 && x_bound < world.len() as i32 &&
            y_bound >= 0 && y_bound < row.len() as i32 &&
            world[x_bound as usize][y_bound as usize] {
              live_neighbours += 1;
          }
        }
      }
      if *cell && live_neighbours < 2 {
        false
      } else if *cell && (live_neighbours == 2 || live_neighbours == 3) {
        true
      } else if *cell && live_neighbours > 3 {
        false
      } else if !*cell && live_neighbours == 3 {
        true
      } else {
        *cell
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
   rocket::ignite().mount("/", routes![gol]).launch();
  // let mut world = vec![
  //   vec![false, false, false, false, false],
  //   vec![false, true , true , true , false],
  //   vec![false, false, false, false, false],
  //   vec![false, false, false, false, false],
  //   vec![false, false, false, false, false],
  // ];

  // loop {
  //   world = next(world);
  //   print(&world);
  //   println!();
  //   let sleep_time = time::Duration::from_millis(1000);
  //   thread::sleep(sleep_time);
  // }
}
