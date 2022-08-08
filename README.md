<div align="center">

<h1>GPU N-body WASM Simulation
</br>
<a href="https://www.rust-lang.org/"><img src="https://simpleicons.org/icons/rust.svg" width="50px" height="50px"/></a>
+
<a href="https://www.rust-lang.org/what/wasm"><img src="https://simpleicons.org/icons/webassembly.svg" width="50px" height="50px"/></a>
+
<a href="https://wgpu.rs/"><img src="https://wgpu.rs/logo.min.svg" width="50px" height="50px"/></a>
</h1>

<h2>Click <a href="https://simbleau.github.io/nbody-wasm-sim">here</a> to demo the simulation.</h2>

<img src="demo.gif" width="300px" height="300px"/>

</div>

---

## 📖 Overview
This repository is a 2D N-body simulation of a dynamical system of bodies, under the influence of physical forces such as gravity. The simulation is written completely in Rust with WebGPU and WGSL shading, exported to WebAssembly. We deploy the demo with GitHub Actions.

🔸 Simulations like these are common in astrophysics and are used to understand the evolution of large-scale universal structures.

---

# 🚀 Serve Locally
## Dependencies
- [Rust](https://www.rust-lang.org/)
- [trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [wasm32-unkown-unknown](https://yew.rs/docs/getting-started/introduction#install-webassembly-target) (`rustup target add wasm32-unknown-unknown`)
## Serve
- Run: `trunk serve`
- Preview: [`http://localhost:8080/`](http://localhost:8080/)

![Screenshot](https://user-images.githubusercontent.com/48108917/183275653-a2ee4f9c-a982-482e-8405-bd124d4bbcf5.png)

---

## 📁 Directories

- [__`assets`__](./assets/): directory contains textures and shaders.
- [__`src`__](./src/): directory contains the rust source code.

---

## 🔏 License
This project is dual-licensed under both [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.