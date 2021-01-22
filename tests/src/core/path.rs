// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_core::{env, path};

/// Test the `path` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::append()", {
    let mut p = String::from("/");

    path::append(&mut p, "some/path");
    assert_eq!(p, "/some/path");

    path::append(&mut p, "/abs/olute");
    assert_eq!(p, "/abs/olute");

    p.replace_range(.., "rel/ative");
    path::append(&mut p, "some/path");
    assert_eq!(p, "rel/ative/some/path");

    p.replace_range(.., "/a");
    path::append(&mut p, "");
    assert_eq!(p, "/a/");
  });

  test!(cx, "::is_absolute()", {
    assert!(path::is_absolute("/a"));
    assert!(!path::is_absolute("a"));
  });

  test!(cx, "::join()", {
    assert_eq!(path::join("/", "some/path"), "/some/path");
    assert_eq!(path::join("/x/y", "/abs/olute"), "/abs/olute");
    assert_eq!(path::join("rel/ative", "some/path"), "rel/ative/some/path");
    assert_eq!(path::join("/a", ""), "/a/");
  });

  test!(cx, "::last()", {
    assert_eq!(path::last("/a/b"), Some("b"));
    assert_eq!(path::last("/a"), Some("a"));
    assert_eq!(path::last("/"), None);
    assert_eq!(path::last("a"), Some("a"));
    assert_eq!(path::last(""), None);
  });

  test!(cx, "::normalize()", {
    let mut p: String = "/a/../b".into();

    path::normalize(&mut p);
    assert_eq!(p, "/b");

    p.replace_range(.., "/a/./b");
    path::normalize(&mut p);
    assert_eq!(p, "/a/b");

    p.replace_range(.., "/a/b/");
    path::normalize(&mut p);
    assert_eq!(p, "/a/b");

    p.replace_range(.., ".././a/.");
    path::normalize(&mut p);
    assert_eq!(p, "a");
  });

  test!(cx, "::normalized()", {
    assert_eq!(path::normalized("/a/../b"), "/b");
    assert_eq!(path::normalized("/a/./b"), "/a/b");
    assert_eq!(path::normalized("/a/b/"), "/a/b");
    assert_eq!(path::normalized(".././a/."), "a");

    let cow: Cow<str> = "/a/b".into();

    assert!(std::ptr::eq(path::normalized(&cow).as_ref(), cow.as_ref()));
  });

  test!(cx, "::parent()", {
    assert_eq!(path::parent("/a/b"), Some("/a"));
    assert_eq!(path::parent("/a/b"), Some("/a"));
    assert_eq!(path::parent("/a"), Some("/"));
    assert_eq!(path::parent("/"), None);
    assert_eq!(path::parent("a/b/"), Some("a"));
    assert_eq!(path::parent("a"), Some(""));
    assert_eq!(path::parent(""), None);
  });

  test!(cx, "::pop()", {
    let mut p: String = "/a/b/".into();

    assert_eq!(path::pop(&mut p), Some("b".into()));
    assert_eq!(p, "/a");
    assert_eq!(path::pop(&mut p), Some("a".into()));
    assert_eq!(p, "/");
    assert_eq!(path::pop(&mut p), None);
    assert_eq!(p, "/");

    p.replace_range(.., "a/b");

    assert_eq!(path::pop(&mut p), Some("b".into()));
    assert_eq!(p, "a");
    assert_eq!(path::pop(&mut p), Some("a".into()));
    assert_eq!(p, "");
    assert_eq!(path::pop(&mut p), None);
    assert_eq!(p, "");
  });

  test!(cx, "::resolve()", {
    let wd = env::working_path().unwrap();
    let mut p: String = "/a/../b".into();

    path::resolve(&mut p).unwrap();
    assert_eq!(p, "/b");

    p.replace_range(.., "/a/./b/");
    path::resolve(&mut p).unwrap();
    assert_eq!(p, "/a/b");

    p.replace_range(.., "a/../b/.");
    path::resolve(&mut p).unwrap();
    assert_eq!(p, format!("{}/b", wd));
  });

  test!(cx, "::resolved()", {
    let wd = env::working_path().unwrap();

    assert_eq!(path::resolved("/a/../b").unwrap(), "/b");
    assert_eq!(path::resolved("/a/./b/").unwrap(), "/a/b");
    assert_eq!(path::resolved("a/../b/.").unwrap(), format!("{}/b", wd));
  });
}
