use lentsys::control::PadControl;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum InputCode {
  Up,
  Down,
  Left,
  Right,
  Jump,
  Fire,
  Confirm,
  Swap
}


pub fn map_input(controller: PadControl) -> HashSet<InputCode>{
  let mut inputs: HashSet<InputCode> = HashSet::new();

  if controller.start > 0 {
    inputs.insert(InputCode::Confirm);
  }

  if controller.a > 0 {
    inputs.insert(InputCode::Jump);
  }

  if controller.b > 0 {
    inputs.insert(InputCode::Swap);
  }

  if controller.x > 0 {
    inputs.insert(InputCode::Fire);
  }

  if controller.up > 0 {
    inputs.insert(InputCode::Up);
  }

  if controller.down > 0 {
    inputs.insert(InputCode::Down);
  }

  if controller.left > 0 {
    inputs.insert(InputCode::Left);
  }

  if controller.right > 0 {
    inputs.insert(InputCode::Right);
  }

  return inputs;
}