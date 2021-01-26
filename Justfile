# Copyright © 2020 Alexandra Frydl
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# Displays information about available recipes.
help:
  just --list

# Builds all features.
build:
  cargo build --workspace --all-features

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

publish package:
  @cd {{package}} && cargo publish --all-features

test component:
  #!/bin/fish

  switch {{component}}
  case core
    cd af-core && cargo run --all-features
  case postgres
    cd af-postgres && cargo build --all-features && cd .. && docker-compose run test-postgres
  case slack
    cd af-slack && cargo run --all-features
  case sentry
    cd af-sentry && cargo run --all-features
  case '*'
    echo 'Unrecgonized component `'{{component}}'`.' >&2
    exit 1
  end
