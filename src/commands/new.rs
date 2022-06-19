use crate::cli::*;
use crate::util::{paths};

pub fn cli() -> App {
    subcommand("new")
        .about("Create a new kakasi project at <path>")
        .arg_quiet()
        .arg(Arg::new("path").required(true))
        .arg(opt("registry", "Registry to use").value_name("REGISTRY"))
        .arg_new_opts()
        .after_help("Run `kakasi help new` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let opts = args.new_options(config)?;

    let path = args.get_one::<String>("path").unwrap();
    let project_name = if let Some(name) = args.get_one::<String>("name") {
        name
    } else {
        path
    };

    make_project(&opts, project_name)?;
    config.shell().status(
        "Created",
        format!("`{}` at: {}", project_name, opts.path.display()),
    )?;
    Ok(())
}

fn make_project(opts: &NewOptions, project_name: &str) -> CliResult {
    paths::create_dir_all(&opts.path)?;
    let source_path = &opts.path.join("src");
    paths::create_dir_all(source_path)?;

    let mut deps = String::new();
    deps.push_str(r#"axum = { version = "0.5.8", features = ["ws"] }
futures = "0.3"
tokio = { version = "1", features = ["full"] }
tower = { version = "0.4", features = ["util"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }"#);
    paths::write(
        &opts.path.join("Cargo.toml"),
        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
{}"#, project_name, deps)
    )?;

    paths::write(
        &opts.path.join("chat.html"),
        &paths::get_template_file("chat.html.stub")
    )?;
    paths::write(
        &source_path.join("main.rs"),
        &paths::get_template_file("main.rs.stub")
    )?;
    Ok(())
}