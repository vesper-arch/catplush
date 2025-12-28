# catplush
An immediate mode ui framework using a Rust implementation of the C-layout (or Clay) algorithm.

catplush is designed to have a small amount of dependencies and be uncoupled from rendering libraries so it can be as flexible as possible. catplush currently has only around 5 dependencies, excluding any rendering or windowing tools used along with it. 

## The Renderer
The raylib crate was exceptionally confusing to make work with images and ended up giving nonsense errors.

The next logical step is to learn OpenGL from scratch and make my own renderer.

More on this as the story develops.

Big thank you to Speykious for letting me use his `frienderer` crate in the meantime.

## What's with the name?
<img width="313" height="330" alt="image" src="https://github.com/user-attachments/assets/363632f6-01d5-410d-86a0-4ad3217a29e0" />

I mean I might as well.

---
The original Clay code can be found here:

https://github.com/nicbarker/clay
