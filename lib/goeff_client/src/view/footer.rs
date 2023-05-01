use hell_core::error::HellResult;
use hell_mod_web_client::view::{Context, Element, ident::HellStyle, ElementContainer};


#[allow(unused)]
pub async fn create_footer(cx: Context) -> HellResult<Element> {
    let mut footer = Element::create_footer(cx)?.get();
    footer.add_classes(&[
        HellStyle::bg_secondary_400,
        HellStyle::txt_secondary_400,
        HellStyle::pos_fixed,
        HellStyle::inset_x_0,
        HellStyle::bottom_0,
    ])?;

    let mut footer_content = Element::create_div(cx)?.get();
    footer_content.add_classes(&[
        HellStyle::max_width_4xl,
        HellStyle::mgn_auto,
        HellStyle::height_10,
        HellStyle::flex,
        HellStyle::items_center,
        HellStyle::gap_2,
    ])?;
    footer.append_child(&footer_content)?;

    Ok(footer)
}
