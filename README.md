<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" width="90"/>
</p>

<h1 align="center">The Rust Journey</h1>

<p align="center">
  <sub>A new era of systems programming. Safe. Fast. Fearless.</sub>
</p>

---

## Why Rust

<p align="center">
  <sub>Performance of C. Safety of modern languages. Loved by developers worldwide.</sub>
</p>

<table align="center" border="0" cellpadding="24">
<tr>
<td align="center">
<b>Blazing Fast</b><br/>
<sub>No runtime. No GC.<br/>Pure performance.</sub>
</td>
<td align="center">
<b>Memory Safe</b><br/>
<sub>Guaranteed safety<br/>at compile time.</sub>
</td>
<td align="center">
<b>Fearless Concurrency</b><br/>
<sub>No data races.<br/>Ever.</sub>
</td>
<td align="center">
<b>Reliable</b><br/>
<sub>If it compiles,<br/>it works.</sub>
</td>
</tr>
</table>

---

## The Progress

<p align="center">
  <sub>Every journey begins with a single <code>fn main()</code></sub>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Progress-25%25-F74C00?style=for-the-badge"/>
</p>


<table align="center">
  <tr>
    <th>Area</th>
    <th>Status</th>
  </tr>
  <tr>
    <td>Basics</td>
    <td>Completed</td>
  </tr>
  <tr>
    <td>Ownership</td>
    <td>Completed</td>
  </tr>
  <tr>
    <td>Traits</td>
    <td>Completed</td>
  </tr>
  <tr>
    <td>Async</td>
    <td>Completed</td>
  </tr>
</table>


---

## The Code

<p align="center">
  <sub>Writing software that stands the test of time.</sub>
</p>

```rust
// A minimal, compilable example representing the journey.

struct Developer;

impl Developer {
    fn learn_rust(&self) {
        println!("Learning Rust: safe, fast, fearless.");
    }
}

fn main() {
    let developer = Developer;
    developer.learn_rust();
}
