#![allow(clippy::multiple_crate_versions)]

use argh::FromArgs;
use cargo_metadata::MetadataCommand;
use inquire::{Confirm, Select, Text};
use std::env::consts::OS;
use std::fs::{self, read_to_string, remove_file, File};
use std::io::Write;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;
use std::process::{exit, Command};

const DEV_BRANCH: &str = "develop";
const FEATURE_BRANCH_PREFIX: &str = "feature";

const LANG: &str = "en_US";
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

fn check(sentence: &str) -> bool {
    let mut f = File::create("/tmp/commiter").expect("msg");
    writeln!(f, "{sentence}").expect("msg");
    let o = Command::new("hunspell")
        .arg("-d")
        .arg(LANG)
        .arg("-l")
        .arg("/tmp/commiter")
        .output()
        .expect("msg")
        .stdout;
    o.is_empty()
}
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
    writeln!(
        f,
        "\n## Rank\n\n{}\n\n## License\n\n```\n{}\n```",
        get_rank(),
        read_to_string("LICENSE")
            .expect("LINCENSE file not founded")
            .trim()
    )
    .expect("msg");
    remove_file("log").expect("failed to remove log");
    remove_file("rank").expect("failed to remove log");
    assert!(diff());
    prepare_commit();
    assert!(send());
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

fn diff() -> bool {
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
    true
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

fn commit_types_with_help() -> [&'static str; 68] {
    let mut x = HELP;
    x.sort_unstable();
    x
}

fn commit_scope() -> String {
    let mut scope: String;
    loop {
        scope = Text::new("Please enter the commit scope : ")
            .prompt()
            .unwrap();
        if scope.is_empty() {
            continue;
        }
        if scope.len().gt(&20) {
            println!("scope can be superior to 20 character");
            continue;
        }
        if confirm(
            format!("Really use the commit scope : {scope}").as_str(),
            false,
        ) {
            break;
        }
    }
    scope
}

fn commit_types() -> String {
    let mut t: String;
    loop {
        t = Select::new(
            "Please enter the commit type : ",
            commit_types_with_help().to_vec(),
        )
        .prompt()
        .unwrap()
        .to_string();
        if t.is_empty() {
            continue;
        }
        if confirm(format!("Really use the commit type : {t}").as_str(), false) {
            break;
        }
    }
    let x: Vec<&str> = t.split(':').collect();
    (*x.first().unwrap()).to_string()
}

fn commit_summary() -> String {
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
        if confirm(format!("Use the summary : {summary}").as_str(), false) {
            break;
        }
    }
    summary
}

fn commit_description() -> String {
    let mut description: String = String::new();
    loop {
        let d = Text::new("Please enter the commit description : ")
            .prompt()
            .unwrap();
        if d.is_empty() {
            continue;
        }
        description.push_str(format!("{d}\n\n").as_str());
        if confirm("Add a new description line", false) {
            continue;
        }
        break;
    }
    description
}

fn commit_why() -> String {
    let mut why: String = String::new();
    loop {
        let w = Text::new("Please explain the reasoning behind the change : ")
            .prompt()
            .unwrap();
        if w.is_empty() {
            continue;
        }
        if w.len().gt(&50) {
            println!("The reasoning behind the change must be contains less than 50 chararacter");
            continue;
        }
        why.push_str(format!("\n\t* {w}").as_str());
        if confirm("Continue to write the changes : ", false) {
            continue;
        }
        break;
    }
    why
}
fn commit_footer() -> String {
    let mut footer: String = String::new();
    if confirm("Code has breaking changes ?", false) {
        footer.push_str("BREAKING CHANGE: ");
        loop {
            let b = Text::new("Please enter the breaking change description: ")
                .prompt()
                .unwrap();
            if b.is_empty() {
                continue;
            }
            if confirm(
                format!("Use breaking change description : {b}").as_str(),
                false,
            ) {
                footer.push_str(b.as_str());
                break;
            }
        }
    }
    if confirm("Code has resolving issues ?", false) {
        loop {
            footer.push_str("\n\tFixes ");
            let f = Text::new("Please enter the issue number : ")
                .prompt()
                .unwrap();
            if f.is_empty() {
                continue;
            }
            footer.push_str(format!("#{f}\n").as_str());
            if confirm("Code resolving an other issues ?", false) {
                continue;
            }
            break;
        }
    }
    if confirm("Code resolve an issue ?", false) {
        loop {
            footer.push_str("\n\tCloses ");
            let f = Text::new("Please enter the issue number : ")
                .prompt()
                .unwrap();
            if f.is_empty() {
                continue;
            }
            footer.push_str(format!("#{f}\n").as_str());
            if confirm("Code resolve an other issue ?", false) {
                continue;
            }
            break;
        }
    }
    footer
}

fn bad_sentences() {
    println!("The entered text is not correct: it must be written in English and not contain any errors.");
}
fn get_scope() -> String {
    let mut scope: String;
    loop {
        scope = commit_scope();
        if check(scope.as_str()) {
            break;
        }
        bad_sentences();
    }
    scope
}

fn get_summary() -> String {
    let mut summary: String;
    loop {
        summary = commit_summary();
        if check(summary.as_str()) {
            break;
        }
        bad_sentences();
    }
    summary
}

fn get_description() -> String {
    let mut description: String;
    loop {
        description = commit_description();
        if check(description.as_str()) {
            break;
        }
        bad_sentences();
    }
    description
}

