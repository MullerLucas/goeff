#[allow(unused, non_camel_case_types)]
pub enum GoeffStyle {
	CONTENT,
	CHAT,
	CHAT_OUTPUT_BOX,
}
impl AsRef<str> for GoeffStyle {
	fn as_ref(&self) -> &str {
		match &self {
			Self::CONTENT => "content",
			Self::CHAT => "chat",
			Self::CHAT_OUTPUT_BOX => "chat_output_box",
		}
	}
}
