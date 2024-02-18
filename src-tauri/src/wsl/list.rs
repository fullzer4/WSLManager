use std::io;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct WSLDistribution {
    pub name: String,
}

pub fn list_wsl_distributions() -> io::Result<Vec<WSLDistribution>> {
    let output = Command::new("wsl.exe")
        .args(&["--list", "--quiet"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "Erro ao executar o comando wsl.exe"));
    }

    let output_str = match str::from_utf8(&output.stdout) {
        Ok(s) => s,
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Saída inválida do comando")),
    };

    let distributions: Vec<WSLDistribution> = output_str
        .lines()
        .filter_map(|line| {
            let trimmed_line = line.trim();
            if trimmed_line.len() > 1 {
                Some(WSLDistribution {
                    name: trimmed_line.to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    
    Ok(distributions)
}