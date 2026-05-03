#[derive(Debug, Clone, PartialEq)]
pub struct ChatGenerationConfig {
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stop_sequences: Vec<String>,
    pub seed: Option<u64>,
}

impl ChatGenerationConfig {
    /// Perform the `disabled` operation as implemented by this function.
    ///
    /// # Returns
    /// Returns the value produced by this function.
    pub fn disabled() -> Self {
        Self {
            max_output_tokens: None,
            temperature: None,
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stop_sequences: Vec::new(),
            seed: None,
        }
    }
}
