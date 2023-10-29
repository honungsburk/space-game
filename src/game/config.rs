//! Contains code to create configurations for the game.

/// A configuration for the game that is either true or false.
pub struct Flag {
    name: String,
    description: String,
    value: bool,
}

impl Flag {
    /// Creates a new flag.
    pub fn new(name: &str, description: &str, value: bool) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            value,
        }
    }

    /// Returns the name of the flag.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the flag.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Flip the value of the flag.
    pub fn flip(&mut self) {
        self.value = !self.value;
    }

    /// Returns true if the flag is on.
    pub fn is_on(&self) -> bool {
        self.value
    }

    /// Returns true if the flag is off.
    pub fn is_off(&self) -> bool {
        !self.is_on()
    }

    /// Sets the value of the flag.
    pub fn set(&mut self, value: bool) {
        self.value = value;
    }

    /// Turns the flag on.
    pub fn turn_on(&mut self) {
        self.set(true);
    }

    /// Turns the flag off.
    pub fn turn_off(&mut self) {
        self.set(false);
    }
}
