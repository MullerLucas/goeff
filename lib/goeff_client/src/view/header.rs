use hell_core::error::HellResult;
use hell_mod_web_client::view::{Element, ident::HellStyle, Context, ElementContainer};


pub async fn create_header(cx: Context) -> HellResult<Element> {
    let mut header = Element::create_header(cx)?.get();
    header.add_classes(&[
        HellStyle::bg_secondary_400,
        HellStyle::txt_secondary_400,
        HellStyle::pos_fixed,
        HellStyle::inset_x_0,
        HellStyle::top_0,
        HellStyle::height_16,
    ])?;

    let mut header_content = Element::create_div(cx)?.get();
    header_content.add_classes(&[
        HellStyle::pad_x_4,
        HellStyle::mgn_auto,
        HellStyle::max_width_3xl,
        HellStyle::flex,
        HellStyle::items_center,
        HellStyle::gap_2,
    ])?;
    header.append_child(&header_content)?;

    let mut title = Element::create_h1(cx)?.get();
    title.add_class(HellStyle::txt_2xl)?;
    title.set_text_content(Some("Goeff Gipety"));
    header_content.append_child(&title)?;

    // let mut subtitle = Element::create_h4(cx)?.get();
    // subtitle.set_text_content(Some("Powered by Hellmut"));
    // header_content.append_child(&subtitle)?;

    Ok(header)
}
