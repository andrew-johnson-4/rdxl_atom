#![feature(proc_macro_hygiene)]
use rdxl::xhtml;
use rdxl_atom::*;

fn bs(s: String) -> String {
   s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

#[test]
fn format1() {
   assert_eq!(
      bs(xhtml!(<!Feed/>)),
      r#"<?xml version="1.0" encoding="utf-8"?> <feed xmlns="http://www.w3.org/2005/Atom"></feed>"#
   );
}
