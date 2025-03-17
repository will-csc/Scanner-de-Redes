# Scanner de Redes

Este projeto é um script em Rust que realiza a consulta de informações de um IP ou domínio fornecido, incluindo a verificação de país de origem, consulta WHOIS e escaneamento de portas abertas (1-1024).

![image](https://github.com/user-attachments/assets/f1d94500-33ae-4298-9cda-9d8ef960868c)
![image](https://github.com/user-attachments/assets/8521af1b-e89f-49fa-b098-6d425cf8c80d)


## Funcionalidades

1. **Consulta de país do IP**:
   - Utiliza a API pública `ip-api.com` para determinar o país de origem do IP fornecido.

2. **Consulta WHOIS**:
   - Executa uma consulta WHOIS utilizando o comando `whois` do sistema para obter informações sobre o IP ou domínio.

3. **Escaneamento de portas abertas**:
   - Escaneia as portas no intervalo de 1 a 1024 para verificar se há portas abertas no IP ou domínio fornecido.

## Requisitos

- **Rust**: O script foi escrito em Rust e requer que o compilador Rust esteja instalado em sua máquina. Você pode instalar o Rust através do [site oficial](https://www.rust-lang.org/).
- **Comando `whois`**: O script utiliza o comando `whois`, que deve estar disponível no sistema. Para sistemas Windows, você pode precisar instalar o utilitário de WHOIS manualmente.
- **Dependências**:
  - `reqwest`: Para realizar requisições HTTP.
  - `serde`: Para deserializar as respostas JSON da API.

