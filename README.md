# Kaleidoscopeやってみた with Rust 🦀

## 参考

* 本家
  * https://llvm.org/docs/tutorial/
* LLVM IRについて
  * [こわくないLLVM入門！](https://qiita.com/Anko_9801/items/df4475fecbddd0d91ccc)
* そもそもコンパイラ･インタプリタについて
  * https://keens.github.io/blog/2019/02/16/osusumenokonpairanohontoka/
  * https://www.sigbus.info/compilerbook
  * http://craftinginterpreters.com/
* RustでKaleidoscopeやってみたの既出のもの
  * https://github.com/jauhien/iron-kaleidoscope
  * https://github.com/johannst/llvm-kaleidoscope-rs
  * https://github.com/Ubugeeei/kaleidoscope-inkwell
* 便利クレート
  * LLVMバインディング
    * https://github.com/TheDan64/inkwell
  * Lexer
    * https://github.com/maciejhirsz/logos
  * Parser combinators
    * https://github.com/rust-bakery/nom
* Rustで処理系実装してみた系のクレート
  * JSONパーサー
    * https://github.com/togatoga/monkey-json
      * [RustでJSONパーサーをフルスクラッチで実装する](https://qiita.com/togatoga/items/9d600e20325775f09547)
    * https://github.com/sor4chi/json-parser
      * [RustでJSONの構文解析がしたい](https://zenn.dev/monica/articles/63510afc116ad8)
    * serdeは難しそう...
  * YAMLパーサー
    * https://github.com/chyh1990/yaml-rust
  * JavaScriptパーサー
    * https://github.com/Ubugeeei/Glasper
    * JavaScript Parser In Rust
      * https://boshen.github.io/javascript-parser-in-rust/docs/overview/
      * https://www.cs.cornell.edu/~asampson/blog/flattening.html
      * https://zenn.dev/nissy_dev/scraps/2e224466af001e
  * Rust x LLVM
    * https://github.com/JPNYKW/plat
      * [Rust+LLVMでおもちゃ言語を作ったお話](https://qiita.com/jpnykw/items/287036df59ca000edd5a)
    * https://github.com/yubrot/llrl
      * [Rust + LLVMでプログラミング言語を自作してセルフホスティングした話](https://zenn.dev/yubrot/articles/eaaeeab742b4a1)
  * Cコンパイラ
    * https://github.com/itome/nine-cc
      * [Rust製のC言語コンパイラを作り始めました](https://itome.team/blog/2019/08/started-making-rus-c-compiler/)
    * https://gitlab.com/teten927/c-gengo-parser-by-rust
      * [RustでCコンパイラ作ってみた](https://qiita.com/temmaru/items/6da42ede23aeabe8c263)
    * https://github.com/maekawatoshiki/rucc
    * https://github.com/Ubugeeei/ubcc
  * その他
    * [Writing Interpreters in Rust: a Guide](https://rust-hosted-langs.github.io/book/introduction.html)
    * [Create Your Own Programming Language with Rust](https://createlang.rs/intro.html)
    * [Make A Language](https://lunacookies.github.io/lang/)
    * [Learn Assembly by Writing Entirely Too Many Brainfuck Compilers](https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md)