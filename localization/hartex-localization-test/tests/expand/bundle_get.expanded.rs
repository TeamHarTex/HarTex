use hartex_localization_macros::bundle_get;
fn main() {
    let irrelevant = bundle
        .get_message("bundle-get-test-message")
        .ok_or(
            miette::Report::msg({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "key `{0}` not found for locale `{1}`",
                        "bundle-get-test-message", bundle.locales[0],
                    ),
                );
                res
            }),
        )?;
    let mut irrelevant2 = Vec::new();
    let irrelevant = bundle
        .format_pattern(irrelevant.value().unwrap(), None, &mut errors);
    let irrelevant = irrelevant.trim();
    let irrelevant = bundle
        .get_term("bundle-get-test-term")
        .ok_or(
            miette::Report::msg({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "key `{0}` not found for locale `{1}`", "bundle-get-test-term",
                        bundle.locales[0],
                    ),
                );
                res
            }),
        )?;
    let mut irrelevant2 = Vec::new();
    let irrelevant = bundle.format_pattern(irrelevant.value(), None, &mut errors);
    let irrelevant = irrelevant.trim();
}
