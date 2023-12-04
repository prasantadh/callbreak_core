use standard_deck::{Card as Standard_Card, Suit};

use crate::Trick;

const MAX_SIZE: usize = 13;

pub struct Hand {
    cards: [Option<Card>; MAX_SIZE],
    size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    card: Standard_Card,
    playable: bool,
}

impl Hand {
    pub fn new() -> Hand {
        return Hand {
            cards: [None; MAX_SIZE],
            size: 0,
        };
    }

    pub fn add(&mut self, card: &Standard_Card) -> Result<(), &str> {
        if self.size >= MAX_SIZE {
            return Err("Hand is already full");
        }
        self.cards[self.size] = Some(Card {
            card: *card,
            playable: true,
        });
        self.size += 1;
        Ok(())
    }

    // the moves should be for a player
    pub fn get_moves(&self, trick: &Trick) -> Result<Vec<Standard_Card>, &str> {
        assert!(self.size == MAX_SIZE); // temporary soln to make sure hand has card

        let playables = self
            .cards
            .iter()
            .filter(|card| card.unwrap().playable)
            .map(|card| card.unwrap().card)
            .collect::<Vec<Standard_Card>>();

        let leader = trick.leader();
        let winner = trick.winner();
        if leader == None || winner == None {
            return Ok(playables);
        }
        let leader = leader.unwrap();
        let winner = winner.unwrap();

        let moves = |condition: Box<dyn Fn(&&Standard_Card) -> bool>| {
            playables
                .iter()
                .filter(|card| condition(card))
                .map(|card| *card)
                .collect::<Vec<Standard_Card>>()
        };
        let same_suit_winners = Box::new(|card: &&Standard_Card| {
            card.get_suit() == leader.get_suit() && card.get_rank() > winner.get_rank()
        });
        let same_suit_losers =
            Box::new(|card: &&Standard_Card| card.get_suit() == leader.get_suit());
        let spades = Box::new(|card: &&Standard_Card| {
            card.get_suit() == Suit::Spades && card.get_rank() > winner.get_rank()
        });
        let moves = [
            moves(same_suit_winners),
            moves(same_suit_losers),
            moves(spades),
            playables,
        ];
        for m in moves.iter() {
            if m.len() > 0 {
                return Ok(m.clone());
            }
        }
        return Err("no playable moves");
    }

    // to do implement an ordering?
    // is this even necessary because this should all be handled
    // client side
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.card == other.card && self.playable == other.playable
    }
}

#[cfg(test)]
mod tests {
    use standard_deck::{Card, Rank, Suit};

    use crate::{Hand, Trick};

    #[test]
    fn get_moves_works_for_basic_case() {
        let lead: usize = 0;
        let mut trick = Trick::new(0).unwrap();
        assert!(trick
            .add(&Card::new(&Rank::Two, &Suit::Clubs), lead)
            .is_ok());
        let mut hand = Hand::new();
        for rank in Rank::iter() {
            assert!(hand.add(&Card::new(&rank, &Suit::Clubs)).is_ok());
        }
        let moves = hand.get_moves(&trick);
        assert_eq!(moves.unwrap().len(), 12);

        assert!(trick
            .add(&Card::new(&Rank::King, &Suit::Clubs), lead + 1)
            .is_ok());
        let moves = hand.get_moves(&trick);
        assert_eq!(moves.unwrap().len(), 1);
    }
}
