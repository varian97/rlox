#[derive(Clone, Debug)]
pub struct RloxErrorDetail {
    pub line_number: usize,
    pub message: String,
}

impl RloxErrorDetail {
    pub fn new(line_number: usize, message: String) -> Self {
        RloxErrorDetail {
            line_number,
            message,
        }
    }

    pub fn report(self: &Self) {
        eprintln!("[Line {}] Error: {}", self.line_number, self.message);
    }
}
