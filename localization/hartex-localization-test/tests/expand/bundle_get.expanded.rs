use hartex_localization_macros::bundle_get;
fn main() {
    let irrelevant = bundle.get_message("bundle-get-test-message").unwrap();
    let mut irrelevant2 = Vec::new();
    let irrelevant = bundle
        .format_pattern(irrelevant.value().unwrap(), None, &mut errors);
    let irrelevant = irrelevant.trim();
}
