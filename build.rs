const LIB_PATH: &str = "src/lib.rs";

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("include");
    let mut b = autocxx_build::Builder::new(LIB_PATH, &[&path])
        .build()?;
    b.flag_if_supported("-std=c++14")
        .compile("autocxx-bug");
    println!("cargo:rerun-if-changed={}", LIB_PATH);
    Ok(())
}
