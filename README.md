# Trabalho RA1 e RA2 — Rust

Menu interativo de Programação Imperativa, com foco em ownership e borrowing.
Toda a implementação fica em [src/main.rs](src/main.rs), usando apenas a biblioteca padrão.

Dupla: **Yago Landin e Gean Karlo**, identificada na primeira linha do código.

## Compilar e executar

Com Rust e Cargo instalados, abra o terminal na pasta do projeto:

```sh
cargo run
```

Para somente verificar a compilação ou gerar o executável:

```sh
cargo check
cargo build
```

Não há crates externas nem macros próprias. As macros padrão `println!` são usadas para a saída no terminal.

## Como usar

O menu principal oferece arquivos, diretórios, usuários, grupos e uma permissão avulsa.
Cada submenu tem criar, editar, apagar, listar e voltar.
Os itens são selecionados pelo número da listagem, começando em 1.
No menu principal, 0 encerra o programa.

- Arquivos: editar nome, tamanho, permissões, uid e gid.
- Diretórios: editar nome, permissões e uid; mover um arquivo para dentro; editar, apagar e listar seu conteúdo.
- Usuários: editar nome e uid; adicionar, remover e listar IDs de grupos.
- Grupos: editar nome e gid; adicionar, remover e listar IDs de membros.
- Permissão avulsa: mantém uma `Option<Permissao>` para demonstrar o CRUD da estrutura isolada.
- Permissões de arquivos e diretórios: digitar um número de 0 a 7 para dono, grupo e outros, nessa ordem.

Os quatro vetores principais são criados no `main` e emprestados aos submenus.
Mover um arquivo para um diretório o retira da listagem principal de arquivos.
O diretório passa a ser seu dono; apagar o diretório apaga também seus arquivos.
A remoção por nome apaga a primeira ocorrência encontrada.
Os dados ficam apenas na memória e são descartados ao sair.

Usuários e grupos mantêm listas de IDs independentes, conforme os métodos do enunciado.
Adicionar um gid ao usuário não adiciona automaticamente seu uid ao grupo.
Editar ou apagar usuários e grupos também não reescreve os IDs guardados nas demais estruturas.
Não é necessário cadastrar previamente o dono ou grupo de um arquivo.
IDs de associação repetidos no mesmo vetor são recusados. Nomes repetidos na mesma lista são recusados na criação e na renomeação; a comparação diferencia maiúsculas de minúsculas. IDs dos cadastros podem se repetir.
A permissão avulsa é independente das permissões de arquivos e diretórios.

Entradas numéricas inválidas são informadas e solicitadas novamente.
Seleções fora da listagem retornam uma mensagem sem acessar índices inválidos.
Nomes vazios são recusados. Fim de entrada encerra o programa, sem ficar repetindo o menu.

## Borrowing e move (até 10 linhas)

1. `&self` empresta a estrutura para consultar, sem alterá-la ou consumi-la.
2. `octal`, `stat` e os métodos de listagem usam esse empréstimo imutável.
3. `&mut self` permite mudar a estrutura por um empréstimo mutável exclusivo.
4. Trocar permissões e adicionar ou remover elementos dos vetores usam `&mut self`.
5. `for arquivo in &self.arquivos` percorre referências, mantendo os arquivos no vetor.
6. `&str` e `&[Arquivo]` permitem consultar texto e uma sequência sem tomar sua posse.
7. Os construtores recebem `String` por valor e movem o nome para a estrutura.
8. `adiciona_arquivo` recebe `Arquivo` por valor porque o diretório será seu dono.
9. `Vec::remove` devolve o arquivo; passá-lo ao diretório move esse valor, sem cloná-lo.
10. `u16` e `bool` são copiados; transferir esses valores não invalida a variável original.

## Cinco perguntas de autoria

1. **Por que `stat` recebe `&self`, e não `self`?**
   Ele só consulta o arquivo. Com `self`, consumiria o valor recebido; com `&self`,
   o dono pode continuar usando o arquivo depois da impressão.
