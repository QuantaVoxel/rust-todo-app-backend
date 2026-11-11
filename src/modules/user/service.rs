use crate::modules::user::dtos::User;

pub fn get_all_users() -> Vec<User> {
    vec![
        User {
            id: 0,
            email: "user1@gmail.com".to_string(),
            password: "password".to_string(),
            first_name: "Wawan".to_string(),
            last_name: Some("Julianto".to_string()),
        },
        User{
            id: 2,
            email: "user2@gmail.com".to_string(),
            password: "password".to_string(),
            first_name: "User".to_string(),
            last_name: Some("Dua".to_string()),
        }
    ]
}