#[derive(Debug)]
pub enum Error {
    BadDatagram(String),
}

#[derive(Debug, PartialEq)]
pub enum Datagram {
    Subscribe(SubscribeDatagram),
    Unsubscribe(UnsubscribeDatagram),
    Publish(PublishDatagram),
    Error(String),
}

impl Datagram {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let mut iter = s.splitn(2, '|');
        match (iter.next(), iter.next()) {
            (Some("S"), Some(rest)) => Ok(Datagram::Subscribe(SubscribeDatagram::parse(rest))),
            (Some("U"), Some(rest)) => Ok(Datagram::Unsubscribe(UnsubscribeDatagram::parse(rest))),
            (Some("P"), Some(rest)) => Ok(Datagram::Publish(PublishDatagram::parse(rest)?)),
            (Some("E"), Some(rest)) => Ok(Datagram::Error(String::from(rest))),
            _ => Err(Error::BadDatagram(format!(
                "Could not parse Datagram: {}",
                s
            ))),
        }
    }

    pub fn serialize(&self) -> String {
        match self {
            Datagram::Subscribe(c) => format!("S|{}", c.serialize()),
            Datagram::Unsubscribe(c) => format!("U|{}", c.serialize()),
            Datagram::Publish(d) => format!("P|{}", d.serialize()),
            Datagram::Error(e) => format!("E|{}", e),
        }
    }

    pub fn subscribe<C: Into<String>>(channel: C) -> Self {
        Datagram::Subscribe(SubscribeDatagram {
            channel: channel.into(),
        })
    }

    #[cfg(test)]
    pub fn unsubscribe<C: Into<String>>(channel: C) -> Self {
        Datagram::Unsubscribe(UnsubscribeDatagram {
            channel: channel.into(),
        })
    }

    #[cfg(test)]
    pub fn publish<C, N, M>(channel: C, display_name: N, message: M) -> Self
    where
        C: Into<String>,
        N: Into<String>,
        M: Into<String>,
    {
        Datagram::Publish(PublishDatagram {
            channel: channel.into(),
            display_name: display_name.into(),
            message: message.into(),
        })
    }

    #[cfg(test)]
    pub fn error<M: Into<String>>(message: M) -> Self {
        Datagram::Error(message.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct SubscribeDatagram {
    pub channel: String,
}

impl SubscribeDatagram {
    pub fn parse(s: &str) -> Self {
        SubscribeDatagram {
            channel: String::from(s),
        }
    }

    pub fn serialize(&self) -> String {
        self.channel.to_owned()
    }
}

#[derive(Debug, PartialEq)]
pub struct UnsubscribeDatagram {
    pub channel: String,
}

impl UnsubscribeDatagram {
    pub fn parse(s: &str) -> Self {
        UnsubscribeDatagram {
            channel: String::from(s),
        }
    }

    pub fn serialize(&self) -> String {
        self.channel.to_owned()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PublishDatagram {
    pub channel: String,
    pub message: String,
    pub display_name: String,
}

impl PublishDatagram {
    pub fn parse(s: &str) -> Result<Self, Error> {
        let mut iter = s.splitn(3, '|');
        match (iter.next(), iter.next(), iter.next()) {
            (Some(channel), Some(display_name), Some(message)) => Ok(PublishDatagram {
                channel: String::from(channel),
                message: String::from(message),
                display_name: String::from(display_name),
            }),
            _ => Err(Error::BadDatagram(format!(
                "Could not parse PublishDatagram: {}",
                s
            ))),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}|{}|{}", self.channel, self.display_name, self.message)
    }

    pub fn copy(&self) -> Self {
        Self::parse(&self.serialize()).expect("Parsing error in copy...")
    }
}

#[cfg(test)]
mod protocol_tests {
    use super::*;

    #[test]
    fn test_subscribe_parse() {
        let message = "S|rust_club";
        let req = Datagram::parse(message).unwrap();
        assert_eq!(req, Datagram::subscribe("rust_club"));
    }

    #[test]
    fn test_subscribe_serialize() {
        let req = Datagram::subscribe("rust_club");
        assert_eq!(req.serialize(), "S|rust_club");
    }

    #[test]
    fn test_unsubscribe_parse() {
        let message = "U|rust_club";
        let req = Datagram::parse(message).unwrap();
        assert_eq!(
            req,
            Datagram::Unsubscribe(UnsubscribeDatagram {
                channel: String::from("rust_club")
            })
        );
    }

    #[test]
    fn test_unsubscribe_serialize() {
        let req = Datagram::unsubscribe("rust_club");
        assert_eq!(req.serialize(), "U|rust_club");
    }

    #[test]
    fn test_publish_parse() {
        let message = "P|rust_club|me|hello world! ||||| yo";
        let req = Datagram::parse(message).unwrap();
        assert_eq!(
            req,
            Datagram::Publish(PublishDatagram {
                channel: String::from("rust_club"),
                display_name: String::from("me"),
                message: String::from("hello world! ||||| yo"),
            })
        );
    }

    #[test]
    fn test_publish_serialize() {
        let req = Datagram::publish("rust_club", "me", "hello world! ||||| yo");
        assert_eq!(req.serialize(), "P|rust_club|me|hello world! ||||| yo");
    }

    #[test]
    fn test_error_parse() {
        let message = "E|some_error!";
        let req = Datagram::parse(message).unwrap();
        assert_eq!(req, Datagram::error("some_error!"));
    }

    #[test]
    fn test_error_serialize() {
        let req = Datagram::error("some_error!");
        assert_eq!(req.serialize(), "E|some_error!");
    }

    #[test]
    fn test_subscribe_datagram_parsing() {
        let message = "some_fake_channel";
        let req = SubscribeDatagram::parse(message);
        assert_eq!(
            req,
            SubscribeDatagram {
                channel: String::from("some_fake_channel")
            }
        );
    }

    #[test]
    fn test_unsubscribe_datagram_parsing() {
        let message = "some_fake_channel";
        let req = UnsubscribeDatagram::parse(message);
        assert_eq!(
            req,
            UnsubscribeDatagram {
                channel: String::from("some_fake_channel")
            }
        );
    }

    #[test]
    fn test_publish_datagram_parsing() {
        let message = "rust_club|me|hello world! ||||| yo";
        let req = PublishDatagram::parse(message).unwrap();
        assert_eq!(
            req,
            PublishDatagram {
                channel: String::from("rust_club"),
                display_name: String::from("me"),
                message: String::from("hello world! ||||| yo"),
            }
        );
    }
}
