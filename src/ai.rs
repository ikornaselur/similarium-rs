use crate::SimilariumError;
use async_openai::{
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionResponseFormat,
        ChatCompletionResponseFormatType, CreateChatCompletionRequestArgs,
    },
    Client,
};
use serde::{Deserialize, Serialize};

const GAME_EXPLANATION: &str = "Similarium is a secret word guessing game. Multiple players try to guess the secret word, with each guess being ranked by how close it is to the secret. Closeness to the secret is calculated with Word2Vec, to get a semantic similarity to the secret. So 'love' would be close to 'hate' for example. The closest word will rank 1, next closest and so on, all the way up to about 100 thousand.";
const GAME_STATE: &str = "I will provide the state of the game by listing up to top 15 guesses made so far, each guess will be in its own line in the format '123 Apple', which means the word 'Apple' ranked at 123 (122 words are closer to the secret).";
const RETURN_FORMAT: &str = "Your response should be a valid JSON object in the form of '{{\"message\": \"<the message>\"}}'.";
const CLARIFICATIONS: &str = "Users will be referenced by user ids from Slack, in the form of U1234567890. Any reference you make to a user has to be the full ID, surrounded by angle brackets and prefixed with an 'at' symbol. For example, for the user U1234567890 you need to reference them only as <@U1234567890>.";

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

pub async fn get_hint(
    guess_count: usize,
    top_word: &str,
    top_rank: usize,
    secret: &str,
    closest_words: Vec<&str>,
) -> Result<Message, SimilariumError> {
    // Format the closest words in a string, where each word is in it's own line with the index + 1
    // first. So the top line would be '1 Apple' if Apple is the closest word
    let closest_words_str = closest_words
        .iter()
        .enumerate()
        .map(|(idx, word)| format!("{} {}", idx + 1, word))
        .collect::<Vec<String>>()
        .join("\n");

    let hint_prompt = format!("The players have made {guess_count} guesses with the closest guess being {top_word} at {top_rank}. The secret is {secret}. You need to provide some hint to the secret *without* giving away the secret.\nThe 10 words closest to the secret are:\n{closest_words_str})");

    let full_prompt = format!("{GAME_EXPLANATION}\n{GAME_STATE}\n{hint_prompt}\n{RETURN_FORMAT}");

    // Make the request
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4o")
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::JsonObject,
        })
        .messages([ChatCompletionRequestSystemMessageArgs::default()
            .content(full_prompt)
            .build()?
            .into()])
        .build()?;

    log::debug!("{}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;
    let content = response.choices[0]
        .message
        .content
        .clone()
        .map_or_else(|| ai_error!("Unable to get response from model"), Ok)?;

    Ok(serde_json::from_str(&content)?)
}

pub async fn get_celebration(
    guess_count: i64,
    user_id: &str,
    word: &str,
    word_rank: i64,
    bucket: i64,
) -> Result<Message, SimilariumError> {
    let user_status_prompt = format!("The user '{user_id}' just had a milestone by guessing the first word in the top {bucket} words! There have been a total of {guess_count} guesses in the game. They made the guess '{word}' which ranks {word_rank}. Make a celebration for this user that is in the format:\n'<Celebration>! <reference user and the word they guessed and how many guesses it took>'\nAn example is:\n'Our top guessers strike again! <user> just guessed '<word>' in the top {bucket} words in 41 guesses!'\nKeep in mind that if the number of guesses is low (under 25), it's impressive, if the number of guesses is high (over 150) it can be a relief. Keep it fun, witty and slightly over the top. Include 1-3 emojis at the end that are relevant.");

    let full_prompt =
        format!("{GAME_EXPLANATION}\n{GAME_STATE}\n{user_status_prompt}\n{RETURN_FORMAT}\n{CLARIFICATIONS}");

    // Make the request
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-4o")
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::JsonObject,
        })
        .messages([ChatCompletionRequestSystemMessageArgs::default()
            .content(full_prompt)
            .build()?
            .into()])
        .build()?;

    log::debug!("{}", serde_json::to_string(&request)?);

    let response = client.chat().create(request).await?;
    let content = response.choices[0]
        .message
        .content
        .clone()
        .map_or_else(|| ai_error!("Unable to get response from model"), Ok)?;

    Ok(serde_json::from_str(&content)?)
}
