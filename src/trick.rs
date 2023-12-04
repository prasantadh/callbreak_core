use standard_deck::{Card, Rank, Suit};

const MAX_SIZE: usize = 4;

pub struct Trick {
    cards: [Option<Card>; MAX_SIZE],
    lead: usize,
}

impl Trick {
    pub fn new(lead: usize) -> Result<Trick, String> {
        if lead > MAX_SIZE - 1 {
            return Err(format!("lead must be in range [0, {}]", MAX_SIZE - 1));
        }
        Ok(Trick {
            cards: [None; MAX_SIZE],
            lead,
        })
    }

    fn size(&self) -> usize {
        for i in 0..=MAX_SIZE {
            if self.cards[(self.lead + i) % MAX_SIZE] == None {
                return i;
            }
        }
        return MAX_SIZE;
    }

    // somewhere the game needs to validate that
    // if the player has a winning playable card,
    // the player must play that card
    // in fact this itself might be a great place to add that logic
    pub fn add(&mut self, card: &Card, position: usize) -> Result<(), &str> {
        if position >= MAX_SIZE {
            return Err("position out of range, valid range is 0..=3");
        }
        if self.cards[position] != None {
            return Err("position already has valid card, replace not allowed");
        }
        if (self.lead + self.size()) % MAX_SIZE != position {
            return Err("position requested out of turn");
        }
        self.cards[position] = Some(*card);
        Ok(())
    }

    /// returns the winner of the current trick.
    /// If the current trick is not full, will return winner among the
    /// cards in the trick. Returns None if there are no cards in the trick.
    pub fn winner(&self) -> Option<Card> {
        // if there is no lead, there is no winner
        let lead = self.cards[self.lead];
        if lead == None {
            return None;
        }

        // if spades are present, spades will win
        let winner = self.suit_winner(&Suit::Spades);
        if winner != None {
            return Some(Card::new(&winner.unwrap(), &Suit::Spades));
        }

        // the winner must come from the leading suit,
        // and there must be a winner
        let winner = self.suit_winner(&lead.unwrap().get_suit());
        return Some(Card::new(&winner.unwrap(), &lead.unwrap().get_suit()));
    }

    /// returns winner of the current trick among the cards that are of `suit`
    /// returns None if no card of the current suit exists in the trick
    fn suit_winner(&self, suit: &Suit) -> Option<Rank> {
        match self
            .cards
            .iter()
            // filter out the suit cards
            .filter(|card| match card {
                None => false,
                Some(v) => v.get_suit() == *suit,
            })
            // map them into the ranks
            .map(|card| card.unwrap().get_rank())
            .collect::<Vec<_>>()
            .iter()
            // collect highest rank
            .max()
        {
            Some(v) => Some(*v),
            _ => None,
        }
    }

    pub fn leader(&self) -> Option<Card> {
        return self.cards[self.lead];
    }
}

#[cfg(test)]
mod tests {
    use standard_deck::{Card, Rank, Suit};

    use crate::Trick;

    #[test]
    fn empty_trick_has_none_winner() {
        let trick = Trick::new(0).unwrap();
        assert_eq!(trick.winner(), None);
    }

    #[test]
    fn single_card_trick_has_that_card_as_winner() {
        for i in 0..=3 {
            let card = Card::new(&Rank::Two, &Suit::Clubs);
            let mut trick = Trick::new(i).unwrap();
            trick.add(&card, i).unwrap();
            assert_eq!(trick.winner().unwrap(), card);
        }
    }

    #[test]
    fn spades_trick_has_spades_winner() {
        for i in 0..=3 {
            let card = Card::new(&Rank::Two, &Suit::Spades);
            let mut trick = Trick::new(i).unwrap();
            trick.add(&card, i).unwrap();
            let card = Card::new(&Rank::Three, &Suit::Spades);
            trick.add(&card, (i + 1) % 4).unwrap();
            assert_eq!(trick.winner().unwrap(), card);
        }
    }

    #[test]
    fn trumped_trick_has_spades_winner() {
        for i in 0..=3 {
            let card = Card::new(&Rank::Two, &Suit::Clubs);
            let mut trick = Trick::new(i).unwrap();
            trick.add(&card, i).unwrap();
            let card = Card::new(&Rank::Two, &Suit::Spades);
            trick.add(&card, (i + 1) % 4).unwrap();
            assert_eq!(trick.winner().unwrap(), card);
        }
    }

    #[test]
    fn cannot_add_out_of_turn() {
        let card = Card::new(&Rank::Two, &Suit::Clubs);
        let mut trick = Trick::new(1).unwrap();
        assert!(trick.add(&card, 2).is_err());
        assert!(trick.add(&card, 1).is_ok());
        assert!(trick.add(&card, 3).is_err());
        assert!(trick.add(&card, 2).is_ok());
        assert!(trick.add(&card, 0).is_err());
    }
}
