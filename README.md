<img align="right" src="./assets/icon.png" alt="Carla Logo" width="120px" height="120px">
<br><br>

# Carla

**Carla** is a complete, embeddable, multi-paradigm systems programming language with extensible code generation.

It combines low-level control with deliberate, explicit syntax and a lightweight abstraction model. Carla is designed to be deeply integrable into external systems while remaining adaptable across architectures and execution environments — without losing its identity.

### Syntax & Abstraction

Carla prioritizes **explicitness** and **predictability**:

- Keywords have a single, unambiguous meaning
- Data grouping is done through **tuples** instead of traditional structs
- Behavior can be attached to types via `extend`
- Compile-time evaluation (`constexpr`) allows target-specific code selection
- Direct control over memory, syscalls and hardware is first-class

The language sits at a **systems programming** level of abstraction: close enough to the metal for manual memory management and architecture-specific code, yet structured enough to support higher-level organization when desired.

### Multi-paradigm by design

Carla natively supports:

- **Procedural** programming
- **Object-Oriented Programming (OOP)**
- **Data-Oriented Design (DOD)**

These can be used separately or combined. A common and intentional pattern is to use **OOP on top of DOD** — defining data as tuples and then extending them with methods:

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
    p.show_name();          -- Lucas
    Person.show_name(&p);
}
```

This approach keeps data layout explicit and predictable (DOD) while still allowing method-style organization (OOP) when it improves clarity.

---

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

This small example introduces Carla's namespace imports, entry-point declaration, local variables, and standard I/O.

## What is Carla's objective?

Carla is designed to provide:

- A complete programming language with its own ecosystem.
- A straightforward embedding interface for external applications and systems.
- Flexible integration with different architectures, runtimes, and execution environments.
- An extensible compilation and code generation architecture.
- A project-oriented configuration and build workflow.
- A lightweight foundation suitable for both general-purpose systems and low-level development.

Carla aims to be more than a language that can be embedded. It is intended to be a language that can be **deeply integrated, extended, and adapted without losing its identity**.

## Design Philosophy

Carla follows a set of deliberate design principles that influence its syntax, semantics, memory model, and compiler architecture.

These principles are not universal rules for programming languages. Some reflect the developer's own preferences, with the goal of creating a language that is consistent, predictable, and enjoyable to work with.

### Multi-paradigm programming

Carla accommodates different programming paradigms without forcing the developer into a single programming style.

The language is designed to work with principles commonly associated with:

- **Object-Oriented Programming (OOP):** Encapsulation, instance-oriented design, and organization of behavior around data.
- **Data-Oriented Design (DOD):** Explicit data organization, predictable memory layouts, and consideration of how data is accessed and processed.
- **Procedural programming:** Direct control over execution and explicit operations.

These paradigms are not treated as mutually exclusive. Carla aims to provide the flexibility to combine them according to the requirements of the application.

### No runtime global variables

Carla intentionally avoids runtime global variables.

Runtime state should belong to an explicit instance or execution context rather than being implicitly shared across the entire program.

Compile-time globals are an exception, since they belong to the compilation process rather than the runtime execution model.

```carla
const kilobyte := 1024;
```

This distinction separates compile-time configuration and computation from runtime state.

### Instance-bound data by default

Data in Carla is designed to be associated with an instance by default.

This encourages explicit ownership and reduces reliance on implicit shared state. Instead of treating data as something that exists independently of its context, Carla encourages developers to organize it around the instances and execution contexts that use it.

### Tuples instead of structs

Carla deliberately does not use traditional `struct` or `union` declarations.

Instead, tuples are the fundamental mechanism for grouping data.

```carla
using Person = |int, string|;
```

Tuples provide a direct way to represent grouped values without introducing a separate structure declaration system.

This is a conscious language-design decision, rather than an attempt to reproduce the traditional C type model.

### Single-purpose keywords

Carla follows a strict principle:

**A keyword should have one meaning.**

Keywords should not be overloaded with unrelated semantic interpretations depending on context.

This keeps the language explicit and makes its syntax easier to understand, parse, and reason about.

### Syntax as a deliberate design choice

Carla's syntax is shaped by its own design goals rather than by the expectation that it should imitate an existing language.

It prioritizes:

- Explicitness and consistency.
- Clear distinctions between language constructs.
- Avoidance of unnecessary syntactic ambiguity.
- Semantically distinct keywords.
- Explicit declarations.

## Compiler Architecture

### Morgana

[Morgana](https://github.com/devlucasfs/morgana) is Carla's machine-code generation backend.

Carla does not intend to rely on LLVM or QBE as mandatory intermediate compiler infrastructures. Instead, Morgana provides a more direct and adaptable relationship with the code generation process.

For Carla, Morgana is responsible for turning the compiler's target-specific representation into machine-level output. It provides the foundation for controlling how Carla programs are lowered and generated for different architectures.

This architecture allows Carla to:

- Maintain control over its internal compilation model.
- Implement target-specific code generation.
- Evolve the backend alongside the language's semantics.
- Extend support for architectures and instruction sets.
- Accept direct community contributions to code generation.

Morgana is not merely an implementation detail. It is a deliberate architectural choice that allows Carla to maintain control over its compilation process while keeping the backend open to extension.

**See more here:** [Morgana repository](https://github.com/devlucasfs/morgana)

### Extensible code generation

Morgana is designed to make new architectures, instruction sets, and target-specific capabilities straightforward to integrate.

The objective is to keep code generation adaptable to Carla's needs without requiring the entire compiler architecture to be redesigned whenever a new target or capability is introduced.

## Compile-Time Environment

Carla provides a compile-time environment that allows projects to define and evaluate information before the final program is generated.

This environment is used for target selection, project-specific definitions, and other compilation decisions that should not become runtime state.

### Runa

[Runa](https://github.com/devlucasfs/runa) is a Lua-based runtime used by Carla to manage project-level compile-time definitions.

Runa allows Carla's compilation process to inspect, read, modify, and generate definitions associated with the current project.

For Carla, this means that project-specific language behavior and compilation configuration do not need to be hardcoded into the compiler itself. A workspace can define its own compile-time values and use them to influence compilation.

Runa definitions are project-scoped, allowing different workspaces to customize their compilation environment without introducing unnecessary global runtime state.

Runa provides the programmable side of Carla's compile-time environment, while Carla determines how those definitions affect the compilation process.

**See more here:** [Runa repository](https://github.com/devlucasfs/runa)

### Eva

[Eva](https://github.com/carla-repos/eva) is Carla's declarative configuration language.

It represents structured project data through namespaces, nested values, lists, references, and utility functions.

Carla uses an Eva file named `target.eva` to describe project-level configuration, including information such as the selected architecture, operating system, and other target settings.

Eva is not limited to static key-value pairs. Its runtime interpretation allows configuration values to be resolved through functions and references.

For example:

```eva
@paths
home: env("HOME")
```

Here, `env()` resolves an environment variable and provides its value to the configuration.

For Carla, Eva acts as the configuration layer between a workspace and the compilation tooling. It allows each project to describe its own environment without requiring those settings to become global language state.

**See more here:** [Eva repository](https://github.com/carla-repos/eva)

## Build Orchestration

### Sake

[Samar](https://github.com/devlucasfs/samar) is a portable build language used by Carla through its project-level `Sake` file.

Sake describes the operations that Carla's build workflow should execute. Rather than hardcoding every build operation into Carla, projects can define their own dependencies, runners, conditions, and target-specific commands.

For Carla, Samar provides the orchestration layer that connects the compiler and backend to the tools required by a target.

This includes operations such as:

- Checking whether required tools are installed.
- Declaring dependencies for specific architectures and operating systems.
- Running assemblers, linkers, and other external tools.
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

This allows Carla projects to use existing toolchains without requiring a dedicated wrapper for every external command.

Samar also supports conditional operations and project-level overrides. This makes it possible to maintain a shared build configuration while allowing each workspace to replace or extend the behavior it requires.

**See more here:** [Samar repository](https://github.com/devlucasfs/samar)

## Project Structure

A Carla project is organized around a workspace containing its source code, project configuration, compile-time definitions, build description, and generated target artifacts.

A typical workspace follows this structure:

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

Each component has a specific responsibility:

| Path | Purpose |
| --- | --- |
| `target/` | Stores generated artifacts and target-specific output. |
| `src/` | Contains the project's Carla source files. |
| `src/main.crl` | The project's main Carla source file. |
| `core/` | Contains project-level compile-time definitions and supporting files. |
| `core/constexpr.lua` | Lua definitions used by the project's compile-time environment. |
| `target.eva` | Defines the project's configuration through Eva. |
| `Sake` | Defines the project's build operations through Samar. |

This organization separates source code, configuration, build orchestration, and generated output.

## Compile-Time Evaluation

Carla provides compile-time evaluation through `constexpr`.

Compile-time expressions allow the compiler to evaluate conditions before generating the final code for a target.

Unlike runtime conditionals, compile-time conditions can determine whether a particular implementation is included in the generated program.

Carla uses Lua expressions through the `runa:` evaluator to describe certain compile-time conditions.

For example:

```carla
int read = fn(int fd, byte* buf, uint size) {
    constexpr(runa: os == Linux and arch == x86_64)
    {
        return syscall! 0x0(fd, buf, size);
    }
}
```

In this example, the condition is evaluated during compilation. The compiler can determine whether the implementation applies to the selected operating system and architecture.

This makes it possible to maintain target-specific implementations within the same source file without requiring the resulting program to contain every implementation.

### Compile-time code selection

Compile-time conditions can select different implementations for different targets:

```carla
constexpr(runa: arch == x86_64)
{
    -- x86_64-specific implementation
}
```

A different implementation can be provided for another architecture:

```carla
constexpr(runa: arch == ARM)
{
    -- ARM-specific implementation
}
```

These conditions describe compilation-time decisions rather than runtime branching.

The selected code is determined by the target environment and the compile-time definitions available to the project.

### Project-defined compile-time definitions

Carla uses Runa to provide project-level language definitions.

These definitions are stored within the project's compile-time environment and can be read or written during compilation.

The project's `core/constexpr.lua` file is part of this environment. It allows projects to define compile-time values and use them to influence compilation behavior, while keeping project-specific configuration separate from the runtime state of the resulting program.

## Norn: Precompiler Integration

Norn is Carla's default precompiler.

Certain compiler operations, including `@outside` and `@import`, depend on Norn being available during compile time.

These operations are not intended to function as ordinary runtime features. They require the precompiler to process the relevant source-level directives.

Norn is integrated into Carla's compilation workflow so that source processing and import-related operations can be handled before the final code generation stage.

### `@outside`

The `@outside` directive is restricted to namespace-based imports.

It is intended to operate on imported namespaces rather than arbitrary source files, keeping the operation tied to Carla's namespace-based import model.

## Import System

Carla provides two distinct import forms.

They differ in how the imported code is incorporated into the current source file.

### Namespace-based imports

```carla
const opa := @import("stdlib");
```

This form imports a namespace and binds it to a local name.

The imported namespace can then be accessed through the declared alias.

Namespace-based imports are distinct from direct source inclusion.

### Direct imports

```carla
..@import("stdlib");
```

This form behaves similarly to a C `#include`.

