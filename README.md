# Kaleidoscopeやってみた with Rust 🦀

## 参考

* 本家
  * https://llvm.org/docs/tutorial/
* LLVM IRについて
  * [こわくないLLVM入門！](https://qiita.com/Anko_9801/items/df4475fecbddd0d91ccc)
* そもそもコンパイラについて
  * https://keens.github.io/blog/2019/02/16/osusumenokonpairanohontoka/
  * https://www.sigbus.info/compilerbook
* RustでKaleidoscopeやってみたの既出のもの
  * https://github.com/jauhien/iron-kaleidoscope
  * https://github.com/johannst/llvm-kaleidoscope-rs
* Rustで処理系実装してみた系のクレート
  * JSONパーサー
    * https://github.com/togatoga/monkey-json
      * [RustでJSONパーサーをフルスクラッチで実装する](https://qiita.com/togatoga/items/9d600e20325775f09547)
    * https://github.com/sor4chi/json-parser
      * [RustでJSONの構文解析がしたい](https://zenn.dev/monica/articles/63510afc116ad8)
    * serdeは難しそう...
  * YAMLパーサー
    * https://github.com/chyh1990/yaml-rust
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