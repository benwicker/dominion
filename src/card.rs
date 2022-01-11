use super::player::Player;

#[derive(Clone)]
pub struct Card {
  pub name: String,
  pub description: String,
  pub cost: i32,
  pub card_type: CardType,
}

impl Card {
  pub fn copper() -> Card {
    Card {
      name: String::from("Copper"),
      description: String::from("+1c"),
      cost: 0,
      card_type: CardType::Treasure(1),
    }
  }

  pub fn estate() -> Card {
    Card {
      name: String::from("Estate"),
      description: String::from("+1VP"),
      cost: 2,
      card_type: CardType::VictoryPoint(1),
    }
  }

  pub fn militia() -> Card {
    Card {
      name: String::from("Militia"),
      description: String::from("+2c"),
      cost: 4,
      card_type: CardType::Action(vec![Effects::AddCoin(2)]),
    }
  }

  pub fn play(&self, player: &mut Player) {
    match self.card_type {
      CardType::Action(ref effects) => {
        if player.available_actions < 1 {
          panic!("no actions left")
        }

        for effect in effects {
          effect.apply(player);
        }
      },
      CardType::Treasure(ref amount) => {
        player.available_coin += amount;
      },
      _ => (),
    }      
  }
}

impl ToString for Card {
  fn to_string(&self) -> String {
    format!("{} ({})", self.name, self.description)
  }
}

#[derive(Clone)]
pub enum CardType {
  Treasure(i32),
  VictoryPoint(i32),
  Action(Vec<Effects>),
}

#[derive(Clone)]
pub enum Effects {
  AddCoin(i32),
  AddVp(i32),
}

impl Effects {
  pub fn apply(&self, player: &mut Player) {
    use Effects::*;

    match *self {
      AddVp(ref amount) => player.vp += amount,
      AddCoin(ref amount) => player.available_coin += amount,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_vp() {
    let mut player = Player::new();
    let effect = Effects::AddVp(1);

    effect.apply(&mut player);

    assert_eq!(player.vp, 1);
  }

  #[test]

  fn add_coin() {
    let mut player = Player::new();
    let effect = Effects::AddCoin(1);

    effect.apply(&mut player);

    assert_eq!(player.available_coin, 1);
  }

  #[test]
  fn play_militia() {
    let mut player = Player::new();
    let card = Card::militia();

    card.play(&mut player);

    assert_eq!(player.available_coin, 2);
  }

  #[test]
  fn play_copper() {
    let mut player = Player::new();
    let card = Card::copper();

    card.play(&mut player);

    assert_eq!(player.available_coin, 1);
  }

  #[test]
  fn play_estate() {
    let mut player = Player::new();
    let card = Card::estate();

    card.play(&mut player);

    assert_eq!(player.vp, 0);
  }
}