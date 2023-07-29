use naia_hecs_shared::Replicate;

#[derive(Replicate)]
pub struct Marker;

impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

impl Marker {
    pub fn new() -> Self {
        Self::new_complete()
    }
}
