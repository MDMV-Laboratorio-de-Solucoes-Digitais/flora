// Topic naming conventions for Valkey pub/sub
// Example: org:{org_id}:channel:{channel_id}

pub fn organization_topic(org_id: &str) -> String {
    format!("org:{}", org_id)
}

pub fn channel_topic(org_id: &str, channel_id: &str) -> String {
    format!("org:{}:channel:{}", org_id, channel_id)
}

pub fn user_topic(org_id: &str, user_id: &str) -> String {
    format!("org:{}:user:{}", org_id, user_id)
}
