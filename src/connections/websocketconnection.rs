use ws::Sender;

pub struct Connection {
    pub ip: String,
    pub fingerprint: String,
    pub sender: Sender
}
