#[allow(unused)]
pub struct Reject {
    message: String,
    code: char,
    reason: String,
    extra_data: [u8; 32],
}

// TODO: maybe implement?
