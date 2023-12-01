pub mod ant {
    use rand::seq::SliceRandom;
    use std::rc::Rc;

    use crate::hive::hive::Hive;
    #[derive(Clone, Debug)]

    /// Ant holds basic structure about itself and its location
    pub struct Ant {
        pub id: usize,
        pub location: String,
        move_counter: usize,
    }

    impl Ant {
        pub fn new(id: usize, location: String) -> Self {
            Ant {
                id,
                location,
                move_counter: 0,
            }
        }

        pub fn add_counter(&mut self) {
            self.move_counter += 1;
        }

        /// Moves an ant through a tunnel, if possible.
        ///
        /// # Arguments
        /// * '&mut self' - self reference
        /// * '&mut Hive' - hive reference
        ///
        /// # Returns
        /// * 'Option<&String>' - Ant's new location
        ///
        /// # Notes
        /// * Destroys trapped ants (ie. no tunnels available)
        pub fn move_ant(&mut self, hive: &mut Hive) -> Option<&String> {
            // check to see if there are any tunnels available, if not return none and destroy ant
            // if available: take ant, add counter, move to new location and return location reference

            let mut colony = hive.colonies.get_mut(&self.location).unwrap().clone();
            let mut tunnels = colony.tunnels.clone();
            tunnels.shuffle(&mut rand::thread_rng()); // randomize tunnels
            for tunnel in tunnels {
                // if tunneled colony exists
                if let Some(_) = hive.colonies.get_mut(&tunnel) {
                    self.location = tunnel.clone();
                    self.add_counter();
                    colony.ants.retain(|ant| ant.id != self.id); // remove ant
                    hive.colonies
                        .get_mut(&tunnel)
                        .unwrap()
                        .ants
                        .push(Rc::new(self.clone())); // push ant
                    return Some(&self.location);
                } else {
                    hive.destroy_ant(&self.id);
                }
            }

            None
        }
    }
}
