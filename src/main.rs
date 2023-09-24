use turmoil;
use uuid::Uuid;

enum Status {
    Leader,
    Unknown,
}

struct State {
    u: Uuid,
    send: Option<Uuid>,
    status: Status,
}

impl Default for State {
    fn default() -> Self {
        State {
            u: Uuid::new_v4(),
            send: None,
            status: Status::Unknown,
        }
    }
}

impl State {
    fn new() -> Self {
	Default::default()
    }
}

fn main() {
    let mut sim = turmoil::Builder::new().build();
}
