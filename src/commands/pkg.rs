use colorize::AnsiColor;

#[allow(unused_imports)]
use crate::{command, command_fn, terminal::Command, versions::FIRST_VERSION_EVER};
use crate::{
    package_manager, utils::compiler_outputs::CompilerOutputs,
};

command_fn!(pkg(args) {
    install:  String = "install",
    update:   String = "update",
    provider: String = "provider" || "github".into(),
    @bool global     = "global",
    @bool help       = "help",
} => {

    if help {
        println!(
            "
    {title} {desc}

    {usage}
        carla pkg {install} <package> [{provider} <provider>] [{global}]
        carla pkg {update}  <package> [{provider} <provider>] [{global}]

    {flags}
        {i}, {install} <package>     Package you want to install
        {u}, {update}  <package>     Package you want to update
        {p}, {provider} <provider>   Package provider {default}
        {g}, {global}                Install in the libs path {recommended}
        {h}, {help}                  Show this message

    {examples}
        {c1}
        carla pkg --install author@lib --global

        {c2}
        carla pkg -i author@lib -g

        {c3}
        carla pkg --update author@lib --global

    {notes}
        {dot} Use {install} or {update}, never both in the same command.
        {dot} If no provider is given, {github} is used.
    ",
            title = "carla pkg".cyan().bold(),
            desc = "- manage your packages into your Carla project".grey(),
            usage = "USAGE".yellow().bold(),
            flags = "FLAGS".yellow().bold(),
            examples = "EXAMPLES".yellow().bold(),
            notes = "NOTES".yellow().bold(),
            install = "--install".green(),
            update = "--update".green(),
            provider = "--provider".green(),
            global = "--global".green(),
            help = "--help".green(),
            i = "-i".green(),
            u = "-u".green(),
            p = "-p".green(),
            g = "-g".green(),
            h = "-h".green(),
            default = "(default: github)".grey(),
            recommended = "(recommended)".grey(),
            c1 = "# Install a package from GitHub into the libs path".grey(),
            c2 = "# Same thing, using the short flags".grey(),
            c3 = "# Update an installed package".grey(),
            dot = "•".cyan(),
            github = "github".bold(),
        );

        return;
    }

    match (install, provider, global, update) {
        (Some(pkg), p, g, None)      => package_manager::install::service(pkg, p, g),
        (None,      p, g, Some(pkg)) => package_manager::update::service(pkg, p, g),

        _ => CompilerOutputs::fatal("That isn't a valid command pattern. Try to use `carla pkg --help` to listen all possible commands and their examples"),
    }
});

pub const PKG: Command = command! {
    name: "pkg",
    description: "Manage your packages into your Carla project",
    since: FIRST_VERSION_EVER,
    example: "",
    flags: {
        Evaluated("provider", "p") => "Package provider. Eg. Github",
        Evaluated("install", "i") => "Package you want to install",
        Evaluated("update", "u") => "Package you want to update",

        Boolean("global", "g") => "Use it to install the package in the libs path (recommended)",
        Boolean("help", "h") => "See all pkg flags and how to use them",
    },
    function: pkg,
};
