// can specify return type
fn func() -> i32 {
  let x = 5;
  return x;
}

{
  let x = 3;
  // 'x + 1' is an expression (returns 3)
  // 'x + 1;' is a statement (returns nothing)
  x + 1
}

// this is a valid funciton in rust
fn five() -> i32 {
  5
}

// can specify param types
fn plus_one(x: i32) -> i32 {
  x + 1
  // x + 1; would throw an error (statement not expression)
}