use crate::dbs::user_repository::UserRepository;

pub struct UserService<T: UserRepository>{
    db: T
}

impl <T: UserRepository> UserService<T> {
    pub fn new(db: T) -> Self {
        UserService{db}
    }
    
    pub async fn multi_service_user_job(&self) {
        let user = self.db.get_users().await;
        // Do more things
    }
}