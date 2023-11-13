use serde::{Deserialize, Serialize};
use std::{io::StdoutLock, u64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Identification of the node that this message come from
    src: String,

    /// Identification of the node that this message is to
    dest: String,

    /// The payload of the message
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    /// The identification of the message
    #[serde(rename = "msg_id")]
    id: Option<usize>,

    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

pub struct Node {
    id: usize,
}

impl Node {
    pub fn reply(
        &mut self,
        input: Message,
        stdout: &mut serde_json::Serializer<StdoutLock>,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                reply.serialize(stdout)?;

                self.id += 1;
            }
            Payload::EchoOk { echo: _ } => todo!(),
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let stdout = std::io::stdout().lock();
    let mut output = serde_json::Serializer::new(stdout);

    let mut state = Node { id: 0 };

    for input in inputs {
        let input = input?;
        state.reply(input, &mut output)?
    }

    Ok(())
}
