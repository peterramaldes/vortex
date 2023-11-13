use std::io::StdoutLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    /// Identification of the node that this message come from
    src: String,

    /// Identification of the node that this message is to
    dest: String,

    /// The payload of the message
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    /// The identification of the message
    #[serde(rename = "msg_id")]
    id: u64,

    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        /// The message that will be apper on STDOUT
        echo: String,
    },
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    for input in inputs {
        let input = input?;
        println!("{:?}", input);
    }

    Ok(())
}
