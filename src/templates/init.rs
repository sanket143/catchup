use anyhow::Result;
use once_cell::sync::OnceCell;
use tera::Tera;

static TERA: OnceCell<Tera> = OnceCell::new();

pub fn init_tera(template_dir: &str) -> Result<()> {
    let tera = Tera::new(template_dir)?;
    TERA.set(tera)
        .expect("Failed to state shared state of Tera");

    Ok(())
}

// If you want a globally accessible, lazily initialized Tera instance (common for read-only access)
pub fn get_tera() -> &'static Tera {
    TERA.get()
        .expect("Tera has not been initialized. Call init_tera() first.")
}
