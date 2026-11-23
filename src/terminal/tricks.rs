pub const TRICK_MSG: &str =
    "Tricks are a simple feature of the Carla Compiler. \
    They are flags that are not associated with any specific command. \
    We call them tricks because they usually modify the compiler's \
    behavior globally, giving users greater control over their \
    project workflow.";

pub const TRICKS: &[(&str, &str, &str)] = &[
    (
        "--platform:<ns>",
        "Flags such as --main use @target fields with their own names \
        as default values. The --platform trick changes the @target \
        field to @<ns>, where <ns> represents the selected platform \
        namespace. This allows you to define default values for flags \
        across different platforms.",
        "carla upload --platform:esp32",
    ),
];
