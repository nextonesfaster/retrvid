use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use arboard::Clipboard;
use clap::{AppSettings, ArgGroup, Parser};

const HELP_TEMPLATE: &str = r"{before-help}{bin} {version}
{author}

{about}

{usage-heading}
    {usage}

{all-args}{after-help}";

const ABOUT: &str = "retrvid (rid) lets you store and retrieve ids with a lookup name.

The ids are stored in a `toml` file in your default data directory. You can change \
the location of this file by setting the `RETRVID_DATA` environment variable. \
See the project home page for more information.

Project home page: https://github.com/nextonesfaster/retrvid";

type Data = HashMap<String, String>;
type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = ABOUT)]
#[clap(settings(&[
    AppSettings::ArgRequiredElseHelp,
    AppSettings::DeriveDisplayOrder
]))]
#[clap(help_template(HELP_TEMPLATE))]
#[clap(group(
    ArgGroup::new("command")
        .multiple(false)
        .args(&["name", "list", "add", "remove"]),
))]
struct App {
    /// The name of the id to get.
    name: Option<String>,
    /// Print the id on the console.
    #[clap(short, long)]
    print: bool,
    /// Do not copy the id to the system clipboard.
    ///
    /// The id is copied to the system clipboard if this flag is not present.
    #[clap(short = 'C', long)]
    no_copy: bool,
    /// List all stored id names.
    #[clap(short, long)]
    list: bool,
    /// Add provided name and id to the database.
    #[clap(short, long, value_names = &["NAME", "ID"])]
    add: Option<Vec<String>>,
    /// Remove provided id (name) from the database.
    #[clap(short, long, value_name = "NAME")]
    remove: Option<String>,
}

fn data_path() -> Result<PathBuf, &'static str> {
    if let Ok(path) = env::var("RETRVID_DATA") {
        Ok(PathBuf::from(&path))
    } else {
        dirs_next::data_dir()
            .map(|h| h.join("retrvid/ids.toml"))
            .ok_or("unable to retrieve home directory path")
    }
}

fn get_data<P: AsRef<Path>>(path: P) -> Result<Data, Error> {
    let path = path.as_ref();
    let data_str = if !&path.exists() {
        fs::create_dir_all(&path.parent().ok_or("invalid data path")?)?;
        fs::File::create(&path)?;
        String::new()
    } else {
        fs::read_to_string(&path)?
    };

    toml::from_str(&data_str).map_err(|e| e.into())
}

fn list_names(data: HashMap<String, String>) {
    if data.is_empty() {
        println!("no ids found");
    } else {
        println!(
            "{}",
            itertools::Itertools::intersperse(data.into_keys(), String::from("\n"))
                .collect::<String>()
        );
    }
}

fn add_id<P: AsRef<Path>>(
    app: App,
    mut data: HashMap<String, String>,
    path: P,
) -> Result<(), Error> {
    let mut add_data = app.add.ok_or("name and id to add not specified")?;
    if add_data.len() != 2 {
        return Err("name and/or id to add not specified".into());
    }
    let name = add_data.remove(0);
    data.insert(name.clone(), add_data.remove(0));

    // the path is guaranteed to exist at this point
    fs::write(path, toml::to_string_pretty(&data)?)?;
    println!("added id with name {}", name);

    Ok(())
}

fn remove_id<P: AsRef<Path>>(
    app: App,
    mut data: HashMap<String, String>,
    path: P,
) -> Result<(), Error> {
    let id = app.remove.ok_or("id to remove not specified")?;
    if data.remove(&id).is_none() {
        println!("no id with name {} found", id);
        return Ok(());
    }

    // the path is guaranteed to exist at this point
    fs::write(path, toml::to_string_pretty(&data)?)?;
    println!("removed id with name {}", id);

    Ok(())
}

fn get_id(app: App, mut data: HashMap<String, String>) -> Result<(), Error> {
    let name = app.name.ok_or("no id name specified")?;
    let id = data
        .remove(&name)
        .ok_or(format!("id `{}` not found", &name))?;

    if app.print {
        println!("{}", &id);
    }

    if !app.no_copy {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(id)?;
        println!("copied id to the clipboard");
    }

    Ok(())
}

fn run() -> Result<(), Error> {
    let app = App::parse();

    let data_path = data_path()?;
    let data = get_data(&data_path)?;

    if app.list {
        list_names(data);
    } else if app.add.is_some() {
        add_id(app, data, &data_path)?;
    } else if app.remove.is_some() {
        remove_id(app, data, &data_path)?;
    } else {
        get_id(app, data)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
    }
}
