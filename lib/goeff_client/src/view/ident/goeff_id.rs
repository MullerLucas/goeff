#[allow(unused, non_camel_case_types)]
pub enum GoeffId {
	header,
	footer,
}
impl AsRef<str> for GoeffId {
	fn as_ref(&self) -> &str {
		match &self {
			Self::header => "header",
			Self::footer => "footer",
		}
	}
}
