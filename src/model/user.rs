use crate::schema::users;
use validator::Validate;

#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
}

// Requests

#[derive(Deserialize, Debug, Validate)]
pub struct AddUserRequest {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateUserRequest {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_validation_add_user() {
        // Given
        let new_user = AddUserRequest {
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            email: "valid.email@email.com".to_string(),
        };

        // When
        let validation_result = new_user.validate();

        // Then
        assert!(validation_result.is_ok());
    }

    #[test]
    fn should_fail_validation_add_user() {
        // Given
        let new_user = AddUserRequest {
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            email: "dummy".to_string(),
        };

        // When
        let validation_result = new_user.validate();

        // Then
        assert!(validation_result.is_err());
        let validation_errors = validation_result.unwrap_err();
        assert!(validation_errors.field_errors().contains_key("email"));
    }

    #[test]
    fn should_pass_validation_update_user() {
        // Given
        let updated_user = UpdateUserRequest {
            id: 1,
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            email: "valid.email@email.com".to_string(),
        };

        // When
        let validation_result = updated_user.validate();

        // Then
        assert!(validation_result.is_ok());
    }

    #[test]
    fn should_fail_validation_update_user() {
        // Given
        let updated_user = UpdateUserRequest {
            id: 1,
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            email: "dummy".to_string(),
        };

        // When
        let validation_result = updated_user.validate();

        // Then
        assert!(validation_result.is_err());
        let validation_errors = validation_result.unwrap_err();
        assert!(validation_errors.field_errors().contains_key("email"));
    }
}