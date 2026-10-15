#![allow(dead_code)]
use core::time;

fn main() {
    let new_client = Client::new();
    let connected_client = new_client.connect().unwrap();

    let token = Token::try_from("auth-token").unwrap();
    let mut auth_client = connected_client.authenticate(token).unwrap();

    auth_client.perform_request();
    let token = Token::try_from("auth-token").unwrap();
    auth_client.update_token(token).unwrap();

    let unauthed_client = auth_client.logout();
    let _disconnected_client = unauthed_client.disconnect();
}

struct Disconnected;
struct Connected;
struct Authenticated {
    token: Token,
}

struct Client<State> {
    state: State,
}

impl Client<Disconnected> {
    fn new() -> Self {
        Self {
            state: Disconnected,
        }
    }

    fn connect(self) -> Result<Client<Connected>, ClientError> {
        // connection logic
        if cfg!(test) {
            Err(ClientError::ConnectionError)
        } else {
            Ok(Client { state: Connected })
        }
    }
}

impl Client<Connected> {
    fn authenticate(self, token: Token) -> Result<Client<Authenticated>, ClientError> {
        // auth logic
        if cfg!(test) {
            Err(ClientError::AuthError)
        } else {
            Ok(Client {
                state: Authenticated { token },
            })
        }
    }
}

impl Client<Authenticated> {
    fn perform_request(&self) {
        println!("Performing request");
        std::thread::sleep(time::Duration::from_secs(2));
        println!("Request done");
    }

    fn update_token(&mut self, new_token: Token) -> Result<(), ClientError> {
        println!("Updating token");
        if cfg!(test) {
            return Err(ClientError::AuthError);
        }

        self.state.token = new_token;
        Ok(())
    }

    fn logout(self) -> Client<Connected> {
        Client { state: Connected }
    }
}

trait ConnectedState {}

impl ConnectedState for Connected {}
impl ConnectedState for Authenticated {}

impl<State: ConnectedState> Client<State> {
    fn disconnect(self) -> Client<Disconnected> {
        println!("Disconnecting");
        Client {
            state: Disconnected,
        }
    }
}

struct Token(String);
impl TryFrom<&str> for Token {
    type Error = TokenParseError;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        if raw.is_empty() || raw.len() > MAX_TOKEN_LENGTH {
            return Err(TokenParseError("Invalid raw token".to_string()));
        }

        Ok(Token(raw.to_owned()))
    }
}

const MAX_TOKEN_LENGTH: usize = 64;
#[derive(Debug)]
struct TokenParseError(String);

#[derive(Debug)]
enum ClientError {
    ConnectionError,
    AuthError,
}
