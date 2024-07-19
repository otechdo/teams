#![allow(clippy::multiple_crate_versions)]

use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use inquire::{Confirm, Select, Text};
use std::env::consts::OS;
use std::fs::{self, read_to_string, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;
use std::process::Command;

pub const TYPES: [&str; 68] = [
    "Star",
    "Comet",
    "Nebula",
    "Pulsar",
    "Quasar",
    "Asteroid Belt",
    "Solar Flare",
    "Dwarf Planet",
    "Terraform",
    "Black Hole",
    "Wormhole",
    "Big Bang",
    "Launch",
    "Lightspeed",
    "Mission Control",
    "Spacewalk",
    "Moon Landing",
    "First Contact",
    "Interstellar Communication",
    "Solar Eclipse",
    "Supernova",
    "Meteor Shower",
    "Solar Wind",
    "Lunar Eclipse",
    "Cosmic Dawn",
    "Solar Storm",
    "Lunar Transit",
    "Perihelion",
    "Aphelion",
    "White Dwarf",
    "Red Giant",
    "Neutron Star",
    "Binary Star",
    "Brown Dwarf",
    "Quark Star",
    "Rogue Planet",
    "Stellar Nursery",
    "Planetary Nebula",
    "Globular Cluster",
    "Void",
    "Gravity",
    "Dark Matter",
    "Time Dilation",
    "Spacetime",
    "Gravitational Lensing",
    "Cosmic String",
    "Quantum Fluctuation",
    "Hawking Radiation",
    "Quantum Entanglement",
    "Gravitational Redshift",
    "Space Probe",
    "Station",
    "Rocket Launch",
    "Spacewalk",
    "Space Elevator",
    "Warp Drive",
    "Dyson Sphere",
    "Generation Ship",
    "Lagrange Point",
    "Orbital Maneuver",
    "Mission Control",
    "Moon Landing",
    "Interstellar Travel",
    "Rover",
    "Singularity",
    "Relativity",
    "Expansion",
    "Big Crunch",
];

const HELP: [&str; 68] = [
    "Star: New feature or enhancement",
    "Comet: Bug fix or error resolution",
    "Nebula: Code refactoring",
    "Pulsar: Performance improvement",
    "Quasar: Documentation or clarity improvement",
    "Asteroid Belt: Code cleanup and maintenance",
    "Solar Flare: Testing-related changes",
    "Dwarf Planet: Minor updates or fixes",
    "Terraform: Infrastructure changes",
    "Black Hole: Removing large chunks of code or features",
    "Wormhole: Merging branches or connecting code parts",
    "Big Bang: Initial commit or major feature start",
    "Launch: Deploying to production or releasing a version",
    "Lightspeed: Significant performance improvements",
    "Mission Control: Project management changes",
    "Spacewalk: Urgent hotfixes",
    "Moon Landing: Major milestone or goal completion",
    "First Contact: Initial integrations with external systems",
    "Interstellar Communication: Improving documentation or communication",
    "Solar Eclipse: Temporarily masking functionality",
    "Supernova: Major, transformative change",
    "Meteor Shower: Series of small changes or fixes",
    "Solar Wind: Refactoring code structure",
    "Lunar Eclipse: Temporarily disabling a feature",
    "Cosmic Dawn: Initial implementation of a feature",
    "Solar Storm: Rapid, impactful changes",
    "Lunar Transit: Minor, temporary change",
    "Perihelion: Brings the project closer to its goals or objectives",
    "Aphelion: Immediate goals, but is necessary for long-term progress",
    "White Dwarf: Improving code comments or documentation",
    "Red Giant: Expanding a feature or functionality",
    "Neutron Star: Optimizing code for performance",
    "Binary Star: Merging features or components",
    "Brown Dwarf: Undeveloped feature with potential",
    "Quark Star: Experimental or speculative change",
    "Rogue Planet: Independent change",
    "Stellar Nursery: Creation of new components",
    "Planetary Nebula: Removal or deprecation of a component",
    "Globular Cluster: Collection of related changes",
    "Void: Removal of a module, component, or feature",
    "Gravity: Resolving merge conflicts or dependencies",
    "Dark Matter: Fixing unknown or mysterious bugs",
    "Time Dilation: Improving code performance",
    "Spacetime: Changes to date, time, or scheduling",
    "Gravitational Lensing: Altering data or information flow",
    "Cosmic String: Connecting code parts",
    "Quantum Fluctuation: Small, random change",
    "Hawking Radiation: Removing technical debt",
    "Quantum Entanglement: Establishing close relationships between code parts",
    "Gravitational Redshift: Slowing down or reducing code performance",
    "Space Probe: Testing new features or technologies",
    "Station: Creating or improving environments",
    "Rocket Launch: Deploying to production",
    "Spacewalk: Urgent production hotfixes",
    "Space Elevator: Making codebase more accessible",
    "Warp Drive: Significant speed improvement",
    "Dyson Sphere: Comprehensive optimization of a specific area",
    "Generation Ship: Long-term project for a self -sustaining system",
    "Lagrange Point: Stabilizing or balancing code parts",
    "Orbital Maneuver: Changing project direction",
    "Mission Control: Represents project management-related changes",
    "Moon Landing: Celebrates the completion of major milestones",
    "Interstellar Travel: Migration to a new architecture or language",
    "Rover: Exploration of new technologies or approaches",
    "Singularity: Resolution of a complex or hard-to-reproduce issue",
    "Relativity: Changes related to time, dates, or timestamps",
    "Expansion: Scaling up the system or increasing capacity",
    "Big Crunch: Reduction of codebase size or removal of features",
];

fn get_last_tag() -> String {
    let tag: String = String::from_utf8(
        Command::new("git")
            .arg("describe")
            .arg("--tags")
            .arg("--abbrev=0")
            .current_dir(".")
            .output()
            .unwrap()
            .stdout,
    )
    .expect("Faile to find a TAG");
    let data: Vec<&str> = tag.split('\n').collect();
    (*data.first().expect("msg")).to_string()
}
fn get_log() -> String {
    let log = File::create("log").expect("failed to create log");
    let d = format!("{}..HEAD", get_last_tag());
    assert!(Command::new("git")
        .arg("log")
        .arg("--format=fuller")
        .arg(d.as_str())
        .stdout(log)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    read_to_string("log").expect("failed to parse file")
}

fn get_rank() -> String {
    let log = File::create("rank").expect("failed to create rank");
    assert!(Command::new("git")
        .arg("rank")
        .stdout(log)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    read_to_string("rank")
        .expect("failed to parse file")
        .trim()
        .to_string()
}

fn create_changelog() {
    if Path::new("./logs").is_dir().eq(&false) {
        fs::create_dir_all("./logs").expect("msg");
    }
    let filename = format!(
        "./logs{MAIN_SEPARATOR_STR}{}-{}-changes.md",
        project(),
        version()
    );
    let logs = get_log();
    let lines = logs.lines();
    let mut f = File::create(filename.as_str()).expect("failed to create file");
    writeln!(f, "# {}\n\n> {}\n", project(), description()).expect("msg");
    writeln!(f, "- Authors\n").expect("msg");
    for author in authors() {
        writeln!(f, "  - {author}").expect("msg");
    }
    for t in commit_types_with_help() {
        let ttt: Vec<&str> = t.split(':').collect();
        let title: String = (*ttt.last().unwrap()).to_string();
        writeln!(f, "\n-{title}").expect("msg");
        for line in lines.clone() {
            let current = (*ttt.first().unwrap()).to_string();
            if line.contains(current.as_str()) {
                let lll = line.split('\n');
                for l in lll {
                    let c = l.replace(ttt.first().unwrap(), "");
                    let cc: Vec<&str> = c.split(':').collect();
                    let ccc: Vec<&str> = cc.last().unwrap().split('\n').collect();
                    let message = ccc.join("\n");
                    writeln!(f, "\n  -{message}").expect("msg");
                }
            }
        }
    }
    writeln!(f, "\n## Rank\n\n{}", get_rank()).expect("msg");
    remove_file("log").expect("failed to remove log");
    remove_file("rank").expect("failed to remove log");
    diff();
    prepare_commit();
    send();
    if Path::new("Cargo.toml").exists() {
        publish();
    }
}

fn create_patch() {
    if Path::new("./patches").exists().eq(&false) {
        assert!(fs::create_dir_all("./patches").is_ok());
    }
    assert!(Command::new("git")
        .arg("format-patch")
        .arg("-1")
        .current_dir("./patches")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}
fn create_tag() {
    let m: String = Text::new("Enter the tag message : ").prompt().unwrap();

    let v: String = Text::new("Enter the tag version : ")
        .with_default(version().as_str())
        .prompt()
        .unwrap();

    if m.is_empty() || v.is_empty() {
        create_tag();
    }
    assert!(Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(v.as_str())
        .arg("-m")
        .arg(m.as_str())
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    assert!(Command::new("cargo")
        .arg("publish")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}

fn send_tag() {
    assert!(Command::new("git")
        .arg("push")
        .arg("--all")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    assert!(Command::new("git")
        .arg("push")
        .arg("--tags")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}
fn commit(m: &str) {
    assert!(Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(m)
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    create_patch();
    if confirm("Create new tag", false) {
        create_tag();
        send_tag();
    }
}

fn diff() {
    loop {
        clear();
        assert!(Command::new("git")
            .arg("diff")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .arg("status")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        if confirm("Add modifications ?", false) {
            assert!(Command::new("git")
                .arg("add")
                .arg(".")
                .current_dir(".")
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
            break;
        }
    }
}

fn fmt() {
    assert!(Command::new("cargo")
        .arg("fmt")
        .current_dir(".")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
    clear();
}

fn zuu() -> bool {
    clear();
    if Path::new("Cargo.toml").exists() {
        fmt();
        if Command::new("zuu")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success()
        {
            clear();
            return true;
        }
        return false;
    }
    clear();
    true
}

fn version() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package = metadata.packages.first().unwrap();
    package.version.to_string()
}

fn project() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.name.to_string()
}
///
/// # Panics
///
fn description() -> String {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package
        .description
        .as_ref()
        .expect("missing description")
        .to_string()
}

fn authors() -> Vec<String> {
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let package: &cargo_metadata::Package = metadata.packages.first().unwrap();
    package.authors.clone()
}
fn clear() {
    if OS.eq("windows") {
        assert!(Command::new("cls")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    } else {
        assert!(Command::new("clear")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn commit_types() -> [&'static str; 68] {
    let mut x = TYPES;
    x.sort_unstable();
    x
}

fn commit_types_with_help() -> [&'static str; 68] {
    let mut x = HELP;
    x.sort_unstable();
    x
}

fn prepare_commit() {
    let help = Confirm::new("Display commit type with help message ?")
        .with_default(false)
        .prompt()
        .unwrap()
        .eq(&true);
    let t = if help {
        Select::new("Select a commit type : ", commit_types_with_help().to_vec())
            .prompt()
            .unwrap()
    } else {
        Select::new("Select a commit type : ", commit_types().to_vec())
            .prompt()
            .unwrap()
    };

    let s = Text::new("Please enter the commit scope : ")
        .prompt()
        .unwrap();
    if s.is_empty() {
        prepare_commit();
    }
    let mut summary: String;
    loop {
        summary = Text::new("Please enter the commit summary : ")
            .prompt()
            .unwrap();
        if summary.is_empty() {
            continue;
        }
        if summary.len().gt(&50) {
            println!("Summary must be contains less than 50 chararacter");
            continue;
        }
        break;
    }

    let mut description: String;
    loop {
        description = Text::new("Please enter the commit description : ")
            .prompt()
            .unwrap();
        if description.is_empty() {
            continue;
        }
        break;
    }

    let mut why: String = String::new();
    loop {
        let w = Text::new("Please explain the reasoning behind the change : ")
            .prompt()
            .unwrap();
        if w.is_empty() {
            continue;
        }
        if w.len().gt(&50) {
            println!("the reasoning behind the change must be contains less than 50 chararacter");
            continue;
        }
        why.push_str(format!("\n* {w}").as_str());
        if confirm("Continue to write the changes : ", true) {
            continue;
        }
        break;
    }
    let mut footer: String = String::new();
    loop {
        let f = Text::new("Please enter the commit footer : ")
            .prompt()
            .unwrap();
        if f.is_empty() {
            continue;
        }
        footer.push_str(format!("\n{f}").as_str());
        if confirm("Continue to write the footer : ", true) {
            continue;
        }
        break;
    }
    let c: String = if help {
        let x: Vec<&str> = t.split(':').collect();
        format!(
            "{}({s}): {}\n\n{description}\n\nThe following changes were made:\n{why}\n{footer}\n",
            x.first().unwrap(),
            summary.to_lowercase().replace('.', "")
        )
    } else {
        format!(
            "{t}({s}): {}\n\n{description}\n\nThe following changes were made:\n{why}\n{footer}\n",
            summary.to_lowercase().replace('.', "")
        )
    };
    commit(c.as_str());
}

fn confirm(msg: &str, default: bool) -> bool {
    Confirm::new(msg)
        .with_default(default)
        .prompt()
        .unwrap()
        .eq(&true)
}
fn publish() {
    if confirm("Publish ?", false) {
        assert!(Command::new("cargo")
            .arg("publish")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn send() {
    if confirm("Send to remotes ?", true) {
        assert!(Command::new("git")
            .arg("push")
            .arg("--all")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        assert!(Command::new("git")
            .arg("push")
            .arg("--tags")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

#[derive(FromArgs)]
#[argh(description = "manage the repository commit message and changelog")]
struct Commiter {
    #[argh(switch, description = "generate change log")]
    generate_change_log: Option<bool>,
    #[argh(switch, description = "show commiter rank")]
    rank: Option<bool>,
}
fn main() {
    let commiter: Commiter = argh::from_env();

    if commiter.generate_change_log.is_some() {
        create_changelog();
    } else if commiter.rank.is_some() {
        assert!(Command::new("git")
            .arg("rank")
            .current_dir(".")
            .spawn()
            .expect("missing alias")
            .wait()
            .unwrap()
            .success());
    } else if Path::new(".git").exists() && zuu() {
        diff();
        prepare_commit();
        send();
        if Path::new("Cargo.toml").exists() {
            publish();
        }
    }
}
