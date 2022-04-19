use crate::{BlackjackAction, BlackjackState, QTable};

pub struct Trainer {
    q_table: QTable<BlackjackState, BlackjackAction>
}

impl Trainer {
    pub fn new() -> Trainer {
        Trainer{ q_table: QTable::new(0.0) }
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
            if i % 1000 == 0 {
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

        println!("\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\",\"{:?}\"", episodes-1, wins, losses, draws, avg_error);

        let q_values = self.q_table.get_all_values();
        println!("Total state action values: {:?}", q_values.len());
        for value in q_values {
            println!("{:?}", value);
        }
    }
}

