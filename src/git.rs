use crate::process;

use std::str;

pub fn tags_for_repository(repository: &'static str) -> Vec<String> {
    let process = process::run(
        "git",
        vec![
            "-c",
            "versionsort.suffix=-",
            "ls-remote",
            "--tags",
            "--sort",
            "v:refname",
            repository,
        ],
    );

    process::split_lines(process.stdout)
        .iter()
        .map(|line| {
            line.split('\t')
                .last()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string()
        })
        .filter(|tag| tag != "")
        .filter(|tag| !tag.contains("{}"))
        .collect()
}

pub fn branches_for_repository(repository: &'static str) -> Vec<String> {
    let process = process::run(
        "git",
        vec![
            "-c",
            "versionsort.suffix=-",
            "ls-remote",
            "--heads",
            "--sort=v:refname",
            repository,
        ],
    );

    process::split_lines(process.stdout)
        .iter()
        .map(|line| {
            line.split('\t')
                .last()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string()
        })
        .filter(|head| head != "")
        .collect()
}

pub fn clone_at_ref(repository: &'static str, reference: &'static str, target: &'static str) {
    process::run(
        "git",
        vec!["clone", "-b", reference, "--depth", "1", repository, target],
    );
}
