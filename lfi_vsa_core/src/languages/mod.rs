// ============================================================
// LFI Language Intelligence — Universal Polyglot Code Engine
//
// The LFI agent must code in ALL major languages and paradigms:
//   Systems: Rust, C, C++, Assembly, Go
//   JVM: Java, Kotlin, Scala
//   .NET: C#, VB.NET, ASP.NET, F#
//   Apple: Swift, Objective-C
//   Web: JavaScript, TypeScript, PHP, HTML/CSS
//   Frontend Frameworks: React, Angular, Vue, Svelte
//   Functional: Elixir, Erlang, Haskell, OCaml, Clojure
//   Scripting: Python, Ruby, Perl, Lua, Bash/Shell
//   Data: SQL, R, Julia, MATLAB
//   Low-Level: WebAssembly (WASM), Verilog, VHDL, SystemVerilog
//   Mobile: Kotlin (Android), Swift (iOS), Dart (Flutter)
//   Infrastructure: Terraform, Ansible, Docker, K8s YAML
//
// Strategy: Each language's grammar and idioms are encoded as
// hypervector relationships in the unified 10,000-bit VSA space.
// Language constructs that are semantically equivalent across
// languages (e.g., "for loop" in C vs Python vs Rust) will have
// HIGH cosine similarity, enabling cross-language reasoning.
//
// This is fundamentally different from LLM token prediction.
// The LFI agent KNOWS the mathematical structure of code,
// not the statistical probability of the next token.
// ============================================================

pub mod constructs;
pub mod registry;
pub mod self_improve;

pub use constructs::{UniversalConstruct, Paradigm, PlatformTarget};
pub use registry::{LanguageId, LanguageRegistry};
pub use self_improve::SelfImproveEngine;
