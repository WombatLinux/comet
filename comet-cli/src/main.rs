use clap::{Parser, Subcommand};
use comet::{install_package, remove_package};

#[derive(Parser)]
#[command(author = "afroraydude", version = "1.0.0", about = "The simple package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    #[command(about = "Install a package")]
    Install {
        package: Vec<String>,
        #[arg(short, long)]
        local: bool,
        #[arg(short, long)]
        force: bool,
    },

    #[command(about = "Remove a package")]
    Remove {
        package: Vec<String>,
        #[arg(short, long)]
        force: bool,
    },

    #[command(about = "Update a package")]
    Update {
        package: String
    },

    #[command(about = "Initializes the system for use")]
    Init {
    },

    #[command(about = "Update Cache")]
    UpdateCache {
    },

    #[command(about = "List installed packages")]
    List {
    },

    #[command(about = "List available packages")]
    ListAvailable {
    },

    #[command(about = "Update all packages")]
    UpdateAll {
    },
}

fn main() {
    // ascii art
    let art = r#"
                ----
++++++          -------
 ++++++------    -----------
 +++++--------------------------
  ++++----------------------------
   +++------------------------------
    ++-------------------------------
     +--------------------------------
      --------------------------------
       -------------------------------
        ------------------------------
         -----------------------------
          ----------------------------
           --------------------------
            ------------------------
               -------------------
                 --------------

            Comet Package Manager
"#;

    println!("{}", art);

    let cli = Cli::parse();

    // setup only sets up everything IF it hasn't been setup already
    comet::setup().unwrap();

    // if not sudo or admin, exit
    if !comet::check_permissions() {
        println!("You do not have the proper permissions to use the package manager. Are you root?");
        std::process::exit(1);
    }

    match cli.command {
        Commands::Install { package, local, force } => {
            for p in package.clone() {
                install_package(p, local, force).unwrap_or_else(|err| {
                    panic!("Error while installing packages: {}", err);
                });
            }
        },
        Commands::Remove { package, force } => {
            for p in package.clone() {
                remove_package(p, force).unwrap_or_else(|err| {
                    panic!("Error while removing packages: {}", err);
                });
            }
        },
        Commands::Update { package } => {
            println!("Updating package");
            comet::update_package(package).expect("Failed to update package");
        },
        Commands::Init {} => {
            println!("Initializing system");
            comet::setup();
            println!("Done!")
        },
        Commands::UpdateCache {} => {
            println!("Updating cache");
            comet::update_cache_file().expect("Failed to update cache");
            println!("Done!")
        },
        Commands::List {} => {
            println!("Listing installed packages");
            let packages = comet::list_packages();

            for (name, version) in packages {
                println!("{}: {}", name, version);
            }

            println!("Done!")
        },
        Commands::ListAvailable {} => {
            println!("Listing available packages");
            let packages = comet::list_available_packages();

            for (name, version) in packages {
                println!("{}: {}", name, version);
            }
        },
        Commands::UpdateAll {} => {
            println!("Updating all packages");
            comet::update_all_packages();
            println!("Done!")
        },
    }
}
