#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum BlockType {
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "divider")]
    Divider,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "input")]
    Input,
}

#[derive(Serialize, Deserialize, Debug)]
enum TextType {
    #[serde(rename = "mrkdwn")]
    Markdown,
    #[serde(rename = "plain_text")]
    PlainText,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    r#type: TextType,
    text: String,
    emoji: bool,
}

impl Text {
    pub fn plain(text: &str, emoji: bool) -> Self {
        Text {
            r#type: TextType::PlainText,
            text: text.to_string(),
            emoji,
        }
    }
    pub fn markdown(text: &str, emoji: bool) -> Self {
        Text {
            r#type: TextType::Markdown,
            text: text.to_string(),
            emoji,
        }
    }
    pub fn label(text: &str) -> Self {
        Text {
            r#type: TextType::PlainText,
            text: text.to_string(),
            emoji: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum ElementType {
    #[serde(rename = "plain_text_input")]
    PlainTextInput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    r#type: ElementType,
    action_id: String,
    min_length: Option<u32>,
}

impl Element {
    pub fn new(action_id: &str, min_length: Option<u32>) -> Self {
        Element {
            r#type: ElementType::PlainTextInput,
            action_id: action_id.to_string(),
            min_length,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    r#type: BlockType,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    element: Option<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<Text>,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            r#type: BlockType::Divider,
            text: None,
            block_id: None,
            dispatch_action: None,
            element: None,
            label: None,
        }
    }
}

impl Block {
    pub fn header(text: &str) -> Self {
        Block {
            r#type: BlockType::Header,
            text: Some(Text::plain(text, true)),
            ..Default::default()
        }
    }
    pub fn section(text: &str) -> Self {
        Block {
            r#type: BlockType::Section,
            text: Some(Text::markdown(text, true)),
            ..Default::default()
        }
    }
    pub fn divider() -> Self {
        Block {
            ..Default::default()
        }
    }
    pub fn input(block_id: &str, dispatch_action: bool, element: Element, label: Text) -> Self {
        Block {
            r#type: BlockType::Input,
            block_id: Some(block_id.to_string()),
            dispatch_action: Some(dispatch_action),
            element: Some(element),
            label: Some(label),
            ..Default::default()
        }
    }
    pub fn guess_input() -> Self {
        let element = Element::new("guess", Some(2));
        let label = Text::label("Guess");
        Block::input("guess", true, element, label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialising_divider() {
        let block = Block::divider();
        let json = serde_json::to_string(&block).unwrap();
        assert_eq!(json, r#"{"type":"divider"}"#);
    }

    #[test]
    fn test_serialising_header() {
        let block = Block::header("Hello");
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "header",
  "text": {
    "type": "plain_text",
    "text": "Hello",
    "emoji": true
  }
}"#
        );
    }

    #[test]
    fn test_serialising_section() {
        let block = Block::section("Hello");
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "section",
  "text": {
    "type": "mrkdown",
    "text": "Hello",
    "emoji": true
  }
}"#
        );
    }

    #[test]
    fn test_serialising_input() {
        let element = Element::new("action-id", Some(2));
        let label = Text::label("label");

        let block = Block::input("block-id", true, element, label);
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "input",
  "block_id": "block-id",
  "dispatch_action": true,
  "element": {
    "type": "plain_text_input",
    "action_id": "action-id",
    "min_length": 2
  },
  "label": {
    "type": "plain_text",
    "text": "label",
    "emoji": true
  }
}"#
        );
    }
}
