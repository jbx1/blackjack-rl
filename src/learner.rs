use crate::{BlackjackAction, BlackjackState, QTable};

pub struct Learner {
    q_table: QTable<BlackjackState, BlackjackAction>,
}

impl Learner {
    pub fn new() -> Learner {
        Learner { q_table: QTable::new(0.0) }
    }

    pub fn new_trained<F>(run_episode: F) -> Learner
        where F: Fn(&mut QTable<BlackjackState, BlackjackAction>, usize) -> (i32, f64) {
        let mut learner = Learner::new();
        learner.train(run_episode);
        return learner;
    }

    pub fn train<F>(&mut self, run_episode: F)
        where F: Fn(&mut QTable<BlackjackState, BlackjackAction>, usize) -> (i32, f64) {
        let episodes = 500000;

        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;
        let mut avg_error = 0.0;
        let mut count = 0.0;
        for i in 0..episodes {
            if i % 1000 == 0 && i > 0 {
                println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", i, wins, losses, draws, avg_error);
                wins = 0;
                losses = 0;
                draws = 0;
            }

            count = count + 1.0;
            let (reward, error) = run_episode(&mut self.q_table, i);
            if reward > 0 {
                wins += 1;
            } else if reward < 0 {
                losses += 1;
            } else {
                draws += 1;
            }

            avg_error = avg_error + (error - avg_error) / count;
        }

        println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", episodes - 1, wins, losses, draws, avg_error);

        let q_values = self.q_table.get_all_values();
        println!("Total state action values: {:?}", q_values.len());
        for value in q_values {
            println!("{:?}", value);
        }
    }

    pub fn print_strategy(&self) {
        self.print_strategy_ace(false);
        self.print_strategy_ace(true);
    }

    fn print_strategy_ace(&self, ace: bool) {
        let policy = self.q_table.get_policy();

        println!("\nAce: {:?}", ace);
        print!("   |");
        for header in 2u8..=10 {
            print!(" {} |", header);
        }
        println!(" A |");
        println!("---------------------------------------------");
        for player in (12u8 ..=20).rev() {
            print!("{} |", player);
            for dealer in 2u8..=11 {
                let state = BlackjackState { player, dealer, ace };

                if dealer == 10 {
                    print!(" ");
                }
                match policy.get(&state) {
                    None => print!(" - |"),
                    Some(BlackjackAction::Hit) => print!(" H |"),
                    Some(BlackjackAction::Stand) =>  print!(" S |")
                }
            }
            println!();
        }
        println!("---------------------------------------------");
        println!();
    }
}

