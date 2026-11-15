use macros::{CheckMate, Pawn};
use macros_core::factory::Pawn;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Role {
    Developer,
    Admin,
    User,
    Guest,
}

#[derive(CheckMate, Debug)]
#[allow(dead_code)] // cuz we want to test macro work here or not
struct FullRoleCoverage {
    id: i64,

    #[validating("required|email|max:180|lowercase")]
    email: String,

    #[validating("nullable|min:8|max:128|confirmed:password_confirmation|uppercase")]
    password: Option<String>,

    password_confirmation: Option<String>,

    #[validating("size:10", "ascii", "alphanumeric")]
    code: String,

    #[validating("url|ip")]
    endpoint: String,

    #[validating("hex_color|starts_with:#|ends_with:ff")]
    color: String,

    #[validating("in:admin,user,guest|not_in:banned,suspended")]
    role: String,

    // #[validating("unique:users,email,id|exists:users,email")]
    // user_ref: String,
    #[validating("file|image|mimetypes:image/png,image/jpeg|extensions:png,jpg")]
    avatar: String,

    #[validating("date|datetime|time")]
    published_at: String,

    #[validating("before:2026-01-01", "after:2024-01-01")]
    date_range: String,

    #[validating("json")]
    metadata: String,

    #[validating("array")]
    test: HashMap<String, String>,
}
#[allow(dead_code)]
#[derive(Debug, Pawn)]
struct UserFactory {
    #[fake(name)]
    name: String,
    #[fake(username)]
    username: String,
    #[fake(password(length = 8))]
    password: String,
    #[fake(email)]
    email: String,
    #[fake(lorem(words = 20))]
    bio: String,
    #[generator(take_role)]
    role: Role,
    #[value("2020-01-01")]
    created_at: String,
    #[fake(date)]
    updated_at: String,
    #[fake(betweenDatetime(start = "2025-01-01T00:00:00Z", end = "2026-12-31T23:59:59Z"))]
    register_at: String,
    #[fake(afterDatetime(after = "2025-01-01T00:00:00Z"))]
    blocked_at: String,
    #[fake(beforeDatetime(before = "2025-01-01T00:00:00Z"))]
    base_datetime: String,
}

/// Picks a role at random for each generated user.
fn take_role() -> Role {
    use fake::Fake;
    match (0u8..4u8).fake::<u8>() {
        0 => Role::Developer,
        1 => Role::Admin,
        2 => Role::User,
        _ => Role::Guest,
    }
}

#[test]
fn test_all_roles_parsed() {
    // 10 users with random data -> Vec<UserFactory>
    let users = UserFactory::factory().count(10).create();

    assert_eq!(users.len(), 10);

    // A single user forced into the Admin role -> UserFactory
    let admin = UserFactory::factory()
        .state(|u| {
            u.role = Role::Admin;
        })
        .create();
    assert_eq!(admin.role, Role::Admin);
}
