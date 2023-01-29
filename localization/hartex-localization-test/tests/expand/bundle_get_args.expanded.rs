fn main() {
    let irrelevant = 1;
    let irrelevant = bundle.get_message("bundle-get-args-message").unwrap();
    let mut irrelevant2 = Vec::new();
    let mut args = hartex_localization_core::types::LocalizationArgs::new();
    args.set("irrelevant", irrelevant);
    let irrelevant = bundle
        .format_pattern(irrelevant.value().unwrap(), Some(&args), &mut errors);
    let irrelevant = irrelevant.trim();
    let irrelevant = bundle.get_term("bundle-get-args-term").unwrap();
    let mut irrelevant2 = Vec::new();
    let mut args = hartex_localization_core::types::LocalizationArgs::new();
    args.set("irrelevant", irrelevant);
    let irrelevant = bundle.format_pattern(irrelevant.value(), Some(&args), &mut errors);
    let irrelevant = irrelevant.trim();
}
