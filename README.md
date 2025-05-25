<div align="center">
<h1>Ray Tracer Challenge</h1> 

<picture>
<img alt="Example of simple sphere with light and shading." src="/renders/light-and-shading.png" height="250">
</picture>

</div>

#### Introduction  
This educational project follows ["The Ray Tracer Challenge"](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/) by Jamis Buck, learning 3D computer graphics fundamentals, implementing core mathematical primitives and mastering Rust.  

**Learning Focus**   
- **Rust**: Ownership, type safety, const generics, error handling.   
- **Graphics**: Linear algebra, transformations, coordinate systems, color theory.   
- **TDD**: Comprehensive test coverage for mathematical correctness.   

### Roadmap  
- [x] **3D tuples:** representing points and vectors.  
- [x] **Colors:** operations, creating canvas and saving to PPM file (Portable Pixmap format).  
- [x] **Matrices:** creating, multiplying, transposing and inverting.  
- [x] **Matrix transformations:** translation, scaling, rotation, shearing.  
- [x] **Ray-sphere intersections:** rays, tracking intersections, identifying hits, transforming rays and spheres.  
- [x] **Light and shading:** surface normals, reflecting vectors, the Phong Reflection Model.  
- [ ] **Scene and camera**  
- [ ] **Shadows**  
- [ ] **Planes**
- [ ] **Patterns**  
- [ ] **Reflection and Refraction**  
- [ ] **Cubes**  
- [ ] **Cylinders**  
- [ ] **Groups, triangles, CSG&** and more  

#### Quick Start  

**Prerequisites:**  
- Rust 2024 edition or later  
- Cargo (comes with Rust)  

```bash
git clone https://github.com/ipersids/rust-ray-tracer-challenge.git ray_tracer
cd ray_tracer

# Build first, then run the optimized binary directly
cargo build --release
./target/release/06-shading

# Or use release mode for specific binary
cargo run --release --bin 06-shading

# Or use debug mode
cargo run --bin 06-shading

# To run all tests use command
cargo test
```

________  
<div align="center">

<p>Developed by <a href="https://www.linkedin.com/in/iuliia-persidskaia/">Julia Persidskaia</a>.</p>

</div>
