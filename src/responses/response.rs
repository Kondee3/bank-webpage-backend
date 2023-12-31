use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseState {
    pub state: State,
}
#[derive(Serialize)]
pub enum State {
    SUCCESS,
    DUPLICATE,
    NOTFOUND,
    WRONGPASSWORD,
    EMPTY,
}
