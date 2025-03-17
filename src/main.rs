use reqwest;
use serde::Deserialize;
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct IpInfo {
    country: String,
}

fn get_country(ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://ip-api.com/json/{}", ip);
    let response: IpInfo = reqwest::blocking::get(&url)?.json()?;
    Ok(response.country)
}

fn whois_lookup(target: &str) {
    println!("\nConsultando WHOIS para: {}", target);
    let output = Command::new("whois")
        .arg(target)
        .output()
        .expect("Falha ao executar whois");

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("Nenhuma resposta do WHOIS.");
    }
}

fn scan_port(ip: &str, port: u16) {
    if let Ok(addr) = format!("{}:{}", ip, port).to_socket_addrs() {
        for socket in addr {
            if TcpStream::connect_timeout(&socket, Duration::from_secs(1)).is_ok() {
                println!("[+] Porta aberta: {}", port);
                return;
            }
        }
    }
}

fn main() {
    println!("Digite um IP ou domínio para análise:");
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("Erro ao ler entrada");
    let target = target.trim();

    // Consulta do país
    match get_country(target) {
        Ok(country) => println!("\nO IP {} está registrado no país: {}", target, country),
        Err(_) => println!("\nNão foi possível determinar o país para {}", target),
    }

    // Consulta WHOIS
    whois_lookup(target);

    println!("\nEscaneando portas abertas (1-1024)...");

    let ports = 1..1025;

    let handles: Vec<_> = ports.map(|port| {
        let ip = target.to_string();
        thread::spawn(move || scan_port(&ip, port))
    }).collect();

    for handle in handles {
        let _ = handle.join();
    }
}
