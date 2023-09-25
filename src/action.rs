#[derive(Debug)]
pub enum Action {
    Publish,
    Consume
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Action::Publish, Action::Publish) => true,
            (Action::Consume, Action::Consume) => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}