use hartex_localization_macros::bundle_get_args;
fn main() {
    let irrelevant = 1;
    let irrelevant = bundle
        .get_message("bundle-get-args-message")
        .ok_or(
            miette::Report::msg({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "key `{0}` not found for locale `{1}`",
                        "bundle-get-args-message", bundle.locales[0],
                    ),
                );
                res
            }),
        )?;
    let mut irrelevant2 = Vec::new();
    let mut args = hartex_localization_core::types::LocalizationArgs::new();
    args.set("irrelevant", irrelevant);
    let irrelevant = bundle
        .format_pattern(irrelevant.value().unwrap(), Some(&args), &mut errors);
    let irrelevant = irrelevant.trim();
    let irrelevant = bundle
        .get_term("bundle-get-args-term")
        .ok_or(
            miette::Report::msg({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "key `{0}` not found for locale `{1}`", "bundle-get-args-term",
                        bundle.locales[0],
                    ),
                );
                res
            }),
        )?;
    let mut irrelevant2 = Vec::new();
    let mut args = hartex_localization_core::types::LocalizationArgs::new();
    args.set("irrelevant", irrelevant);
    let irrelevant = bundle.format_pattern(irrelevant.value(), Some(&args), &mut errors);
    let irrelevant = irrelevant.trim();
}
