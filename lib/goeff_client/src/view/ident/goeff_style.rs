#[allow(unused, non_camel_case_types)]
pub enum GoeffStyle {
	bg_user_400,
	bg_assistant_400,
	txt_user_400,
	txt_assistant_400,
	border_user_400,
	border_assistant_400,
	chat_input_gradient,
}
impl AsRef<str> for GoeffStyle {
	fn as_ref(&self) -> &str {
		match &self {
			Self::bg_user_400 => "bg_user_400",
			Self::bg_assistant_400 => "bg_assistant_400",
			Self::txt_user_400 => "txt_user_400",
			Self::txt_assistant_400 => "txt_assistant_400",
			Self::border_user_400 => "border_user_400",
			Self::border_assistant_400 => "border_assistant_400",
			Self::chat_input_gradient => "chat_input_gradient",
		}
	}
}
