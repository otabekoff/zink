---
layout: home

hero:
  name: Zink
  text: Say what you mean.
  tagline: A fast, readable scripting language built in Rust. Designed for humans.
  image:
    light: /logo-light.svg
    dark: /logo-dark.svg
    alt: Zink Logo
  actions:
    - theme: brand
      text: ⚡ Get Started
      link: /guide/getting-started
    - theme: alt
      text: Try in Playground →
      link: /playground/

features:
  - icon: ⚡
    title: Rust-Powered
    details: Tree-walk interpreter written in Rust. Compiles in seconds, runs instantly. Zero dependencies.
  - icon: 📖
    title: Readable by Design
    details: No semicolons. No type annotations. String interpolation built in. Code that reads like prose.
  - icon: 🎓
    title: Beginner-First
    details: 'say "Hello!" is a complete program. Learn to code without fighting the language.'
  - icon: 🧩
    title: 28 Builtins
    details: Math, strings, arrays, higher-order functions — everything you need, nothing you don't.
  - icon: 🌐
    title: Runs in Browser
    details: Full playground with WASM. Write, run, and share Zink code — no install needed.
  - icon: 🔓
    title: Open Source
    details: MIT licensed. Use it, fork it, learn from it, contribute to it. Free forever.
---

<div class="motto-banner">
  <q class="motto">Say what you mean. Mean what you say.</q>
</div>

<div class="stats-bar">
  <div class="stat-item">
    <div class="stat-number">28</div>
    <div class="stat-label">Builtins</div>
  </div>
  <div class="stat-item">
    <div class="stat-number">5</div>
    <div class="stat-label">Types</div>
  </div>
  <div class="stat-item">
    <div class="stat-number">0</div>
    <div class="stat-label">Semicolons</div>
  </div>
  <div class="stat-item">
    <div class="stat-number">∞</div>
    <div class="stat-label">Readability</div>
  </div>
</div>


<div class="code-showcase">
  <h2>See it in action</h2>
  <p class="subtitle">Clean syntax that reads like English, runs like Rust</p>

```zink
# Variables and functions — no ceremony
let name = "World"
fn greet(who) {
  say "Hello, {who}!"
}
greet(name)

# Arrays and higher-order functions
let nums = [1, 2, 3, 4, 5]
let even = filter(nums, fn(x) { return x % 2 == 0 })
let doubled = map(even, fn(x) { return x * 2 })
say "Result: {doubled}"

# Counted loops — reads like English
loop 3 times {
  say "Zink! ⚡"
}
```

</div>

<div class="zen-section">
  <h2>The Zen of Zink</h2>
  <ol class="zen-list">
    <li>Readable is better than clever.</li>
    <li>Obvious is better than implicit.</li>
    <li>One way is better than many ways.</li>
    <li>Simple things should be simple.</li>
    <li>Complex things should be possible.</li>
    <li>Errors should explain, not blame.</li>
    <li>Batteries included, not batteries required.</li>
    <li>A program is a story — it should read like one.</li>
    <li>If you need a comment to explain the syntax, the syntax is wrong.</li>
    <li>Say what you mean. Mean what you say.</li>
  </ol>
</div>

<div class="home-cta">
  <h2>Ready to try Zink?</h2>
  <p>Start writing readable code in minutes.</p>
  <div class="cta-buttons">
    <a href="/zink/guide/getting-started" class="cta-btn primary">Get Started</a>
    <a href="/zink/playground/" class="cta-btn secondary">Open Playground</a>
    <a href="https://github.com/otabekoff/zink" class="cta-btn secondary" target="_blank">GitHub →</a>
  </div>
</div>

<style scoped>
.motto-banner {
  text-align: center;
  margin: 2.5rem 0 1.5rem;
}
.motto-banner .motto {
  font-size: 2.4rem;
  font-weight: 500;
  color: var(--vp-c-text-2);
  opacity: 0.8;
  font-style: oblique;
  margin: 2rem 0;
}

.stats-bar {
  display: flex;
  justify-content: center;
  gap: 2.5rem;
  flex-wrap: wrap;
  padding: 1.2rem 1rem;
  margin: 0 auto 0.5rem;
  max-width: 720px;
  background: var(--vp-c-bg-elv);
  border: 1px solid var(--vp-c-border);
  border-radius: 10px;
}
.stat-item {
  text-align: center;
  min-width: 80px;
}
.stat-number {
  font-size: 2rem;
  font-weight: 700;
  color: var(--vp-c-brand-1);
  font-family: var(--vp-font-family-mono);
}
.stat-label {
  font-size: 0.75rem;
  color: var(--vp-c-text-2);
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-weight: 600;
  margin-top: 2px;
  opacity: 0.7;
}

.code-showcase {
  max-width: 720px;
  margin: 2.5rem auto 3rem !important;
  padding: 0 1.5rem !important;
  background: var(--vp-c-bg-elv);
  border: 1px solid var(--vp-c-border);
  border-radius: 10px;
}
.code-showcase h2 {
  text-align: center;
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--vp-c-brand-1);
  margin-top: 1.5rem !important;
  margin-bottom: 0.2rem;
}
.code-showcase .subtitle {
  text-align: center;
  color: var(--vp-c-text-2);
  font-size: 0.95rem;
  margin-bottom: 1.2rem;
  opacity: 0.7;
}

.zen-section {
  max-width: 720px;
  margin: 2.5rem auto;
  padding: 2rem 1.5rem;
  background: var(--vp-c-bg-elv);
  border: 1px solid var(--vp-c-border);
  border-radius: 10px;
}
.zen-section h2 {
  text-align: center;
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--vp-c-brand-1);
  margin-bottom: 1.2rem;
}
.zen-list {
  list-style: decimal inside;
  padding: 0;
  margin: 0;
}
.zen-list li {
  padding: 8px 0 8px 0.5em;
  font-size: 1rem;
  color: var(--vp-c-text-1);
  border-bottom: 1px solid var(--vp-c-border);
}
.zen-list li:last-child {
  border-bottom: none;
}

.home-cta {
  text-align: center;
  padding: 2.5rem 1.5rem 3rem;
  background: var(--vp-c-bg-elv);
  border: 1px solid var(--vp-c-border);
  border-radius: 10px;
  max-width: 720px;
  margin: 2.5rem auto 0;
}
.home-cta h2 {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--vp-c-brand-1);
  margin-bottom: 0.5rem;
}
.home-cta p {
  color: var(--vp-c-text-2);
  font-size: 0.95rem;
  margin-bottom: 1.2rem;
  opacity: 0.8;
}
.cta-buttons {
  display: flex;
  justify-content: center;
  gap: 12px;
  flex-wrap: wrap;
}
.cta-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 10px 24px;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.95rem;
  transition: background 0.2s, color 0.2s, border 0.2s;
  text-decoration: none !important;
  border: 1px solid var(--vp-c-border);
  background: var(--vp-c-bg);
  color: var(--vp-c-brand-1) !important;
}
.cta-btn.primary {
  background: var(--vp-c-brand-1);
  color: #fff !important;
  border: 1px solid var(--vp-c-brand-1);
}
.cta-btn.primary:hover {
  background: var(--vp-c-brand-2);
  border-color: var(--vp-c-brand-2);
}
.cta-btn.secondary:hover {
  background: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-2) !important;
  border-color: var(--vp-c-brand-2);
}
.vp-doc h2 {
    margin: 0;
    border-top: unset;
    padding-top: 0;
}
</style>
