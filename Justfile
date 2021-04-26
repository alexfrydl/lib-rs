# Copyright © 2020 Alexandra Frydl
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Builds all features.
build:
  cargo build --workspace --all-features

# Displays information about available recipes.
help:
  just --list

# Adds the MPL license header to all source files.
license:
  #!/bin/fish

  set -l notice \
  "// Copyright © 2021 Alexandra Frydl
  //
  // This Source Code Form is subject to the terms of the Mozilla Public
  // License, v. 2.0. If a copy of the MPL was not distributed with this
  // file, You can obtain one at http://mozilla.org/MPL/2.0/.
  "

  for file in **/*.rs
    if grep -q "http://mozilla.org/MPL/2.0/" $file
      continue
    end

    echo $notice > $file.tmp
    and cat $file >> $file.tmp
    and mv $file{.tmp,}
  end

test:
  cargo test --workspace --all-features
