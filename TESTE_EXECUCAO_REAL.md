# Teste de execução real

Execução real de processos cargo e do binário Rust. As entradas foram escritas em stdin; as saídas abaixo foram capturadas de stdout/stderr, sem reconstrução ou substituição por resumos.

## Compilação: saída completa

Comando: cargo build

stdout (bloco vazio significa zero bytes):
```text
```
stderr, incluindo mensagens normais do Cargo:
```text
   Compiling trabalho_ra1_ra2 v0.1.0 (C:\Users\yagok\Documents\ChatGPT\PUC\Trabalho-RA1-e-2---Rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.38s
```
Código de saída: 0

Comando: cargo clippy --all-targets -- -D warnings

stdout (bloco vazio significa zero bytes):
```text
```
stderr, incluindo mensagens normais do Cargo:
```text
    Checking trabalho_ra1_ra2 v0.1.0 (C:\Users\yagok\Documents\ChatGPT\PUC\Trabalho-RA1-e-2---Rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.73s
```
Código de saída: 0

## Tabela de todas as opções executadas

As linhas pertencem à mesma sessão; os valores separados por → são enviados em linhas distintas. Opções de criação e edição sem confirmação textual também são verificadas pelas listagens posteriores.

| Opção do menu | Entrada usada | Resultado | OK/FALHOU |
| --- | --- | --- | --- |
| Principal / 1 Arquivos | 1 | [Saída real integral](#opcao-1) | OK |
| Arquivos / 1 Criar teste | 1 → teste → 0 → 1000 → 1000 | [Saída real integral](#opcao-2) | OK |
| Arquivos / 1 Criar outro | 1 → outro → 10 → 1001 → 1001 | [Saída real integral](#opcao-3) | OK |
| Arquivos / 4 Listar e stat padrão | 4 | [Saída real integral](#opcao-4) | OK |
| Arquivos / 2 Editar | 2 → 1 | [Saída real integral](#opcao-5) | OK |
| Editar arquivo / 1 Nome | 1 → teste_editado | [Saída real integral](#opcao-6) | OK |
| Editar arquivo / 2 Tamanho | 2 → 42 | [Saída real integral](#opcao-7) | OK |
| Editar arquivo / 3 Permissões | 3 → 7 → 5 → 0 | [Saída real integral](#opcao-8) | OK |
| Editar arquivo / 4 Uid | 4 → 2000 | [Saída real integral](#opcao-9) | OK |
| Editar arquivo / 5 Gid | 5 → 3000 | [Saída real integral](#opcao-10) | OK |
| Editar arquivo / 0 Voltar | 0 | [Saída real integral](#opcao-11) | OK |
| Arquivos / 4 Conferir edição e preservação do outro | 4 | [Saída real integral](#opcao-12) | OK |
| Arquivos / 3 Apagar outro | 3 → 2 | [Saída real integral](#opcao-13) | OK |
| Arquivos / 0 Voltar | 0 | [Saída real integral](#opcao-14) | OK |
| Principal / 2 Diretórios | 2 | [Saída real integral](#opcao-15) | OK |
| Diretórios / 1 Criar | 1 → docs → 7 → 5 → 5 → 1000 | [Saída real integral](#opcao-16) | OK |
| Diretórios / 4 Listar | 4 | [Saída real integral](#opcao-17) | OK |
| Diretórios / 2 Editar | 2 → 1 | [Saída real integral](#opcao-18) | OK |
| Editar diretório / 1 Nome | 1 → documentos | [Saída real integral](#opcao-19) | OK |
| Editar diretório / 2 Permissões | 2 → 7 → 0 → 0 | [Saída real integral](#opcao-20) | OK |
| Editar diretório / 3 Uid | 3 → 2000 | [Saída real integral](#opcao-21) | OK |
| Editar diretório / 4 Adicionar arquivo | 4 → 1 | [Saída real integral](#opcao-22) | OK |
| Editar diretório / 7 Listar conteúdo e stat | 7 | [Saída real integral](#opcao-23) | OK |
| Editar diretório / 6 Editar arquivo interno | 6 → 1 | [Saída real integral](#opcao-24) | OK |
| Arquivo interno / 3 alterar_permissao | 3 → 6 → 6 → 4 | [Saída real integral](#opcao-25) | OK |
| Arquivo interno / 0 Voltar | 0 | [Saída real integral](#opcao-26) | OK |
| Editar diretório / 7 stat depois de alterar | 7 | [Saída real integral](#opcao-27) | OK |
| Editar diretório / 5 Remover arquivo | 5 → teste_editado | [Saída real integral](#opcao-28) | OK |
| Editar diretório / 7 Confirmar vazio | 7 | [Saída real integral](#opcao-29) | OK |
| Editar diretório / 0 Voltar | 0 | [Saída real integral](#opcao-30) | OK |
| Diretórios / 4 Conferir campos editados | 4 | [Saída real integral](#opcao-31) | OK |
| Diretórios / 3 Apagar | 3 → 1 | [Saída real integral](#opcao-32) | OK |
| Diretórios / 4 Confirmar exclusão | 4 | [Saída real integral](#opcao-33) | OK |
| Diretórios / 0 Voltar | 0 | [Saída real integral](#opcao-34) | OK |
| Principal / 3 Usuários | 3 | [Saída real integral](#opcao-35) | OK |
| Usuários / 1 Criar | 1 → Ana → 1000 | [Saída real integral](#opcao-36) | OK |
| Usuários / 4 Listar | 4 | [Saída real integral](#opcao-37) | OK |
| Usuários / 2 Editar | 2 → 1 | [Saída real integral](#opcao-38) | OK |
| Editar usuário / 1 Nome | 1 → Ana_Editada | [Saída real integral](#opcao-39) | OK |
| Editar usuário / 2 Uid | 2 → 2000 | [Saída real integral](#opcao-40) | OK |
| Editar usuário / 3 Adicionar grupo | 3 → 1000 | [Saída real integral](#opcao-41) | OK |
| Editar usuário / 5 Listar grupos | 5 | [Saída real integral](#opcao-42) | OK |
| Editar usuário / 4 Remover grupo | 4 → 1000 | [Saída real integral](#opcao-43) | OK |
| Editar usuário / 5 Confirmar sem grupos | 5 | [Saída real integral](#opcao-44) | OK |
| Editar usuário / 0 Voltar | 0 | [Saída real integral](#opcao-45) | OK |
| Usuários / 4 Conferir campos editados | 4 | [Saída real integral](#opcao-46) | OK |
| Usuários / 3 Apagar | 3 → 1 | [Saída real integral](#opcao-47) | OK |
| Usuários / 4 Confirmar exclusão | 4 | [Saída real integral](#opcao-48) | OK |
| Usuários / 0 Voltar | 0 | [Saída real integral](#opcao-49) | OK |
| Principal / 4 Grupos | 4 | [Saída real integral](#opcao-50) | OK |
| Grupos / 1 Criar | 1 → alunos → 1000 | [Saída real integral](#opcao-51) | OK |
| Grupos / 4 Listar | 4 | [Saída real integral](#opcao-52) | OK |
| Grupos / 2 Editar | 2 → 1 | [Saída real integral](#opcao-53) | OK |
| Editar grupo / 1 Nome | 1 → turma | [Saída real integral](#opcao-54) | OK |
| Editar grupo / 2 Gid | 2 → 2000 | [Saída real integral](#opcao-55) | OK |
| Editar grupo / 3 Adicionar membro | 3 → 1000 | [Saída real integral](#opcao-56) | OK |
| Editar grupo / 5 Listar membros | 5 | [Saída real integral](#opcao-57) | OK |
| Editar grupo / 4 Remover membro | 4 → 1000 | [Saída real integral](#opcao-58) | OK |
| Editar grupo / 5 Confirmar sem membros | 5 | [Saída real integral](#opcao-59) | OK |
| Editar grupo / 0 Voltar | 0 | [Saída real integral](#opcao-60) | OK |
| Grupos / 4 Conferir campos editados | 4 | [Saída real integral](#opcao-61) | OK |
| Grupos / 3 Apagar | 3 → 1 | [Saída real integral](#opcao-62) | OK |
| Grupos / 4 Confirmar exclusão | 4 | [Saída real integral](#opcao-63) | OK |
| Grupos / 0 Voltar | 0 | [Saída real integral](#opcao-64) | OK |
| Principal / 5 Permissão avulsa | 5 | [Saída real integral](#opcao-65) | OK |
| Permissão / 1 Criar | 1 → 6 | [Saída real integral](#opcao-66) | OK |
| Permissão / 4 Listar | 4 | [Saída real integral](#opcao-67) | OK |
| Permissão / 2 Editar | 2 → 7 | [Saída real integral](#opcao-68) | OK |
| Permissão / 4 Conferir edição | 4 | [Saída real integral](#opcao-69) | OK |
| Permissão / 3 Apagar | 3 | [Saída real integral](#opcao-70) | OK |
| Permissão / 4 Confirmar exclusão | 4 | [Saída real integral](#opcao-71) | OK |
| Permissão / 0 Voltar | 0 | [Saída real integral](#opcao-72) | OK |
| Principal / 0 Sair | 0 | [Saída real integral](#opcao-73) | OK |

## Saída inicial real
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```

<a id="opcao-1"></a>
## 1. Principal / 1 Arquivos

Entrada enviada:
```text
1
```
Saída real, sem cortes:
```text

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-2"></a>
## 2. Arquivos / 1 Criar teste

Entrada enviada:
```text
1
teste
0
1000
1000
```
Saída real, sem cortes:
```text
Nome do arquivo:
Tamanho em bytes:
Uid do dono:
Gid do grupo:

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-3"></a>
## 3. Arquivos / 1 Criar outro

Entrada enviada:
```text
1
outro
10
1001
1001
```
Saída real, sem cortes:
```text
Nome do arquivo:
Tamanho em bytes:
Uid do dono:
Gid do grupo:

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-4"></a>
## 4. Arquivos / 4 Listar e stat padrão

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste
Tamanho: 0 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1000
Gid: 1000

Item 2
Arquivo: outro
Tamanho: 10 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1001
Gid: 1001

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-5"></a>
## 5. Arquivos / 2 Editar

Entrada enviada:
```text
2
1
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste
Tamanho: 0 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1000
Gid: 1000

Item 2
Arquivo: outro
Tamanho: 10 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1001
Gid: 1001
Escolha um item de 1 a 2.
Número do item:

Editar arquivo: teste
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-6"></a>
## 6. Editar arquivo / 1 Nome

Entrada enviada:
```text
1
teste_editado
```
Saída real, sem cortes:
```text
Novo nome:

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-7"></a>
## 7. Editar arquivo / 2 Tamanho

Entrada enviada:
```text
2
42
```
Saída real, sem cortes:
```text
Novo tamanho:

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-8"></a>
## 8. Editar arquivo / 3 Permissões

Entrada enviada:
```text
3
7
5
0
```
Saída real, sem cortes:
```text
Dono:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Grupo:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Outros:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-9"></a>
## 9. Editar arquivo / 4 Uid

Entrada enviada:
```text
4
2000
```
Saída real, sem cortes:
```text
Novo uid:

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-10"></a>
## 10. Editar arquivo / 5 Gid

Entrada enviada:
```text
5
3000
```
Saída real, sem cortes:
```text
Novo gid:

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-11"></a>
## 11. Editar arquivo / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-12"></a>
## 12. Arquivos / 4 Conferir edição e preservação do outro

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000

Item 2
Arquivo: outro
Tamanho: 10 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1001
Gid: 1001

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-13"></a>
## 13. Arquivos / 3 Apagar outro

Entrada enviada:
```text
3
2
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000

Item 2
Arquivo: outro
Tamanho: 10 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1001
Gid: 1001
Escolha um item de 1 a 2.
Número do item:
Arquivo apagado.

ARQUIVOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-14"></a>
## 14. Arquivos / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```
Resultado: OK

<a id="opcao-15"></a>
## 15. Principal / 2 Diretórios

Entrada enviada:
```text
2
```
Saída real, sem cortes:
```text

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-16"></a>
## 16. Diretórios / 1 Criar

Entrada enviada:
```text
1
docs
7
5
5
1000
```
Saída real, sem cortes:
```text
Nome do diretório:
Dono:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Grupo:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Outros:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Uid do dono:

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-17"></a>
## 17. Diretórios / 4 Listar

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Diretório: docs
Uid: 1000
Permissoes: (755 / rwxr-xr-x)
Diretório vazio.

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-18"></a>
## 18. Diretórios / 2 Editar

Entrada enviada:
```text
2
1
```
Saída real, sem cortes:
```text

Item 1 - Diretório: docs
Uid: 1000
Permissoes: (755 / rwxr-xr-x)
Diretório vazio.
Escolha um item de 1 a 1.
Número do item:

Editar diretório: docs
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-19"></a>
## 19. Editar diretório / 1 Nome

Entrada enviada:
```text
1
documentos
```
Saída real, sem cortes:
```text
Novo nome:

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-20"></a>
## 20. Editar diretório / 2 Permissões

Entrada enviada:
```text
2
7
0
0
```
Saída real, sem cortes:
```text
Dono:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Grupo:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Outros:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-21"></a>
## 21. Editar diretório / 3 Uid

Entrada enviada:
```text
3
2000
```
Saída real, sem cortes:
```text
Novo uid:

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-22"></a>
## 22. Editar diretório / 4 Adicionar arquivo

Entrada enviada:
```text
4
1
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000
Escolha um item de 1 a 1.
Número do item:
Arquivo movido para o diretório.

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-23"></a>
## 23. Editar diretório / 7 Listar conteúdo e stat

Entrada enviada:
```text
7
```
Saída real, sem cortes:
```text
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-24"></a>
## 24. Editar diretório / 6 Editar arquivo interno

Entrada enviada:
```text
6
1
```
Saída real, sem cortes:
```text

Item 1
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000
Escolha um item de 1 a 1.
Número do item:

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-25"></a>
## 25. Arquivo interno / 3 alterar_permissao

Entrada enviada:
```text
3
6
6
4
```
Saída real, sem cortes:
```text
Dono:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Grupo:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):
Outros:
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):

Editar arquivo: teste_editado
1 - Nome
2 - Tamanho
3 - Permissões
4 - Uid
5 - Gid
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-26"></a>
## 26. Arquivo interno / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-27"></a>
## 27. Editar diretório / 7 stat depois de alterar

Entrada enviada:
```text
7
```
Saída real, sem cortes:
```text
Arquivo: teste_editado
Tamanho: 42 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 2000
Gid: 3000

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-28"></a>
## 28. Editar diretório / 5 Remover arquivo

Entrada enviada:
```text
5
teste_editado
```
Saída real, sem cortes:
```text
Nome do arquivo a apagar:
Arquivo apagado do diretório.

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-29"></a>
## 29. Editar diretório / 7 Confirmar vazio

Entrada enviada:
```text
7
```
Saída real, sem cortes:
```text
Diretório vazio.

Editar diretório: documentos
1 - Nome
2 - Permissões
3 - Uid
4 - Mover arquivo para cá
5 - Apagar arquivo pelo nome
6 - Editar arquivo
7 - Listar conteúdo
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-30"></a>
## 30. Editar diretório / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-31"></a>
## 31. Diretórios / 4 Conferir campos editados

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Diretório: documentos
Uid: 2000
Permissoes: (700 / rwx------)
Diretório vazio.

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-32"></a>
## 32. Diretórios / 3 Apagar

Entrada enviada:
```text
3
1
```
Saída real, sem cortes:
```text

Item 1 - Diretório: documentos
Uid: 2000
Permissoes: (700 / rwx------)
Diretório vazio.
Escolha um item de 1 a 1.
Número do item:
Diretório e seus arquivos apagados.

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-33"></a>
## 33. Diretórios / 4 Confirmar exclusão

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Nenhum diretório cadastrado.

DIRETÓRIOS
1 - Criar
2 - Editar
3 - Apagar diretório e seus arquivos
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-34"></a>
## 34. Diretórios / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```
Resultado: OK

<a id="opcao-35"></a>
## 35. Principal / 3 Usuários

Entrada enviada:
```text
3
```
Saída real, sem cortes:
```text

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-36"></a>
## 36. Usuários / 1 Criar

Entrada enviada:
```text
1
Ana
1000
```
Saída real, sem cortes:
```text
Nome do usuário:
Uid:

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-37"></a>
## 37. Usuários / 4 Listar

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Usuário: Ana | Uid: 1000
Sem grupos.

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-38"></a>
## 38. Usuários / 2 Editar

Entrada enviada:
```text
2
1
```
Saída real, sem cortes:
```text

Item 1 - Usuário: Ana | Uid: 1000
Sem grupos.
Escolha um item de 1 a 1.
Número do item:

Editar usuário: Ana
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-39"></a>
## 39. Editar usuário / 1 Nome

Entrada enviada:
```text
1
Ana_Editada
```
Saída real, sem cortes:
```text
Novo nome:

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-40"></a>
## 40. Editar usuário / 2 Uid

Entrada enviada:
```text
2
2000
```
Saída real, sem cortes:
```text
Novo uid:

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-41"></a>
## 41. Editar usuário / 3 Adicionar grupo

Entrada enviada:
```text
3
1000
```
Saída real, sem cortes:
```text
Gid a adicionar:

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-42"></a>
## 42. Editar usuário / 5 Listar grupos

Entrada enviada:
```text
5
```
Saída real, sem cortes:
```text
Gid: 1000

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-43"></a>
## 43. Editar usuário / 4 Remover grupo

Entrada enviada:
```text
4
1000
```
Saída real, sem cortes:
```text
Gid a remover:
Grupo removido do usuário.

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-44"></a>
## 44. Editar usuário / 5 Confirmar sem grupos

Entrada enviada:
```text
5
```
Saída real, sem cortes:
```text
Sem grupos.

Editar usuário: Ana_Editada
1 - Nome
2 - Uid
3 - Adicionar grupo
4 - Remover grupo
5 - Listar grupos
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-45"></a>
## 45. Editar usuário / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-46"></a>
## 46. Usuários / 4 Conferir campos editados

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Usuário: Ana_Editada | Uid: 2000
Sem grupos.

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-47"></a>
## 47. Usuários / 3 Apagar

Entrada enviada:
```text
3
1
```
Saída real, sem cortes:
```text

Item 1 - Usuário: Ana_Editada | Uid: 2000
Sem grupos.
Escolha um item de 1 a 1.
Número do item:
Usuário apagado.

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-48"></a>
## 48. Usuários / 4 Confirmar exclusão

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Nenhum usuário cadastrado.

USUÁRIOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-49"></a>
## 49. Usuários / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```
Resultado: OK

<a id="opcao-50"></a>
## 50. Principal / 4 Grupos

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-51"></a>
## 51. Grupos / 1 Criar

Entrada enviada:
```text
1
alunos
1000
```
Saída real, sem cortes:
```text
Nome do grupo:
Gid:

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-52"></a>
## 52. Grupos / 4 Listar

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Grupo: alunos | Gid: 1000
Sem membros.

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-53"></a>
## 53. Grupos / 2 Editar

Entrada enviada:
```text
2
1
```
Saída real, sem cortes:
```text

Item 1 - Grupo: alunos | Gid: 1000
Sem membros.
Escolha um item de 1 a 1.
Número do item:

Editar grupo: alunos
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-54"></a>
## 54. Editar grupo / 1 Nome

Entrada enviada:
```text
1
turma
```
Saída real, sem cortes:
```text
Novo nome:

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-55"></a>
## 55. Editar grupo / 2 Gid

Entrada enviada:
```text
2
2000
```
Saída real, sem cortes:
```text
Novo gid:

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-56"></a>
## 56. Editar grupo / 3 Adicionar membro

Entrada enviada:
```text
3
1000
```
Saída real, sem cortes:
```text
Uid a adicionar:

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-57"></a>
## 57. Editar grupo / 5 Listar membros

Entrada enviada:
```text
5
```
Saída real, sem cortes:
```text
Uid: 1000

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-58"></a>
## 58. Editar grupo / 4 Remover membro

Entrada enviada:
```text
4
1000
```
Saída real, sem cortes:
```text
Uid a remover:
Membro removido do grupo.

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-59"></a>
## 59. Editar grupo / 5 Confirmar sem membros

Entrada enviada:
```text
5
```
Saída real, sem cortes:
```text
Sem membros.

Editar grupo: turma
1 - Nome
2 - Gid
3 - Adicionar membro
4 - Remover membro
5 - Listar membros
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-60"></a>
## 60. Editar grupo / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-61"></a>
## 61. Grupos / 4 Conferir campos editados

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text

Item 1 - Grupo: turma | Gid: 2000
Sem membros.

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-62"></a>
## 62. Grupos / 3 Apagar

Entrada enviada:
```text
3
1
```
Saída real, sem cortes:
```text

Item 1 - Grupo: turma | Gid: 2000
Sem membros.
Escolha um item de 1 a 1.
Número do item:
Grupo apagado.

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-63"></a>
## 63. Grupos / 4 Confirmar exclusão

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Nenhum grupo cadastrado.

GRUPOS
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-64"></a>
## 64. Grupos / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```
Resultado: OK

<a id="opcao-65"></a>
## 65. Principal / 5 Permissão avulsa

Entrada enviada:
```text
5
```
Saída real, sem cortes:
```text

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-66"></a>
## 66. Permissão / 1 Criar

Entrada enviada:
```text
1
6
```
Saída real, sem cortes:
```text
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-67"></a>
## 67. Permissão / 4 Listar

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Permissão: 6 / rw-

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-68"></a>
## 68. Permissão / 2 Editar

Entrada enviada:
```text
2
7
```
Saída real, sem cortes:
```text
Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-69"></a>
## 69. Permissão / 4 Conferir edição

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Permissão: 7 / rwx

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-70"></a>
## 70. Permissão / 3 Apagar

Entrada enviada:
```text
3
```
Saída real, sem cortes:
```text
Permissão apagada.

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-71"></a>
## 71. Permissão / 4 Confirmar exclusão

Entrada enviada:
```text
4
```
Saída real, sem cortes:
```text
Permissão não encontrada.

PERMISSÃO AVULSA
1 - Criar
2 - Editar
3 - Apagar
4 - Listar
0 - Voltar
Opção:
```
Resultado: OK

<a id="opcao-72"></a>
## 72. Permissão / 0 Voltar

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text

MENU PRINCIPAL
1 - Arquivos
2 - Diretórios
3 - Usuários
4 - Grupos
5 - Permissão avulsa
0 - Sair
Opção:
```
Resultado: OK

<a id="opcao-73"></a>
## 73. Principal / 0 Sair

Entrada enviada:
```text
0
```
Saída real, sem cortes:
```text
Programa encerrado.
```
Resultado: OK

## Repetição via cargo run

O conteúdo integral de entrada.txt também foi enviado ao stdin de cargo run. A saída de stdout coincidiu com a sessão anterior, após normalizar apenas quebras de linha CRLF/LF.
Saídas integrais: [stdout](testes/resultados/cargo-run.stdout.txt) e [stderr](testes/resultados/cargo-run.stderr.txt).

## Estresse: processos reais independentes

Cada caso teve limite de 10 segundos. Casos normais exigem saída pela opção 0; nos casos EOF, o encerramento é esperado e deve emitir Entrada encerrada., sem repetição infinita.

| Caso | Entrada exata | Saída real integral | Código | Timeout | OK/FALHOU |
| --- | --- | --- | --- | --- | --- |
| entradas ruins em principal | [entrada](testes/resultados/estresse-01.entrada.txt) | [stdout](testes/resultados/estresse-01.stdout.txt), [stderr](testes/resultados/estresse-01.stderr.txt) | 0 | False | OK |
| entradas ruins em arquivos | [entrada](testes/resultados/estresse-02.entrada.txt) | [stdout](testes/resultados/estresse-02.stdout.txt), [stderr](testes/resultados/estresse-02.stderr.txt) | 0 | False | OK |
| entradas ruins em diretorios | [entrada](testes/resultados/estresse-03.entrada.txt) | [stdout](testes/resultados/estresse-03.stdout.txt), [stderr](testes/resultados/estresse-03.stderr.txt) | 0 | False | OK |
| entradas ruins em usuarios | [entrada](testes/resultados/estresse-04.entrada.txt) | [stdout](testes/resultados/estresse-04.stdout.txt), [stderr](testes/resultados/estresse-04.stderr.txt) | 0 | False | OK |
| entradas ruins em grupos | [entrada](testes/resultados/estresse-05.entrada.txt) | [stdout](testes/resultados/estresse-05.stdout.txt), [stderr](testes/resultados/estresse-05.stderr.txt) | 0 | False | OK |
| entradas ruins em permissao | [entrada](testes/resultados/estresse-06.entrada.txt) | [stdout](testes/resultados/estresse-06.stdout.txt), [stderr](testes/resultados/estresse-06.stderr.txt) | 0 | False | OK |
| entradas ruins em editar arquivo | [entrada](testes/resultados/estresse-07.entrada.txt) | [stdout](testes/resultados/estresse-07.stdout.txt), [stderr](testes/resultados/estresse-07.stderr.txt) | 0 | False | OK |
| entradas ruins em editar diretorio | [entrada](testes/resultados/estresse-08.entrada.txt) | [stdout](testes/resultados/estresse-08.stdout.txt), [stderr](testes/resultados/estresse-08.stderr.txt) | 0 | False | OK |
| entradas ruins em editar usuario | [entrada](testes/resultados/estresse-09.entrada.txt) | [stdout](testes/resultados/estresse-09.stdout.txt), [stderr](testes/resultados/estresse-09.stderr.txt) | 0 | False | OK |
| entradas ruins em editar grupo | [entrada](testes/resultados/estresse-10.entrada.txt) | [stdout](testes/resultados/estresse-10.stdout.txt), [stderr](testes/resultados/estresse-10.stderr.txt) | 0 | False | OK |
| limites u16 e u64 | [entrada](testes/resultados/estresse-11.entrada.txt) | [stdout](testes/resultados/estresse-11.stdout.txt), [stderr](testes/resultados/estresse-11.stderr.txt) | 0 | False | OK |
| listar editar e apagar tudo vazio | [entrada](testes/resultados/estresse-12.entrada.txt) | [stdout](testes/resultados/estresse-12.stdout.txt), [stderr](testes/resultados/estresse-12.stderr.txt) | 0 | False | OK |
| arquivo ausente no diretorio | [entrada](testes/resultados/estresse-13.entrada.txt) | [stdout](testes/resultados/estresse-13.stdout.txt), [stderr](testes/resultados/estresse-13.stderr.txt) | 0 | False | OK |
| indice inexistente em arquivo | [entrada](testes/resultados/estresse-14.entrada.txt) | [stdout](testes/resultados/estresse-14.stdout.txt), [stderr](testes/resultados/estresse-14.stderr.txt) | 0 | False | OK |
| nome vazio e com 1200 caracteres em arquivo | [entrada](testes/resultados/estresse-15.entrada.txt) | [stdout](testes/resultados/estresse-15.stdout.txt), [stderr](testes/resultados/estresse-15.stderr.txt) | 0 | False | OK |
| indice inexistente em diretorio | [entrada](testes/resultados/estresse-16.entrada.txt) | [stdout](testes/resultados/estresse-16.stdout.txt), [stderr](testes/resultados/estresse-16.stderr.txt) | 0 | False | OK |
| nome vazio e com 1200 caracteres em diretorio | [entrada](testes/resultados/estresse-17.entrada.txt) | [stdout](testes/resultados/estresse-17.stdout.txt), [stderr](testes/resultados/estresse-17.stderr.txt) | 0 | False | OK |
| indice inexistente em usuario | [entrada](testes/resultados/estresse-18.entrada.txt) | [stdout](testes/resultados/estresse-18.stdout.txt), [stderr](testes/resultados/estresse-18.stderr.txt) | 0 | False | OK |
| nome vazio e com 1200 caracteres em usuario | [entrada](testes/resultados/estresse-19.entrada.txt) | [stdout](testes/resultados/estresse-19.stdout.txt), [stderr](testes/resultados/estresse-19.stderr.txt) | 0 | False | OK |
| indice inexistente em grupo | [entrada](testes/resultados/estresse-20.entrada.txt) | [stdout](testes/resultados/estresse-20.stdout.txt), [stderr](testes/resultados/estresse-20.stderr.txt) | 0 | False | OK |
| nome vazio e com 1200 caracteres em grupo | [entrada](testes/resultados/estresse-21.entrada.txt) | [stdout](testes/resultados/estresse-21.stdout.txt), [stderr](testes/resultados/estresse-21.stderr.txt) | 0 | False | OK |
| gid duplicado e ausente no usuario | [entrada](testes/resultados/estresse-22.entrada.txt) | [stdout](testes/resultados/estresse-22.stdout.txt), [stderr](testes/resultados/estresse-22.stderr.txt) | 0 | False | OK |
| uid duplicado e ausente no grupo | [entrada](testes/resultados/estresse-23.entrada.txt) | [stdout](testes/resultados/estresse-23.stdout.txt), [stderr](testes/resultados/estresse-23.stderr.txt) | 0 | False | OK |
| oito combinacoes de permissao | [entrada](testes/resultados/estresse-24.entrada.txt) | [stdout](testes/resultados/estresse-24.stdout.txt), [stderr](testes/resultados/estresse-24.stderr.txt) | 0 | False | OK |
| permissao invalida e recuperacao | [entrada](testes/resultados/estresse-25.entrada.txt) | [stdout](testes/resultados/estresse-25.stdout.txt), [stderr](testes/resultados/estresse-25.stderr.txt) | 0 | False | OK |
| mil opcoes inexistentes e recuperacao | [entrada](testes/resultados/estresse-26.entrada.txt) | [stdout](testes/resultados/estresse-26.stdout.txt), [stderr](testes/resultados/estresse-26.stderr.txt) | 0 | False | OK |
| preservacao do arquivo 2 ao editar e apagar arquivo 1 | [entrada](testes/resultados/estresse-27.entrada.txt) | [stdout](testes/resultados/estresse-27.stdout.txt), [stderr](testes/resultados/estresse-27.stderr.txt) | 0 | False | OK |
| preservacao do usuario 2 ao editar e apagar usuario 1 | [entrada](testes/resultados/estresse-28.entrada.txt) | [stdout](testes/resultados/estresse-28.stdout.txt), [stderr](testes/resultados/estresse-28.stderr.txt) | 0 | False | OK |
| preservacao do grupo 2 ao editar e apagar grupo 1 | [entrada](testes/resultados/estresse-29.entrada.txt) | [stdout](testes/resultados/estresse-29.stdout.txt), [stderr](testes/resultados/estresse-29.stderr.txt) | 0 | False | OK |
| preservacao do diretorio 2 e arquivo interno | [entrada](testes/resultados/estresse-30.entrada.txt) | [stdout](testes/resultados/estresse-30.stdout.txt), [stderr](testes/resultados/estresse-30.stderr.txt) | 0 | False | OK |
| EOF no principal | [entrada](testes/resultados/estresse-31.entrada.txt) | [stdout](testes/resultados/estresse-31.stdout.txt), [stderr](testes/resultados/estresse-31.stderr.txt) | 0 | False | OK |
| EOF no nome | [entrada](testes/resultados/estresse-32.entrada.txt) | [stdout](testes/resultados/estresse-32.stdout.txt), [stderr](testes/resultados/estresse-32.stderr.txt) | 0 | False | OK |
| EOF no uid | [entrada](testes/resultados/estresse-33.entrada.txt) | [stdout](testes/resultados/estresse-33.stdout.txt), [stderr](testes/resultados/estresse-33.stderr.txt) | 0 | False | OK |
| EOF no tamanho | [entrada](testes/resultados/estresse-34.entrada.txt) | [stdout](testes/resultados/estresse-34.stdout.txt), [stderr](testes/resultados/estresse-34.stderr.txt) | 0 | False | OK |
| EOF no meio das permissoes | [entrada](testes/resultados/estresse-35.entrada.txt) | [stdout](testes/resultados/estresse-35.stdout.txt), [stderr](testes/resultados/estresse-35.stderr.txt) | 0 | False | OK |
| EOF no indice | [entrada](testes/resultados/estresse-36.entrada.txt) | [stdout](testes/resultados/estresse-36.stdout.txt), [stderr](testes/resultados/estresse-36.stderr.txt) | 0 | False | OK |
| sessao longa com 217 opcoes de menu | [entrada](testes/resultados/estresse-37.entrada.txt) | [stdout](testes/resultados/estresse-37.stdout.txt), [stderr](testes/resultados/estresse-37.stderr.txt) | 0 | False | OK |

Falhas totais: 0

## Conferência final e limites da evidência

O stat padrão foi comparado como cinco linhas consecutivas na opção 4, com nome teste, tamanho 0, permissões 664 / rw-rw-r--, uid 1000 e gid 1000.
Os quatro casos de preservação conferem os campos completos do segundo item após editar e apagar o primeiro; o caso de diretórios também verifica o arquivo que pertence ao segundo diretório.
Também foi realmente compilado testes/conformidade.rs, que inclui o main.rs atual e atribui os 17 métodos a ponteiros de função tipados. Os 18 campos, construtores, pesos octais e mutações de vetores também foram verificados. Esse arquivo é um teste separado; a implementação continua apenas em src/main.rs.

Saída integral da execução desses quatro testes:
```text

running 4 tests
test conferencia::assinaturas_exatas_do_enunciado ... ok
test conferencia::campos_e_tipos_exatos ... ok
test conferencia::construtores_e_permissoes ... ok
test conferencia::vetores_inicialmente_vazios_e_operacoes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
Código de saída: 0; stderr vazio: True
SHA256 do main.rs executado: 67E5B1C6B00BC808ED0733DB0C7C1D87964E3CB72E46E87A42635AAB129FEC66

Analisado no código: comentários, manutenção dos nomes, uso de &self e &mut self, guardas dos índices e correspondência com o enunciado transcrito. A Tabela 2 original não foi fornecida; permanece a interpretação de nome_arquivo como &str. A tabela detalhada da análise está em VALIDACAO_FINAL.md.
Não foi pressionado Ctrl+D em uma interface gráfica: o teste fechou stdin de verdade no meio da entrada, produzindo EOF. Os seis processos responderam Entrada encerrada. e saíram com código 0; esse encerramento é intencional.
Os resultados demonstram ausência de panic e timeout nestas entradas. Não são prova para todas as entradas possíveis e não cobrem falta de memória, falha física do terminal ou falha de stdout. Os processos de estresse tiveram limite de 10 segundos.

## Bugs encontrados e correções

- Programa Rust: nenhum bug encontrado nesta execução; src/main.rs não foi alterado.
- Executor de testes: corrigida a resolução do Cargo quando há mais de um cargo.exe no PATH; passa a selecionar o primeiro. A tentativa anterior falhou antes de iniciar cargo build. Depois da correção, a bateria inteira foi executada novamente.

## Reproduzir

PowerShell 7, na raiz do projeto: pwsh -File testes/executar.ps1
Bash ou cmd.exe, para a sessão completa: cargo run < entrada.txt
No PowerShell, o operador < não redireciona stdin; use cmd /c "cargo run < entrada.txt".

O teste roda o programa; não se limita a gerar arquivos de entrada. EOF foi produzido fechando stdin do processo, equivalente ao efeito de Ctrl+D em um terminal Unix.
