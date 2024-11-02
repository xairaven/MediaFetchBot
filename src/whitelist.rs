use teloxide::types::User;

pub fn is_user_whitelisted(sender: &Option<User>, whitelist: &[u64]) -> bool {
    // Sender, empty for messages sent to channels.
    match sender {
        None => false,
        Some(user) => whitelist.contains(&user.id.0),
    }
}
