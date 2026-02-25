use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(index = 1)]
    pub command: String,

    #[arg(index = 2)]
    pub arg: Option<String>,
}

pub fn parse_args(cli: Cli) -> Result<(String, Option<String>), String> {
    match cli.command.as_str() {
        "add" => {
            if let Some(ref name) = cli.arg {
                if name.is_empty() {
                    return Err("Название задачи не может быть пустым".to_string());
                }
                Ok((cli.command, cli.arg))
            } else {
                Err("Для команды 'add' требуется название задачи".to_string())
            }
        }
        "get" | "delete" => {
            if let Some(ref id_str) = cli.arg {
                if id_str.parse::<u32>().is_err() {
                    return Err(format!("ID должен быть числом: {}", id_str));
                }
                Ok((cli.command, cli.arg))
            } else {
                Err(format!("Для команды '{}' требуется ID задачи", cli.command))
            }
        }
        _ => Err(format!("Неизвестная команда: {}. Доступные: add, get, delete", cli.command)),
    }
}
