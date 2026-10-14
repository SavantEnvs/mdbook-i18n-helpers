#![no_main]

use libfuzzer_sys::fuzz_target;
use mdbook_i18n_helpers::normalize::normalize;
use polib::catalog::Catalog;
use polib::message::Message;
use polib::metadata::CatalogMetadata;

// Unlike upstream's harness (every source pinned to `foo.md:{idx}`), the `#:` source field is
// fuzzed too, so multi-reference sources such as `a.md b.md:1` reach normalize().
fuzz_target!(|entries: Vec<(&str, &str, &str)>| {
    let mut catalog = Catalog::new(CatalogMetadata::new());
    for (source, msgid, msgstr) in entries {
        let message = Message::build_singular()
            .with_source(String::from(source))
            .with_msgid(String::from(msgid))
            .with_msgstr(String::from(msgstr))
            .done();
        catalog.append_or_update(message);
    }
    let _ = normalize(catalog);
});