2. **Por que `alterar_permissao` precisa de `&mut self`?**
   Porque substitui um campo. O empréstimo mutável dá acesso exclusivo temporário
   para modificar o arquivo, sem transferir sua posse.
3. **O que acontece depois de `diretorio.adiciona_arquivo(arquivo)`?**
   O valor é movido para o vetor do diretório. A variável `arquivo` não pode mais
   ser usada; o arquivo passa a ser acessado pelo diretório.
4. **Por que usar `for arquivo in &self.arquivos`?**
   Para percorrer referências. Consumir o vetor tentaria retirar os arquivos
   de uma estrutura que o método recebeu apenas emprestada.
5. **Como `rw-rw-r--` vira 664?**
   Cada trio soma leitura=4, escrita=2 e execução=1. Dono e grupo somam
   4+2=6; outros somam 4. São três dígitos octais exibidos lado a lado.

## Roteiro de teste manual

1. Execute `cargo run`. Digite `abc`, `-1`, `65536` e depois `9`:
   todos devem receber: Opção inválida. Digite um número de 0 a 5.
2. Entre em **1 — Arquivos**, escolha **1 — Criar** e informe:
   `teste`, `0`, `1000`, `1000`. Em **4 — Listar**, confira:

   ```text
   Arquivo: teste
   Tamanho: 0 bytes
   Permissoes: (664 / rw-rw-r--)
   Uid: 1000
   Gid: 1000
   ```

3. Escolha **2 — Editar**, selecione o item `1`, entre em
   **3 — Permissões** e digite `7`, `5`, `0`.
   Volte com `0` e liste: deve aparecer `750 / rwxr-x---`.
   Teste também editar nome, tamanho, uid e gid.
4. Volte ao principal. Em **2 — Diretórios**, crie `documentos`,
   permissões `7`, `5`, `5` e uid `1000`.
   Edite o diretório `1`, escolha **4 — Mover arquivo para cá** e selecione `1`.
   Liste o conteúdo: o arquivo deve estar no diretório e sair do vetor principal de arquivos.
   Use **6 — Editar arquivo** para verificar a edição dentro do diretório.
5. Em **3 — Usuários**, crie `Ana` com uid `1000`. Edite o item `1`,
   adicione gid `100` duas vezes e liste: deve haver um único gid.
   Remova `999` (não encontrado), depois `100` e confira que a lista ficou vazia.
6. Em **4 — Grupos**, crie `alunos` com gid `100`. Edite o item `1`,
   adicione uid `1000` duas vezes, liste, remova `999`, depois `1000` e liste novamente.
   Edite também nome e gid; no usuário, edite nome e uid.
7. Em **5 — Permissão avulsa**, tente listar antes de criar, crie `6`,
   liste (`6 / rw-`), edite para `7`, liste (`7 / rwx`) e apague.
   Tente criar com `8`: a pergunta deve se repetir até um valor válido.
8. No diretório, tente apagar um nome inexistente e depois o nome do arquivo real.
   Confira o diretório vazio. Apague o diretório, usuário e grupo pelos respectivos menus.
   Crie e apague também um arquivo pelo menu de arquivos.
9. Tente editar ou apagar em listas vazias e selecionar itens `0`, `999`
   ou `abc` em listas preenchidas. Deve haver mensagem de erro, sem panic.
10. Teste tamanho negativo, texto e um número maior que `18446744073709551615`;
    depois informe `0`. Termine com **0 — Sair** no menu principal.

## Revisão e testes detalhados

Consulte [REVISAO.md](REVISAO.md) para os problemas encontrados, as correções aplicadas, os resultados e as entradas exatas do roteiro manual.

A [validação final](VALIDACAO_FINAL.md) contém a sessão completa como usuário, a repetição com erros intencionais e a tabela de conformidade de todos os requisitos. As transcrições integrais estão em [sessão normal](SESSAO_NORMAL.txt) e [sessão com erros](SESSAO_ERROS.txt).