Instead of introducing a namespace alias, the imported source is incorporated directly into the current compilation context.

This distinction allows Carla to support both namespace-oriented code organization and direct source inclusion.

### Import guards

By default, imports are protected by `pragma once`, except for namespace-based imports.

This prevents the same source file from being processed repeatedly through recursive or duplicated imports.

The behavior can be explicitly changed by placing the following directive on the first line of the imported file:

```carla
@pragma always
```

With `always`, the file is allowed to be imported repeatedly.

This provides an explicit way to control import behavior when repeated inclusion is required.

## Standard Library and Hardware Abstraction

Carla's standard library is intentionally lightweight, following an approach closer to C than to languages with extensive built-in runtime ecosystems.

The standard library aims to provide essential functionality without imposing a large runtime dependency on every Carla application.

This is particularly important for Carla's goal of supporting environments ranging from general-purpose systems to low-level and resource-constrained architectures.

### Hardware Abstraction Layer (HAL)

Carla intends to provide Hardware Abstraction Layer (HAL) implementations for its major supported architectures.

These implementations cover the architectures considered primary within the Carla ecosystem.

The HAL is designed to provide a common interface for interacting with architecture-specific hardware capabilities while preserving the ability to access target-specific functionality when necessary.

Carla does not intend to maintain first-party HAL implementations for every possible architecture.

