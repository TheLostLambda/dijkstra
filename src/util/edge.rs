use super::*;

#[derive(Clone)]
pub struct Edge {
    pub link: (ID, ID),
    pub weight: Dist,
}
