use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};
use std::{
    io::{StdoutLock, Write},
    u64,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Identification of the node that this message come from
    src: Option<String>,

    /// Identification of the node that this message is to
    dest: Option<String>,

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
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

pub struct Node {
    id: usize,
}

impl Node {
    pub fn reply(&mut self, input: Message, stdout: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let msg = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk {},
                    },
                };

                serde_json::to_writer(&mut *stdout, &msg).context("serialize response to init")?;
                stdout.write_all(b"\n").context("write trailing newline")?;

                self.id += 1;
            }
            Payload::Echo { echo } => {
                let msg = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };

                serde_json::to_writer(&mut *stdout, &msg).context("serialize response to init")?;
                stdout.write_all(b"\n").context("write trailing newline")?;

                self.id += 1;
            }
            Payload::InitOk { .. } => bail!("we doesn't expect InitOk back"),
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut node = Node { id: 0 };

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    for input in inputs {
        let input = input?;
        node.reply(input, &mut stdout)?
    }

    Ok(())
}
