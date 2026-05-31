pub fn update() -> anyhow::Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("Behruz-s-organization")
        .repo_name("bat_in_rust")
        .bin_name("show")
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    match status {
        self_update::Status::Updated(v) => println!("✓ {v} ga yangilandi"),
        self_update::Status::UpToDate(v) => println!("✓ Allaqachon eng yangi versiya: {v}"),
    }

    Ok(())
}
