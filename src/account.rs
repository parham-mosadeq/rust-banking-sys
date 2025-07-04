#[derive(Debug, Clone)]
pub struct Account {
    pub id: u32,
    pub balance: i32,
    pub holder: String,
}

impl Account {
    pub fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}
