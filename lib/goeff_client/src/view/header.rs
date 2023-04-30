use hell_core::error::HellResult;
use hell_mod_web_client::view::{Element, ident::HellStyle, Context, ElementContainer};


pub async fn create_header(cx: Context) -> HellResult<Element> {
    let mut header = Element::create_header(cx)?.get();
    header.add_classes(&[
        HellStyle::bg_secondary_400,
        HellStyle::text_secondary_400,
        HellStyle::pos_fixed,
        HellStyle::inset_x_0,
        HellStyle::top_0,
    ])?;

    let mut header_content = Element::create_header(cx)?.get();
    header_content.add_classes(&[
        HellStyle::m_auto,
        HellStyle::max_w_4xl,
        HellStyle::flex,
        HellStyle::items_center,
        HellStyle::gap_2,
    ])?;
    header.append_child(&header_content)?;

    let mut title = Element::create_h1(cx)?.get();
    title.add_class(HellStyle::text_2xl)?;
    title.set_text_content(Some("Goeff Jipedy"));
    header_content.append_child(&title)?;

    let mut subtitle = Element::create_h4(cx)?.get();
    subtitle.set_text_content(Some("Powered by Hellmut"));
    header_content.append_child(&subtitle)?;

    Ok(header)
}
