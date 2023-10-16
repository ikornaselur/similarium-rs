use crate::{
    models::GuessContext,
    slack_client::utils::{formatted_rank, get_progress_bar},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum BlockType {
    #[serde(rename = "section")]
    Section,
    #[serde(rename = "divider")]
    Divider,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "context")]
    Context,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum TextType {
    #[serde(rename = "mrkdwn")]
    Markdown,
    #[serde(rename = "plain_text")]
    PlainText,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Text {
    r#type: TextType,
    text: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum ElementType {
    #[serde(rename = "plain_text_input")]
    PlainTextInput,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ContextElement {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt_text: Option<String>,
}

impl ContextElement {
    pub fn text(text: &str) -> Self {
        ContextElement {
            r#type: "mrkdwn".to_string(),
            text: Some(text.to_string()),
            image_url: None,
            alt_text: None,
        }
    }
    pub fn image(image_url: &str, alt_text: &str) -> Self {
        ContextElement {
            r#type: "image".to_string(),
            text: None,
            image_url: Some(image_url.to_string()),
            alt_text: Some(alt_text.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    r#type: String,
    block_id: String,
    elements: Vec<ContextElement>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    elements: Option<Vec<ContextElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<Text>>,
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
            elements: None,
            fields: None,
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

    pub fn section(text: &str, fields: Option<Vec<&str>>) -> Self {
        match fields {
            Some(fields) => Block {
                r#type: BlockType::Section,
                text: Some(Text::markdown(text, false)),
                fields: Some(fields.iter().map(|f| Text::markdown(f, false)).collect()),
                ..Default::default()
            },
            None => Block {
                r#type: BlockType::Section,
                text: Some(Text::markdown(text, false)),
                ..Default::default()
            },
        }
    }

    #[allow(dead_code)]
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
        let element = Element::new("submit-guess", Some(2));
        let label = Text::label("Guess");
        Block::input("guess", true, element, label)
    }

    pub fn guess_context(base_id: &str, context: GuessContext, game_active: bool) -> Self {
        let block_id = format!("guess-{}-{}", base_id, context.word);

        let progress_bar = if context.rank < 1000 {
            get_progress_bar(1000 - context.rank as usize, 1000, 6).unwrap()
        } else {
            get_progress_bar(0, 1000, 6).unwrap()
        };

        let word_element = match (context.is_secret, game_active) {
            (false, _) => format!("*{}*", context.word),
            (true, true) => "*Secret will be revealed at the end* :see_no_evil:".to_string(),
            (true, false) => format!("*{}*", context.word),
        };

        let elements = vec![
            ContextElement::image(&context.profile_photo, &context.username),
            ContextElement::text(&format!("{}{}", progress_bar, formatted_rank(context.rank))),
            ContextElement::text(&word_element),
        ];

        Block {
            r#type: BlockType::Context,
            block_id: Some(block_id),
            elements: Some(elements),
            ..Default::default()
        }
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
        let block = Block::section("Hello", None);
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "section",
  "text": {
    "type": "mrkdwn",
    "text": "Hello"
  }
}"#
        );
    }

    #[test]
    fn test_serialising_section_with_fields() {
        let block = Block::section("Hello", Some(vec!["field1", "field2"]));
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "section",
  "text": {
    "type": "mrkdwn",
    "text": "Hello"
  },
  "fields": [
    {
      "type": "mrkdwn",
      "text": "field1"
    },
    {
      "type": "mrkdwn",
      "text": "field2"
    }
  ]
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

    #[test]
    fn test_guess_context() {
        let context = GuessContext {
            word: "word".to_string(),
            guess_num: 3,
            profile_photo: "photo".to_string(),
            username: "username".to_string(),
            similarity: 87.12598124,
            rank: 251,
            is_secret: false,
        };

        let block = Block::guess_context("base-id", context, true);
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "context",
  "block_id": "guess-base-id-word",
  "elements": [
    {
      "type": "image",
      "image_url": "photo",
      "alt_text": "username"
    },
    {
      "type": "mrkdwn",
      "text": ":p8::p8::p8::p8::p4::p0:      251"
    },
    {
      "type": "mrkdwn",
      "text": "*word*"
    }
  ]
}"#
        );
    }

    #[test]
    fn test_guess_context_with_secret_in_active_game() {
        let context = GuessContext {
            word: "word".to_string(),
            guess_num: 3,
            profile_photo: "photo".to_string(),
            username: "username".to_string(),
            similarity: 87.12598124,
            rank: 0,
            is_secret: true,
        };
        let block = Block::guess_context("base-id", context, true);
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "context",
  "block_id": "guess-base-id-word",
  "elements": [
    {
      "type": "image",
      "image_url": "photo",
      "alt_text": "username"
    },
    {
      "type": "mrkdwn",
      "text": ":p8::p8::p8::p8::p8::p8:        :tada:"
    },
    {
      "type": "mrkdwn",
      "text": "*Secret will be revealed at the end* :see_no_evil:"
    }
  ]
}"#
        );
    }

    #[test]
    fn test_guess_context_with_secret_in_inactive_game() {
        let context = GuessContext {
            word: "word".to_string(),
            guess_num: 3,
            profile_photo: "photo".to_string(),
            username: "username".to_string(),
            similarity: 87.12598124,
            rank: 0,
            is_secret: true,
        };
        let block = Block::guess_context("base-id", context, false);
        let json = serde_json::to_string_pretty(&block).unwrap();
        assert_eq!(
            json,
            r#"{
  "type": "context",
  "block_id": "guess-base-id-word",
  "elements": [
    {
      "type": "image",
      "image_url": "photo",
      "alt_text": "username"
    },
    {
      "type": "mrkdwn",
      "text": ":p8::p8::p8::p8::p8::p8:        :tada:"
    },
    {
      "type": "mrkdwn",
      "text": "*word*"
    }
  ]
}"#
        );
    }
}
