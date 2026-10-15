#![allow(dead_code)]

use core::time;

fn main() {
    let mut client = Client::new();

    client.connect().unwrap();

    let token = Token::from_string("auth-token".to_string()).unwrap();
    client.authenticate(token).unwrap();

    client.perform_request().unwrap();

    let token = Token::from_string("auth-token".to_string()).unwrap();
    client.update_token(token).unwrap();

    client.logout().unwrap();
    client.disconnect().unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClientState {
    Disconnected,
    Connected,
    Authenticated,
}

struct Client {
    state: ClientState,
    token: Option<Token>,
}

impl Client {
    fn new() -> Self {
        Self {
            state: ClientState::Disconnected,
            token: None,
        }
    }

    fn connect(&mut self) -> Result<(), ClientError> {
        if self.state != ClientState::Disconnected {
            return Err(ClientError::InvalidState);
        }

        // connection logic
        if cfg!(test) {
            return Err(ClientError::ConnectionError);
        }

        self.state = ClientState::Connected;

        Ok(())
    }

    fn authenticate(&mut self, token: Token) -> Result<(), ClientError> {
        if self.state != ClientState::Connected {
            return Err(ClientError::InvalidState);
        }

        // authentication logic
        if cfg!(test) {
            return Err(ClientError::AuthError);
        }

        self.token = Some(token);
        self.state = ClientState::Authenticated;

        Ok(())
    }

    fn perform_request(&self) -> Result<(), ClientError> {
        if self.state != ClientState::Authenticated {
            return Err(ClientError::InvalidState);
        }

        println!("Performing request");
        std::thread::sleep(time::Duration::from_secs(2));
        println!("Request done");

        Ok(())
    }

    fn update_token(&mut self, new_token: Token) -> Result<(), ClientError> {
        if self.state != ClientState::Authenticated {
            return Err(ClientError::InvalidState);
        }

        println!("Updating token");

        if cfg!(test) {
            return Err(ClientError::AuthError);
        }

        self.token = Some(new_token);

        Ok(())
    }

    fn logout(&mut self) -> Result<(), ClientError> {
        if self.state != ClientState::Authenticated {
            return Err(ClientError::InvalidState);
        }

        self.token = None;
        self.state = ClientState::Connected;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), ClientError> {
        if self.state == ClientState::Disconnected {
            return Err(ClientError::InvalidState);
        }

        println!("Disconnecting");

        self.token = None;
        self.state = ClientState::Disconnected;

        Ok(())
    }
}

struct Token(String);

impl Token {
    fn from_string(raw_token: String) -> Result<Token, TokenParseError> {
        if raw_token.is_empty() || raw_token.len() > MAX_TOKEN_LENGTH {
            return Err(TokenParseError("Invalid raw token".to_string()));
        }

        Ok(Token(raw_token))
    }
}

const MAX_TOKEN_LENGTH: usize = 64;

#[derive(Debug)]
struct TokenParseError(String);

#[derive(Debug)]
enum ClientError {
    InvalidState,
    ConnectionError,
    AuthError,
}
