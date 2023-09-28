# Task Manager


 Está é uma aplicação back-end feita em Rust consumindo uma base de dos MYSQL utilizando atutenticação JSON Web Tokens(JWT) totalmente funcional com tempo de expiração configurado para 1 hora que pode ser alterado no arquivo `controller.rs linha 27`

## Instalação
Asegure que já tenha o Rust e o Cargo instalado na sua maquina, se não só seguir os passo do link baixo

<table>
<tr>
<td align="center"><a href="https://www.rust-lang.org/pt-BR/learn/get-started" target="_blank" rel="noreferrer noopener" title="Instalar Rust"><img src="https://static-00.iconduck.com/assets.00/file-type-rust-icon-256x256-f81hgcmj.png" width="64" /></td>
</tr>
<tr>
<td align="center">Rust</td>
</tr>
</table>

1. Clone o repositório.

## Banco de dados
Neste projeto foi utilizado o banco de dados Mysql segue abaixo os scripts para criação das tabelas
1. Tabela Teams

CREATE TABLE `teams` (
  `team_id` int(11) NOT NULL AUTO_INCREMENT,
  `type_teams` varchar(4) NOT NULL,
  `description` varchar(150) NOT NULL,
  PRIMARY KEY (`team_id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

2. Tabela Users
-- apstech.users definition

CREATE TABLE `users` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `name_user` varchar(20) NOT NULL,
  `password` varchar(50) CHARACTER SET armscii8 COLLATE armscii8_general_ci DEFAULT NULL,
  `date_register` datetime DEFAULT current_timestamp(),
  `date_update` datetime DEFAULT NULL,
  `team` int(11) NOT NULL,
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB AUTO_INCREMENT=3700 DEFAULT CHARSET=latin1 COLLATE=latin1_swedish_ci;

3. Para configurar a string do seu banco de dados basta criar o arquivo `.env` na raiz do projeto e incluir a linha `DATABASE_URL=mysql://USUARIO:SENHA@IPBASE/NOMEBASE` ou em seu terminal digitar `export DATABASE_URL=mysql://USUARIO:SENHA@IPBASE/NOMEBASE`
   
## Uso

1. Execute o comando `cargo run` ou se preferir `cargo whatch -x run` para iniciar o projeto.
2. Acesse o projeto em `http://localhost:8088`.
   
   2.1. Caso queria iniciar este projeto em uma porta diferente basta acessar o arquivo `Rocket.toml` e port para a porta desejada

## Contribuição

1. Faça um fork do repositório.
2. Crie uma branch para sua contribuição.
3. Faça as alterações desejadas.
4. Envie um pull request.

## Desenvolvedor
<img src="bf71a9a3-574e-41d3-90e7-a3839c85d634.jpeg" width="64" />

Alexandre Silva

Dev Senior apaixonado por tecnologia
