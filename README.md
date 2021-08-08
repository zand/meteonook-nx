# Nintendo Switch MeteoNook library.
Modifies and builds the [MeteoNook](https://github.com/Treeki/MeteoNook) source code to generate `include/meteonook.h` and `lib/libmeteonook.a` for `AArch64`.

Documentation and code here (except where specified) is (c) 2021 Chris Butler ("Zand"), and available under the GNU Affero General Public License 3.0 as provided in the LICENSE file.

# Building
In order to build you need have [rustup](https://www.rust-lang.org/tools/install) installed.
```
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
$ git clone --recursive https://github.com/zand/meteonook-nx.git
$ cd meteonook-nx
$ make
```
