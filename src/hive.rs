pub mod hive {
    use crate::ant::ant::Ant;
    use rand::{self, Rng};
    use std::{borrow::Borrow, collections::HashMap, rc::Rc};
    pub struct Hive {
        pub ants: HashMap<usize, Rc<Ant>>,
        pub colonies: HashMap<String, Colony>,
    }
    #[derive(Clone)]
    pub struct Colony {
        pub ants: Vec<Rc<Ant>>,
        pub location: String,
        pub tunnels: Vec<String>, // Can be turned into ints for speed improvements
    }

    /// The Hive is the main interface and contains all ants and all colonies
    impl Hive {
        pub fn new(colonies: Vec<Colony>) -> Self {
            let mut hash: HashMap<String, Colony> = HashMap::new();
            colonies.iter().for_each(|colony| {
                (hash.insert(colony.location.clone(), colony.clone()));
            });
            Hive {
                ants: HashMap::new(),
                colonies: hash,
            }
        }

        fn initialize_colonies(colonies: Vec<Colony>) -> HashMap<String, Colony> {
            let mut hash: HashMap<String, Colony> = HashMap::new();
            for colony in colonies {
                hash.insert(colony.location.clone(), colony);
            }
            hash
        }

        // Creates an ant and places it in a random colony
        pub fn add_ant(&mut self, id: &usize) {
            let rand_index = rand::thread_rng().gen_range(0..self.colonies.len());
            let colonies: Vec<String> = self.colonies.keys().cloned().collect();
            let name = &colonies[rand_index];
            // todo: optimize random placing (fix: repetitive hashmap key cloning)
            self.ants
                .insert(id.clone(), Rc::new(Ant::new(id.clone(), name.clone())));
            let colony = self.colonies.get_mut(name).unwrap();
            colony.add_ant(self.ants.get(&id).unwrap().clone());
        }

        // Destroys an ant given an id
        pub fn destroy_ant(&mut self, id: &usize) {
            self.ants.remove(id);
        }

        // Destroys ants in the colony, and the colony
        fn destroy_colony(&mut self, colony_name: &String) {
            let colony = self.colonies.get(colony_name).unwrap();
            let ants = colony.ants.clone();
            for ant in ants {
                self.destroy_ant(&ant.id);
            }
            self.colonies.remove(colony_name);
        }

        fn move_all_ants(&mut self) {
            let ants: Vec<Rc<Ant>> = self.ants.values().cloned().collect();
            for ant in ants {
                let ant: &Ant = ant.borrow();
                let mut ant = ant.clone();
                ant.move_ant(self);
            }
        }
        /// Main simulator logic, iterates through all ants and destroys any colonies if necessary
        pub fn simulate(&mut self) {
            self.move_all_ants();
            for (key, value) in &self.colonies.clone() {
                if value.ants.len() >= 2 {
                    let ants: Vec<usize> = value.ants.iter().map(|ant| ant.id).collect();
                    println!("{} has been destroyed by ants: {:?}", key, ants);
                    self.destroy_colony(key);
                }
            }
        }
    }

    /// The Colony holds the World Map information that the Hive uses
    impl Colony {
        pub fn new(location: String, tunnels: Vec<String>) -> Self {
            Colony {
                ants: Vec::new(),
                location,
                tunnels,
            }
        }

        fn add_ant(&mut self, ant: Rc<Ant>) {
            self.ants.push(ant);
        }
    }
}
