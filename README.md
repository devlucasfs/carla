<img align="right" src="./assets/icon.png" alt="Carla Logo" width="120px" height="120px">
<br><br>

# Carla

**Carla** is a complete, embeddable, multi-paradigm systems programming language with extensible code generation.

It combines low-level control with deliberate, explicit syntax and a lightweight abstraction model. Carla is designed to be deeply integrable into external systems while remaining adaptable across architectures and execution environments, without losing its identity.

> [!WARNING]
> **Carla is in alpha.** The project is still under development. Everything described in this README will be part of the final result, but not all of it is implemented yet. Expect:
>
> - many `TODO`s throughout the code;
> - debug messages written in **Portuguese** in some places;
> - breaking changes between versions.

## Installation

To install Carla, we recommend downloading it from the official website:

**[carla-lang.vercel.app/downloads](https://carla-lang.vercel.app/downloads)**

## Hello, World!

A minimal Carla program looks like this:

```carla
const io := @import("stdio");

@_start
int main = fn() {
    let language := "Carla";
    io.printf("Hello, World in %!\n", language);
    0
}
```

This small example introduces namespace imports, the entry-point declaration, local variables and standard I/O.

The `@_start` directive is not a decorator or a macro. It is a more convenient representation of:

```carla
..@import("entry");
```

## Syntax & Abstraction

Carla prioritizes **explicitness** and **predictability**:

- Keywords have a single, unambiguous meaning.
- Data grouping is done through **tuples** instead of traditional structs.
- Behavior can be attached to types via `extend`.
- Compile-time evaluation (`constexpr`) allows target-specific code selection.
- Direct control over memory, syscalls and hardware is first-class.

The language sits at a **systems programming** level of abstraction: close enough to the metal for manual memory management and architecture-specific code, yet structured enough to support higher-level organization when desired.

### Multi-paradigm by design

Carla natively supports three paradigms, which can be used separately or combined:

- **Procedural:** direct control over execution and explicit operations.
- **Object-Oriented Programming (OOP):** encapsulation, instance-oriented design and organization of behavior around data.
- **Data-Oriented Design (DOD):** explicit data organization, predictable memory layouts and attention to how data is accessed and processed.

A common and intentional pattern is **OOP on top of DOD**: define data as tuples, then extend them with methods.

```carla
using Person = |int, string|;

extend Person {
    Person make = fn(int age, cstr_z name) {
        return age, name.to_string();
    }

    void show_name = fn(self) {
        if var age, name = self.*; false {};
        stdio.println("%", name);
    }
}

@_start
int main = fn() {
    let p := Person.make(14, "Lucas");
    p.show_name(); -- Lucas
    Person.show_name(&p); -- Lucas
}
```

This keeps data layout explicit and predictable (DOD) while still allowing method-style organization (OOP) when it improves clarity.

## Design Philosophy

Carla follows a set of deliberate design principles that influence its syntax, semantics, memory model and compiler architecture. These are not universal rules for programming languages. Some reflect the developer's own preferences, with the goal of creating a language that is consistent, predictable and enjoyable to work with.

### No runtime global variables

Carla intentionally avoids runtime global variables. Runtime state should belong to an explicit instance or execution context rather than being implicitly shared across the whole program.

Compile-time globals are an exception, since they belong to the compilation process rather than the runtime execution model:

```carla
const kilobyte := 1024;
```

This separates compile-time configuration and computation from runtime state.

### Instance-bound data by default

Data is associated with an instance by default. This encourages explicit ownership and reduces reliance on implicit shared state: instead of data that exists independently of its context, Carla encourages organizing it around the instances and execution contexts that use it.

### Tuples instead of structs

Carla deliberately does not use `struct` or `union` declarations. Tuples are the fundamental mechanism for grouping data:

```carla
using Person = |int, string|;
```

Tuples represent grouped values without a separate structure declaration system. This is a conscious language-design decision, not an attempt to reproduce the traditional C type model.

### Single-purpose keywords

**A keyword has one meaning.** Keywords are not overloaded with unrelated interpretations depending on context, which keeps the language explicit and easier to parse and reason about.

### Syntax as a deliberate choice

Carla's syntax is shaped by its own goals rather than by the expectation that it should imitate an existing language. It prioritizes explicitness and consistency, clear distinctions between language constructs, explicit declarations and the avoidance of unnecessary ambiguity.

## Goals

- A complete programming language with its own ecosystem.
- A straightforward embedding interface for external applications and systems.
- Flexible integration with different architectures, runtimes and execution environments.
- An extensible compilation and code generation architecture.
- A project-oriented configuration and build workflow.
- A lightweight foundation suitable for both general-purpose systems and low-level development.

## Compiler Architecture

### Morgana

[Morgana](https://github.com/devlucasfs/morgana) is Carla's machine-code generation backend. It turns the compiler's target-specific representation into machine-level output and controls how Carla programs are lowered and generated for each architecture.

Carla does not intend to rely on LLVM or QBE as mandatory intermediate compiler infrastructures. Morgana offers a more direct and adaptable relationship with the code generation process. This allows Carla to:

- Maintain control over its internal compilation model.
- Implement target-specific code generation.
- Evolve the backend alongside the language's semantics.
- Extend support for architectures and instruction sets.
- Accept direct community contributions to code generation.

Morgana is designed to make new architectures, instruction sets and target-specific capabilities straightforward to integrate, without requiring the compiler architecture to be redesigned whenever a new target or capability is introduced.

> [!NOTE]
> **Carla itself is not responsible for the number of architectures and operating systems that are supported. That is Morgana's responsibility.** Support for a new architecture or OS is added to Morgana, and Carla uses it through the backend. For the list of supported targets, see the [Morgana repository](https://github.com/devlucasfs/morgana).

## Compile-Time Environment

Carla provides a compile-time environment that allows projects to define and evaluate information before the final program is generated. It is used for target selection, project-specific definitions and other compilation decisions that should not become runtime state.

### Runa

[Runa](https://github.com/devlucasfs/runa) is a Lua-based runtime used by Carla to manage project-level compile-time definitions.

Runa allows the compilation process to inspect, read, modify and generate definitions associated with the current project. This means project-specific language behavior and compilation configuration do not need to be hardcoded into the compiler: a workspace can define its own compile-time values and use them to influence compilation.

Runa definitions are project-scoped, so different workspaces can customize their compilation environment without introducing unnecessary global runtime state.

### Eva

[Eva](https://github.com/carla-repos/eva) is Carla's declarative configuration language. It represents structured project data through namespaces, nested values, lists, references and utility functions.

Carla uses an Eva file named `target.eva` to describe project-level configuration, such as the selected architecture, operating system and other target settings.

Eva is not limited to static key-value pairs. Its runtime interpretation allows values to be resolved through functions and references. For example:

```eva
@paths
home: env("HOME")
```

Here, `env()` resolves an environment variable and provides its value to the configuration. Eva acts as the configuration layer between a workspace and the compilation tooling, so each project can describe its own environment without turning those settings into global language state.

## Build Orchestration

### Samar and the `Sake` file

[Samar](https://github.com/devlucasfs/samar) is a portable build language used by Carla through its project-level `Sake` file.

`Sake` describes the operations that Carla's build workflow should execute. Rather than hardcoding every build operation into Carla, projects define their own dependencies, runners, conditions and target-specific commands. Samar is the orchestration layer that connects the compiler and backend to the tools a target requires. This includes:

- Checking whether required tools are installed.
- Declaring dependencies for specific architectures and operating systems.
- Running assemblers, linkers and other external tools.
- Defining build operations through runners.
- Handling conditional build steps.
- Overriding default SDK build behavior for individual projects.

A dependency can reference a rule that handles a missing tool:

```Sake
rule package {
    die "{@} is not installed."
}
```

Dependencies can be declared for a specific target:

```Sake
architecture x86_64

x86_64::linux {
    dependencies {
        as => package
        ld => package
    }
}
```

A runner defines a build operation:

```Sake
runner binary {
    log "Building Carla project..."
}
```

Runners can execute multiple statements, including shell commands:

```Sake
runner binary {
    `as {file} -o {object}`
    `ld {object} -o {elf}`
}
```

This lets Carla projects use existing toolchains without a dedicated wrapper for every external command. Samar also supports conditional operations and project-level overrides, so a shared build configuration can be maintained while each workspace replaces or extends the behavior it needs.

## Project Structure

A Carla project is organized around a workspace containing its source code, project configuration, compile-time definitions, build description and generated target artifacts:

```text
workspace/
├── target/
├── src/
│   └── main.crl
├── core/
│   └── constexpr.lua
├── target.eva
└── Sake
```

| Path | Purpose |
| --- | --- |
| `target/` | Stores generated artifacts and target-specific output. |
| `src/` | Contains the project's Carla source files. |
| `src/main.crl` | The project's main Carla source file. |
| `core/` | Contains project-level compile-time definitions and supporting files. |
| `core/constexpr.lua` | Lua definitions used by the project's compile-time environment. |
| `target.eva` | Defines the project's configuration through Eva. |
| `Sake` | Defines the project's build operations through Samar. |

## Compile-Time Evaluation

Carla provides compile-time evaluation through `constexpr`. The compiler evaluates conditions before generating the final code for a target, and, unlike runtime conditionals, these conditions can determine whether a particular implementation is included in the generated program at all.

Conditions are written as Lua expressions through the `runa:` evaluator:

```carla
int read = fn(int fd, byte* buf, uint size) {
    constexpr(runa: os == Linux and arch == x86_64)
    {
        return syscall! 0x0(fd, buf, size);
    }
}
```

The condition is evaluated during compilation, and the compiler determines whether the implementation applies to the selected operating system and architecture. This makes it possible to keep target-specific implementations in the same source file without the resulting program containing every implementation.

### Compile-time code selection

Different implementations can be provided for different targets:

```carla
constexpr(runa: arch == x86_64)
{
    -- x86_64-specific implementation
}
```

```carla
constexpr(runa: arch == ARM)
{
    -- ARM-specific implementation
}
```

These are compilation-time decisions, not runtime branching. The selected code is determined by the target environment and the compile-time definitions available to the project.

### Project-defined compile-time definitions

Carla uses Runa to provide project-level language definitions, which can be read or written during compilation. The project's `core/constexpr.lua` file is part of this environment: it lets projects define compile-time values and use them to influence compilation, while keeping project-specific configuration separate from the runtime state of the resulting program.

## Norn: Precompiler Integration

Norn is Carla's default precompiler.

Certain compiler operations, including `@outside` and `@import`, depend on Norn being available at compile time. They are not ordinary runtime features: they require the precompiler to process the relevant source-level directives before code generation.

### `@outside`

The `@outside` directive is restricted to namespace-based imports. It operates on imported namespaces rather than arbitrary source files, keeping the operation tied to Carla's namespace-based import model.

## Import System

Carla provides two distinct import forms, which differ in how the imported code is incorporated into the current source file.

### Namespace-based imports

```carla
const opa := @import("stdlib");
```

This form imports a namespace and binds it to a local name, through which the imported namespace is accessed.

### Direct imports

```carla
..@import("stdlib");
```

This form behaves similarly to a C `#include`. Instead of introducing a namespace alias, the imported source is incorporated directly into the current compilation context.

### Import guards

By default, imports are protected by `pragma once`, except for namespace-based imports. This prevents the same source file from being processed repeatedly through recursive or duplicated imports.

To change this, place the following directive on the first line of the imported file:

```carla
@pragma always
```

With `always`, the file is allowed to be imported repeatedly, which gives explicit control over import behavior when repeated inclusion is required.

## Standard Library and Hardware Abstraction

Carla's standard library is intentionally lightweight, closer to C than to languages with extensive built-in runtime ecosystems. It provides essential functionality without imposing a large runtime dependency on every Carla application, which matters for environments ranging from general-purpose systems to low-level and resource-constrained architectures.

### Hardware Abstraction Layer (HAL)

Carla intends to provide Hardware Abstraction Layer (HAL) implementations for the architectures considered primary within its ecosystem. The HAL is designed to provide a common interface for architecture-specific hardware capabilities, while preserving the ability to access target-specific functionality when necessary.

Carla does not intend to maintain first-party HAL implementations for every possible architecture. For architectures outside the primary support scope, additional implementations will be available through community-maintained libraries.

The standard library and HAL follow the same principle: **keep the core minimal, provide essential implementations, and let the ecosystem expand through independent libraries.**

## Embedding and Integration

Carla is intended to be embedded into a wide range of systems, from general-purpose applications to low-level and resource-constrained environments, without reducing the language to a limited scripting environment. Integration should be flexible enough to accommodate the requirements of the host system, instead of forcing every host to conform to a single rigid execution model.

## The Carla Ecosystem

Carla is built around a collection of components that work together to provide its development and compilation environment.

| Component | Role in Carla |
| --- | --- |
| Carla | The programming language and its compilation model. |
| Morgana | Machine-code generation and target-specific backend. Defines which architectures and OSs are supported. |
| Runa | Lua-based compile-time runtime and project-level definitions. |
| Eva | Declarative project and target configuration. |
| Samar | Build orchestration, dependencies and external tool execution (via the `Sake` file). |
| Norn | Precompiler integration and source-level directive processing. |
