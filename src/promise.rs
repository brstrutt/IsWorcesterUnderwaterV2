
#[derive(Clone, Copy, PartialEq)]
pub enum Promise<T: Clone> {
    Loading,
    Data(T),
    Error,
}

impl<T: Clone> Promise<T> {
    pub fn is_loading(&self) -> bool {
        match self {
            Promise::Loading => true,
            _ => false,
        }
    }

    pub fn as_result(&self) -> Result<T, String> {
        match self {
            Promise::Loading => Err(String::from("Promise was turned into Future while still loading. Dont do that.")),
            Promise::Data(data) => Ok(data.clone()),
            // TODO: Store the error string in the object. This will break the "Copy" trait, and you'll need to figure out how to pass it as a prop without that trait.
            Promise::Error => Err(String::from("An error happened. I don't know what. Check the logs.")),
        }
    }
}
