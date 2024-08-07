pub fn maybe() -> bool {
  rand::random::<i32>() % 2 == 0
}

pub fn maybe_chance(chance: i32) -> bool {
  rand::random::<i32>() % chance == 0
}

pub fn pick_one<T>(vec: &Vec<T>) -> T
where T: Clone
{
  vec[rand::random::<usize>() % vec.len()].clone()
}

pub fn pick_x<T>(vec: Vec<T>, can_repeat: bool, x: i32) -> Vec<T>
where T: Clone
{
  let mut picked = Vec::new();
  let mut vec = vec.clone();
  
  // Ensure there are enough items to enforce can_repeat
  if !can_repeat && vec.len() < x as usize {
    return vec;
  }

  for _ in 0..x {
    let index = rand::random::<usize>() % vec.len();
    picked.push(vec.remove(index));
  }

  picked
}