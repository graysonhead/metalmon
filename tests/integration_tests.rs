extern crate metalmon;
use metalmon;

#[cfg(test)]
mod integration_tests {
    use super::*;
    #[test]
    fn db_connection_test() {
        let _conn = metalmon::database::establish_connection();
    }

    #[test]
    fn db_create_and_delete_user() {
        let connection = metalmon::database::establish_connection();
        let username = String::from("testuser");
        let password = String::from("password");
        let user = metalmon::database::create_user(&connection, &username, &password);
        let num_deleted = metalmon::database::delete_user(&connection, &username);
        assert_eq!(num_deleted, 1);
    }
}

