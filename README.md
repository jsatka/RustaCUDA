<h1 align="center">RustaCUDA</h1>

<div align="center">High-level Interface to <a href="https://developer.nvidia.com/cuda-zone">NVIDIA® CUDA™ Driver API</a> in Rust</div>

<div align="center">
    <a href="https://such-research.github.io/RustaCUDA/rustacuda/index.html">API Documentation (master branch)</a>
</div>
<br/>

This is a fork of [RustaCUDA](https://github.com/bheisler/RustaCUDA) crate by Brook Heisler (@bheisler). Some compromises are, and will be made in favor of flexibility and performance over safety in this particular fork.

RustaCUDA helps you bring GPU-acceleration to your projects by providing a flexible, easy-to-use interface to the CUDA GPU computing toolkit. RustaCUDA makes it easy to manage GPU memory, transfer data to and from the GPU, and load and launch compute kernels written in any language.

## Table of Contents
- [Table of Contents](#table-of-contents)
  - [Goals](#goals)
  - [Roadmap](#roadmap)
  - [Quickstart](#quickstart)
  - [Contributing](#contributing)
  - [Maintenance](#maintenance)
  - [License](#license)
  - [Requirements](#requirements)
  - [Related Projects](#related-projects)

### Goals

 The primary design goals are:

 - __High-Level__: Using RustaCUDA should feel familiar and intuitive for Rust programmers.
 - __Easy-to-Use__: RustaCUDA should be well-documented and well-designed enough to help novice GPU programmers get started, while not limiting more experienced folks too much.
 - __Fast__: RustaCUDA should aim to be as fast as possible, where it doesn't conflict with the above goals.
 - __Safe__: Many aspects of GPU-accelerated computing are difficult to reconcile with Rust's safety guarantees, but RustaCUDA should provide as safe interface that is reasonably practical without compromising too much on performance.

RustaCUDA is intended to provide a programmer-friendly library for working with the host-side CUDA
Driver API. It is not intended to assist in compiling Rust code to CUDA kernels (though see
[rust-ptx-builder](https://github.com/denzp/rust-ptx-builder) for that) or to provide device-side
utilities to be used within the kernels themselves.

RustaCUDA is deliberately agnostic about how the kernels work or how they were compiled. This makes
it possible to use kernels compiled with `nvcc`. The `resources/` directory contains an example
script for compiling the example kernels written in CUDA C with `nvcc` for specified CUDA device
architecture target.

### Roadmap

RustaCUDA currently supports a minimum viable subset of the CUDA API (essentially, the minimum
necessary to manage memory and launch basic kernels). This does not include:

- Access to CUDA 1/2/3D arrays and texture memory
- Multi-GPU support
- Runtime linking
- CUDA Graphs
- And more!

These additional features will be developed later, as time permits and as necessary. If you need a
feature that is not yet supported, consider submitting a pull request!

### Quickstart

Before using RustaCUDA, you must install the CUDA development libraries for your system. Version
8.0 or newer is required. You must also have a CUDA-capable GPU installed with the appropriate
drivers.

#### Ubuntu

```shell
export LIBRARY_PATH=/usr/local/cuda/lib64
```

#### Windows

```shell
export CUDA_LIBRARY_PATH="C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v11.3\lib\x64"
```

#### Using in a crate

Now, to start building a basic CUDA crate. Add the following to your `Cargo.toml`:

```yaml
[dependencies]
rustacuda = "0.1"
rustacuda_core = "0.1"
```

And this to your crate root:

```rust
#[macro_use]
extern crate rustacuda;

#[macro_use]
extern crate rustacuda_core;
```

Next, download the `resources/add.ptx` file from the RustaCUDA repository and place it in
the resources directory for your application.

The `examples/` directory contains sample code that helps getting started. 
To execute the most simple example, (adding two numbers on the CUDA device),
place this code to your `main.rs` file.

```rust
#[macro_use]
extern crate rustacuda;

use rustacuda::prelude::*;
use rustacuda::memory::DeviceBox;
use std::error::Error;
use std::ffi::CString;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the CUDA API
    rustacuda::init(CudaFlags::empty())?;
    
    // Get the first device
    let device = Device::get_device(0)?;

    // Create a context associated to this device
    let context = Context::create_and_push(
        ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    // Load the module containing the function we want to call
    let module_data = CString::new(include_str!("../resources/add.ptx"))?;
    let module = Module::load_from_string(&module_data)?;

    // Create a stream to submit work to
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    // Number of values in device buffers
    const N: u32 = 1;

    // Allocate space on the device and copy numbers to it.
    let mut x = DeviceBox::new(&10.0f32)?;
    let mut y = DeviceBox::new(&20.0f32)?;
    let mut result = DeviceBox::new(&0.0f32)?;

    // Launching kernels is unsafe since Rust can't enforce safety.
    // Think of kernel launches as a foreign-function call. In this case,
    // it is - this kernel is written in CUDA C, compiled to PTX, and then
    // loaded and executed on the CUDA device.
    unsafe {
        // Launch the `add` global kernel function with one block containing one thread
        // without shared memory on the given stream.
        launch!(module.add<<<1, 1, 0, stream>>>(
            x.as_device_ptr(),
            y.as_device_ptr(),
            result.as_device_ptr(),
            N
        ))?;
    }

    // Kernel launches are always asynchronous, so wait for the stream
    // to complete queued kernel launches, as well as any other tasks
    // submitted to the stream.
    stream.synchronize()?;

    // Copy the result back to the host
    let mut result_host = 0.0f32;
    result.copy_to(&mut result_host)?;
    
    println!("Sum is {}", result_host);

    Ok(())
}
```

If everything is working, you should be able to run `cargo run` and see the output:

```text
Sum is 30.0
```

### Contributing

Thanks for your interest! Contributions are welcome.

Issues, feature requests, questions and bug reports should be reported via the issue tracker above.
In particular, because RustaCUDA aims to be well-documented, please report anything you find
confusing or incorrect in the documentation.

Code or documentation improvements in the form of pull requests are also welcome. Please file or
comment on an issue to allow for discussion before doing a lot of work, though.

For more details, see the [CONTRIBUTING.md file](https://github.com/such-research/rustaCUDA/blob/master/CONTRIBUTING.md).

### Maintenance

RustaCUDA is currently maintained by Joonas Satka (@joonassatka).

### License

RustaCUDA is dual-licensed under the Apache 2.0 license and the MIT license.

### Requirements

RustaCUDA requires at least CUDA version 8 to be installed.

### Related Projects

- [accel](https://github.com/rust-accel/accel) is a full CUDA computing framework. Thanks to accel for creating and maintaining the `cuda-sys` FFI wrapper library.
- [rust-ptx-builder](https://github.com/denzp/rust-ptx-builder) is a `build.rs` helper library which makes it easy to compile Rust crates into CUDA kernels.
