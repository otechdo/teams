#![allow(clippy::multiple_crate_versions)]
use inquire::{Select, Text};
use std::{path::Path, process::Command};

const DEV_BRANCH: &str = "develop";
const FEATURE_BRANCH_PREFIX: &str = "feature";

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
                format!("feature/{name}").as_str(),
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
        cmd("git", &["push", "--all"]) && cmd("git ", &["push", "--tags"])
    } else if is_mercurial() {
        cmd("hg", &["push"])
    } else {
        false
    }
}
fn main() {
    loop {
        if cmd("zuu", &[]) {
            let o: &str = Select::new(
                "What you want do :  ",
                vec![
                    "Init a repository",
                    "Start a new feature",
                    "Finish a feature",
                    "Commit",
                    "Generate change log",
                    "Send modidifications",
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
                cmd("commiter", &[""])
            } else if o.starts_with("Generate") && o.contains("change log") {
                cmd("commiter", &["--generate-change-log"])
            } else if o.starts_with("Send") {
                send()
            } else {
                break;
            };
            assert!(x);
        }
    }
    println!("Bye...");
}
