pub mod git;
pub mod process;

fn main() {
    process::run("cat", vec!["/etc/passwd"]);

    print!(
        "{:?}",
        git::tags_for_repository("/home/supakeen/dev/src/work/osbuild")
    );
    print!(
        "{:?}",
        git::branches_for_repository("/home/supakeen/dev/src/work/osbuild")
    );
}