For architectures outside the primary support scope, developers will be able to obtain additional implementations through community-maintained libraries.

This keeps Carla's core ecosystem lightweight while allowing its hardware support to expand beyond the architectures maintained directly by the project.

The standard library and HAL follow the same fundamental principle:

**Keep the core minimal, provide essential implementations, and allow the ecosystem to expand through independent libraries.**

## Embedding and Integration

Carla is intended to be embedded into a wide range of systems.

Its embedding model aims to make integration straightforward without reducing the language to a limited scripting environment.

The language should be adaptable to different hosts, architectures, and runtime configurations.

This includes environments ranging from general-purpose applications to low-level and resource-constrained systems.

The goal is to make Carla's integration capabilities flexible enough to accommodate the requirements of the host system, rather than requiring every host to conform to a single rigid execution model.

## A Carla Program in Context

The following example demonstrates how Carla is currently used to define a program entry point, import a namespace, and execute a simple operation through the standard I/O interface.

```carla
const io := @import("stdio");

@_start
int main = fn() {
    let language := "Carla";
    io.printf("Hello, World in %!\n", language);
    0
}
```

The program imports the `stdio` namespace, declares its entry point, and prints a message.

The `@_start` directive is not a decorator or a macro. It is a more convenient representation of:

```carla
..@import("entry");
```

This provides a clearer way to express the program's entry-point setup while preserving Carla's direct import model.

This example illustrates Carla's explicit import syntax, local declarations, and direct control over program initialization.

## The Carla Ecosystem

Carla is built around a collection of components that work together to provide its development and compilation environment.

| Component | Role in Carla |
| --- | --- |
| Carla | The programming language and its compilation model. |
| Morgana | Machine-code generation and target-specific backend infrastructure. |
| Runa | Lua-based compile-time runtime and project-level definitions. |
| Eva | Declarative project and target configuration. |
| Sake | Build orchestration, dependencies, and external tool execution. |
| Norn | Precompiler integration and source-level directive processing. |

Together, these components allow Carla to maintain control over its language, compilation process, and project environment while remaining extensible.

Carla's objective is to provide a foundation that can scale across different execution environments — from general-purpose systems to low-level and embedded targets — without forcing every project to depend on the same rigid runtime or toolchain.

**Carla is designed to keep the language in control of its own ecosystem, while allowing that ecosystem to grow through extensible tooling and community contributions.**
