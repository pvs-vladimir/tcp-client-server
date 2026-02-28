use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, value_name = "COMMAND", required = true)]
    pub command: String,

    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,

    #[arg(long, value_name = "ID", value_parser = clap::value_parser!(u32))]
    pub id: Option<u32>,
}

impl Cli {
    pub fn validate(&self) -> Result<(), String> {
        match self.command.as_str() {
            "add" => {
                if self.name.is_none() {
                    return Err("Для команды 'add' требуется аргумент --name".to_string());
                }
                if self.id.is_some() {
                    return Err("Команда 'add' не принимает аргумент --id".to_string());
                }
            }
            "get" | "delete" => {
                if self.id.is_none() {
                    return Err(format!("Для команды '{}' требуется аргумент --id", self.command));
                }
                if self.name.is_some() {
                    return Err(format!("Команда '{}' не принимает аргумент --name", self.command));
                }
            }
            _ => return Err(format!("Неизвестная команда: {}", self.command)),
        }
        Ok(())
    }
}
