use derive_more::{Deref, DerefMut};
use fixed::types::I60F4;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Deref, DerefMut)]
pub struct Position(pub mint::Point3<I60F4>);