fn get_why() -> String {
    let mut why: String;
    loop {
        why = commit_why();
        if check(why.as_str()) {
            break;
        }
        bad_sentences();
    }
    why
}
fn get_footer() -> String {
    let mut footer: String;
    loop {
        footer = commit_footer();
        if check(footer.as_str()) {
            break;
        }
        bad_sentences();
    }
    footer
}
fn prepare_commit() {
    let c = format!(
        "{}({}): {}\n\n{}\n\nThe following changes were made:\n\t{}\n\nThe changes :\n{}\n\nCo-authored-by: {} <{}>",
        commit_types(),
        get_scope(),
        get_summary(),
        get_description(),
        get_why(),
        get_footer(),
        name(),
        email()
    );
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

fn email() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.email")
            .current_dir(".")
            .output()
            .expect("git email not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

fn name() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .current_dir(".")
            .output()
            .expect("username not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

enum Verb {
    Start,
    Finish,
}

fn exist(d: &str) -> bool {
    Path::new(d).is_dir()
}

fn cmd(p: &str, args: &[&str]) -> bool {
    Command::new(p)
        .args(args)
        .current_dir(".")
        .spawn()
        .expect("failed to run command")
        .wait()
        .expect("")
        .success()
}
fn is_git() -> bool {
    exist(".git")
}
fn is_mercurial() -> bool {
    exist(".hg")
}

fn checkout(b: &str) -> bool {
    if is_git() {
        return cmd("git", &["checkout", b]);
    } else if is_mercurial() {
        return cmd("hg", &["update", b]);
    }
    false
}
fn create_branch(b: &str) -> bool {
    if is_git() {
        return cmd("git", &["branch", b]);
    } else if is_mercurial() {
        return cmd("hg", &["branch", b]);
    }
    false
}

fn remove_branch(b: &str) -> bool {
    if is_git() {
        return cmd("git", &["branch", "-d", b]);
    } else if is_mercurial() {
        return cmd("hg", &["branch", "-d", b]);
    }
    false
}

fn init() -> bool {
    create_branch(DEV_BRANCH) && checkout(DEV_BRANCH)
}

fn merge(branch: &str) -> bool {
    if is_git() {
        cmd("git", &["merge", branch])
    } else if is_mercurial() {
        cmd("hg", &["merge", branch])
    } else {
        false
    }
}
fn start_feature(name: &str) -> bool {
    if is_git() {
        cmd(
            "git",
            &[
                "checkout",
                "-b",
                format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str(),
                DEV_BRANCH,
            ],
        )
    } else if is_mercurial() {
        cmd("hg", &[""])
    } else {
        false
    }
}
fn finish_feature(name: &str) -> bool {
    assert!(checkout(DEV_BRANCH));
    assert!(pull(DEV_BRANCH));
    assert!(merge(format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str()));
    assert!(remove_branch(
        format!("{FEATURE_BRANCH_PREFIX}/{name}").as_str()
    ));
    true
}

fn ask(msg: &str) -> String {
    let mut x: String;
    loop {
        x = Text::new(msg).prompt().unwrap().to_string();
        if x.is_empty() {
            continue;
        }
        break;
    }
    x
}

fn feature(name: &str, v: &Verb) -> bool {
    match v {
        Verb::Start => start_feature(name),
        Verb::Finish => finish_feature(name),
    }
}

fn pull(branch: &str) -> bool {
    if is_git() {
        cmd("git", &["pull", branch])
    } else if is_mercurial() {
        cmd("hg", &["pull", branch])
    } else {
        false
    }
}

fn send() -> bool {
    if is_git() {
        cmd("git ", &["push", "--all"]) && cmd("git ", &["push", "--tags"])
    } else if is_mercurial() {
        cmd("hg", &["push"])
    } else {
        false
    }
}
fn status() -> bool {
    if is_git() {
        cmd("git", &["status"])
    } else if is_mercurial() {
        cmd("hg", &["status"])
    } else {
        false
    }
}

fn flow(zuu: bool) -> i32 {
    loop {
        if zuu {
            let o: &str = Select::new(
                "What you want do :  ",
                vec![
                    "Init a repository",
                    "Start a new feature",
                    "Finish a feature",
                    "Commit",
                    "Generate change log",
                    "Send modidifications",
                    "Show status",
                    "Show diff",
                    "Send",
                    "Quit",
                ],
            )
            .prompt()
            .unwrap();
            let x: bool = if o.starts_with("Init") {
                init()
            } else if o.starts_with("Start") && o.contains("feature") {
                feature(&ask("Feature name : "), &Verb::Start)
            } else if o.starts_with("Finish") && o.contains("feature") {
                feature(&ask("Feature name : "), &Verb::Finish)
            } else if o.starts_with("Commit") {
                assert!(diff());
                prepare_commit();
                if exist("Cargo.toml") {
                    publish();
                }
                true
            } else if o.starts_with("Generate") && o.contains("change log") {
                create_changelog();
                true
            } else if o.starts_with("Send") {
                send()
            } else if o.starts_with("Show") && o.contains("status") {
                status()
            } else if o.starts_with("Show") && o.contains("diff") {
                diff()
            } else if o.starts_with("Quit") {
                break;
            } else {
                false
            };
            assert!(x);
        }
    }
    println!("Bye...");
    0
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
    if zuu() {
        if exist(".git") || exist(".hg") {
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
            }
            exit(flow(true));
        } else {
            println!("Repository not initialized !");
            exit(1);
        }
    }
    println!("Repository contains errors !");
    exit(1);
}
