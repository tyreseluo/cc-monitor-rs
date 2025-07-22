#[derive(Clone, Debug)]
pub struct DailyCost {
    pub date: String,
    pub cost: f64,
    pub sessions: u32,
}