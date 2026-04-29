pub struct FakeDb;

impl FakeDb {
    pub fn get_profile(&self, user_id: &String) -> String {
        String::from(format!("User:{user_id}"))
    }

}