# Revisão do projeto Rust

A revisão foi feita em duas fases: leitura e reprodução dos problemas na versão
`dd91181`; depois correção e execução dos testes na versão corrigida.
Toda a implementação continua em [src/main.rs](src/main.rs), sem dependências externas.
A primeira linha da dupla, as structs, os campos e os métodos especificados foram preservados.

## 1. Análise e correções aplicadas

### A. Menu com mensagem de erro fora do contexto

- **Onde estava:** `main`, `menu_*` e `editar_*`, no trecho
  `match ler_u16("Opção:")`.
- **Problema reproduzido:** digitar `abc` no menu principal mostrava
  `Número inválido. Digite de 0 a 65535.`, embora só existissem opções de 0 a 5.
- **Correção aplicada:** `ler_opcao(maximo)` converte e valida o intervalo do menu.
  Erros de formato, campos vazios, negativos, overflow e opções inexistentes
  recebem a mesma mensagem contextual.
- **Exemplo corrigido:** `match ler_opcao(5)`; erro:
  `Opção inválida. Digite um número de 0 a 5.`

| Contexto | Intervalo real e agora informado |
| --- | --- |
| Menu principal | 0 a 5 |
| Arquivos, diretórios, usuários, grupos e permissão avulsa | 0 a 4 |
| Edição de arquivo, usuário e grupo | 0 a 5 |
| Edição de diretório | 0 a 7 |

### B. Permissão misturava dois limites diferentes

- **Onde estava:** `ler_permissao`, trecho `let valor = ler_u16(...)`.
- **Problema reproduzido:** `8` produzia erro de 0 a 7, mas `-1`, letras e
  valores maiores que u16 produziam erro de 0 a 65535.
- **Correção aplicada:** leitura direta com `parse::<u8>()` e
  `Ok(valor) if valor <= 7`; qualquer entrada inválida agora informa
  `Permissão inválida. Digite um número de 0 a 7.`
- **Observação:** u8 comporta 0 a 255, mas o domínio desta pergunta é 0 a 7.
  O cálculo octal continua somando 4, 2 e 1.

### C. Nomes duplicados tinham tratamento inconsistente

- **Onde estava:** `criar_arquivo`, criação em `menu_diretorios`,
  `menu_usuarios`, `menu_grupos` e ramo `1 => ...nome = ler_texto(...)`
  nas funções de edição.
- **Problema reproduzido:** era possível criar dois arquivos chamados `teste`.
  Também era possível renomear um arquivo interno para o nome de outro.
  Entretanto, mover um arquivo com nome repetido para um diretório era recusado.
  A remoção por nome ficava ambígua.
- **Correção aplicada:** funções `nome_*_repetido` verificam a lista antes de criar
  ou renomear. Na edição, `Some(indice)` ignora o próprio item: manter o nome
  atual é permitido; usar o nome de outro é recusado.
- **Escopo:** nomes exatos após `trim()`, distintos dentro de cada lista.
  Um arquivo pode ter o mesmo nome de outro em diretório diferente.
  `Ana` e `ana` são nomes diferentes. Não foi criada uma regra nova de unicidade
  de uid ou gid, que não constava na especificação.
- **Em caso de erro:** o item anterior é preservado. Na criação, o submenu é
  reapresentado sem pedir tamanho ou IDs de um cadastro que já foi recusado.

### D. Mensagens repetidas em listas vazias

- **Onde estava:** `listar_arquivos` (e demais listagens), seguido de
  `ler_indice`, que também imprimia `Nenhum item cadastrado.`.
- **Problema reproduzido:** tentar editar uma lista vazia imprimia duas mensagens
  para a mesma condição.
- **Correção aplicada:** a listagem informa o tipo de item ausente;
  `ler_indice(0)` apenas devolve `None`, sem perguntar um número.
- **Correção relacionada:** ao apagar arquivo de um diretório vazio,
  `editar_diretorio` informa `Diretório vazio.` sem pedir um nome inútil.

### E. Seleção de item não informava o limite disponível

- **Onde estava:** `ler_indice`, prompt
  `Número do item (a partir de 1):` e erro
  `Item não encontrado ou número inválido.`.
- **Problema:** faltava indicar o maior índice disponível; o erro misturava
  conversão inválida com uma busca por nome, que não acontece nesse ponto.
- **Correção aplicada:** informa `Escolha um item de 1 a N.`;
  uma seleção recusada informa
  `Item inválido. Escolha um número de 1 a N.`
- **Segurança preservada:** `numero - 1` só é calculado depois de
  `numero >= 1 && numero <= quantidade`.

### F. Duas melhorias de clareza

Estas não eram falhas de conversão ou panic, mas deixavam o texto incompleto:

- **`ler_u64`:** o erro só dizia “inteiro sem sinal”. Agora informa
  `Tamanho inválido. Digite um inteiro de 0 a 18446744073709551615.`
- **`menu_diretorios`:** a opção `3 - Apagar` agora diz
  `3 - Apagar diretório e seus arquivos`, descrevendo o efeito que já existia.

## 2. Pontos conferidos que estavam corretos

- Cada opção exibida existe e chama a função correspondente.
- Prompts de uid e gid correspondem aos campos e tipos lidos.
- `u16` aceita de 0 a 65535; `u64`, de 0 a 18446744073709551615.
- IDs de grupo repetidos no mesmo usuário e membros repetidos no mesmo grupo
  já eram recusados. As remoções só anunciam sucesso quando encontram o ID.
- `remove_arquivo` só anuncia sucesso quando encontra o nome; a mensagem de
  arquivo não encontrado foi conferida.
- `Arquivo::new` produz `664 / rw-rw-r--`.
- `Permissoes:` sem acento foi mantido porque o enunciado exige esse formato exato.
  As demais mensagens em português mantêm os acentos.
- Não há `unwrap()`, `expect()`, divisão, crates externas, traits próprias,
  macros próprias nem lifetimes explícitas.
- Nos três métodos de remoção por busca, o índice vem de `0..vetor.len()`.
  Após `remove`, há `return` imediato: o laço não continua com índices antigos.
- Nas funções de edição, o índice vem de `ler_indice`; a lista não muda de
  tamanho durante a edição.
- Ao mover arquivos, os índices do diretório e do arquivo são validados nos
  respectivos vetores. A remoção acontece somente depois da verificação de nome.
- A soma em `octal` não ultrapassa 7, portanto não estoura u8.

## 3. Testes executados

Compilação com `cargo build --offline`, execução com `cargo run --offline`,
formatação com `rustfmt --check` e revisão de espaços com `git diff --check`.

**58 cenários passaram.** Um deles percorre as **512 tuplas possíveis**
de permissões de dono, grupo e outros pelo menu real.

Cobertura:

- Enter vazio, espaços, letras, símbolos, negativo, decimal, 70000 e inteiro
  muito maior que os tipos comportam, nos dez contextos de menu.
- Opção imediatamente acima do máximo de cada menu.
- Listar, editar e apagar as quatro listas vazias.
- Nome vazio, só espaços, repetido e renomeação para nome ocupado;
  renomeação para o próprio nome.
- Limites válidos 0 e 65535 de u16; 0 e 18446744073709551615 de u64;
  valores acima desses limites.
- Índices vazios, espaços, letras, símbolos, negativos, decimais, zero,
  maior que a lista e maior que usize.
- Edição de todos os campos das estruturas.
- Adição repetida de gid e uid; remoção existente e inexistente.
- Arquivo ausente no diretório; diretório vazio; edição interna; movimentação
  com colisão de nome, verificando a preservação do arquivo na origem.
- CRUD de permissão avulsa e as oito combinações de booleanos.
- Fim de entrada no menu, no nome, na permissão e na seleção de índice.

| Leitura | Escrita | Execução | Octal | Texto |
| --- | --- | --- | --- | --- |
| false | false | false | 0 | `---` |
| false | false | true | 1 | `--x` |
| false | true | false | 2 | `-w-` |
| false | true | true | 3 | `-wx` |
| true | false | false | 4 | `r--` |
| true | false | true | 5 | `r-x` |
| true | true | false | 6 | `rw-` |
| true | true | true | 7 | `rwx` |

## 4. Roteiro manual com entradas exatas

Abra o terminal na pasta do projeto e execute `cargo run` **de novo em cada
caso abaixo**, para começar com listas vazias.
Cada elemento separado por `→` é uma linha digitada, seguida de Enter.
`[Enter]` significa apenas Enter; `[3 espaços]` significa digitar três espaços e Enter.
Essas duas marcações não devem ser digitadas literalmente.
Os resultados indicados são as mensagens e dados relevantes; os menus também reaparecem.

### A. Entradas ruins no menu principal

**Digite:**

`[Enter] → [3 espaços] → abc → @ → -1 → 1.5 → 70000 → 6 → 0`

**Deve aparecer:** depois de cada uma das oito primeiras entradas,
`Opção inválida. Digite um número de 0 a 5.`
Depois de `0`, `Programa encerrado.`

### B. Submenu vazio e intervalo correto

**Digite:**

`1 → abc → 5 → 4 → 2 → 3 → 0 → 0`

**Deve aparecer:**

- Após `abc` e `5`: `Opção inválida. Digite um número de 0 a 4.`
- Após cada `4`, `2` e `3`: uma única mensagem
  `Nenhum arquivo cadastrado.`, sem pedir índice.
- Os dois zeros voltam ao principal e encerram.

Repita a sequência trocando o primeiro `1` por `2`, `3` e `4`.
As mensagens passam a ser `Nenhum diretório cadastrado.`,
`Nenhum usuário cadastrado.` e `Nenhum grupo cadastrado.`, respectivamente.

### C. Nome vazio e duplicado

**Digite:**

`1 → 1 → [Enter] → [3 espaços] → teste → 0 → 1000 → 1000 → 1 → teste → 4 → 0 → 0`

**Deve aparecer:**

- Após os nomes vazios: `Nome inválido. Digite um nome não vazio.`
- Após o segundo `teste`: `Já existe um arquivo com esse nome nesta lista.`
  O programa volta ao submenu, sem pedir tamanho e IDs novamente.
- Na listagem, apenas um arquivo:

```text
Arquivo: teste
Tamanho: 0 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1000
Gid: 1000
```

### D. Limites dos tipos

**Digite:**

`1 → 1 → limite → -1 → 1.5 → 18446744073709551616 → 18446744073709551615 → 70000 → -1 → 65535 → 0 → 4 → 0 → 0`

**Deve aparecer:**

- Para os três tamanhos inválidos:
  `Tamanho inválido. Digite um inteiro de 0 a 18446744073709551615.`
- Para os dois uids inválidos:
  `Número inválido. Digite de 0 a 65535.`
- Na listagem: tamanho `18446744073709551615 bytes`, uid `65535`, gid `0`.

### E. Índice inválido e arquivo preservado

**Digite:**

`1 → 1 → teste → 0 → 1 → 1 → 2 → 999 → 3 → 0 → 2 → abc → 4 → 0 → 0`

**Deve aparecer:** cada seleção de índice `999`, `0` e `abc` recebe
`Item inválido. Escolha um número de 1 a 1.`
A listagem ainda mostra o arquivo `teste`; não aparece `Arquivo apagado.`

### F. Edição completa e exclusão de arquivo

**Digite:**

`1 → 1 → teste → 0 → 1000 → 1000 → 2 → 1 → 1 → editado → 2 → 123 → 3 → 7 → 5 → 0 → 4 → 2000 → 5 → 3000 → 0 → 4 → 3 → 1 → 4 → 0 → 0`

**Deve aparecer na listagem:**

```text
Arquivo: editado
Tamanho: 123 bytes
Permissoes: (750 / rwxr-x---)
Uid: 2000
Gid: 3000
```

Depois da exclusão: `Arquivo apagado.`; na listagem final:
`Nenhum arquivo cadastrado.`

### G. Renomeação com colisão

**Digite:**

`1 → 1 → a → 0 → 1 → 1 → 1 → b → 0 → 1 → 1 → 2 → 2 → 1 → a → 1 → b → 0 → 4 → 0 → 0`

**Deve aparecer:** ao tentar trocar `b` por `a`,
`Já existe um arquivo com esse nome nesta lista.`
Manter o nome `b` em seguida é aceito. A listagem contém `a` e `b`.

### H. Usuário: gid duplicado e remoção inexistente

**Digite:**

`3 → 1 → Ana → 1000 → 2 → 1 → 3 → 100 → 3 → 100 → 5 → 4 → 999 → 4 → 100 → 5 → 0 → 3 → 1 → 4 → 0 → 0`

**Deve aparecer:**

- Na segunda adição: `Grupo já cadastrado neste usuário.`
- Na primeira listagem de grupos: `Gid: 100` uma única vez.
- Remover `999`: `Grupo não encontrado neste usuário.`
- Remover `100`: `Grupo removido do usuário.`; depois, `Sem grupos.`
- Ao excluir: `Usuário apagado.`; depois, `Nenhum usuário cadastrado.`

### I. Grupo: uid duplicado e remoção inexistente

**Digite:**

`4 → 1 → alunos → 100 → 2 → 1 → 3 → 1000 → 3 → 1000 → 5 → 4 → 999 → 4 → 1000 → 5 → 0 → 3 → 1 → 4 → 0 → 0`

**Deve aparecer:**

- `Membro já cadastrado neste grupo.`
- `Uid: 1000` uma única vez na primeira listagem de membros.
- `Membro não encontrado neste grupo.`
- `Membro removido do grupo.` e depois `Sem membros.`
- `Grupo apagado.` e depois `Nenhum grupo cadastrado.`

### J. Diretório, move, arquivo ausente e exclusão

**Digite:**

`1 → 1 → teste → 0 → 1000 → 1000 → 0 → 2 → 1 → docs → 7 → 5 → 5 → 1000 → 2 → 1 → 4 → 1 → 7 → 5 → ausente → 5 → teste → 7 → 5 → 0 → 3 → 1 → 4 → 0 → 1 → 4 → 0 → 0`

**Deve aparecer:**

- `Arquivo movido para o diretório.`
- Ao listar conteúdo: `Arquivo: teste` com permissões padrão.
- Para `ausente`: `Arquivo não encontrado.`
- Para `teste`: `Arquivo apagado do diretório.`
- Ao listar ou tentar apagar novamente: `Diretório vazio.`, sem pedir nome.
- Depois: `Diretório e seus arquivos apagados.`,
  `Nenhum diretório cadastrado.` e `Nenhum arquivo cadastrado.`

### K. As oito permissões e todos os erros no mesmo contexto

**Digite:**

`5 → 4 → 1 → [Enter] → [3 espaços] → abc → @ → -1 → 1.5 → 70000 → 8 → 0 → 4 → 2 → 1 → 4 → 2 → 2 → 4 → 2 → 3 → 4 → 2 → 4 → 4 → 2 → 5 → 4 → 2 → 6 → 4 → 2 → 7 → 4 → 3 → 4 → 0 → 0`

**Deve aparecer:**

- Antes da criação: `Permissão não encontrada.`
- Para cada uma das oito entradas inválidas:
  `Permissão inválida. Digite um número de 0 a 7.`
- Cada listagem deve mostrar, na ordem:

```text
Permissão: 0 / ---
Permissão: 1 / --x
Permissão: 2 / -w-
Permissão: 3 / -wx
Permissão: 4 / r--
Permissão: 5 / r-x
Permissão: 6 / rw-
Permissão: 7 / rwx
```

- Depois de apagar: `Permissão apagada.`; na listagem:
  `Permissão não encontrada.`

### L. Fim de entrada

No PowerShell, execute:

```powershell
"" | cargo run
```

**Deve aparecer:** uma entrada vazia é recusada, seguida de
`Entrada encerrada.` quando o fluxo termina. O programa não fica repetindo o menu.

## 5. Lista das alterações

- Separada a leitura da linha da validação de nomes e números.
- Adicionada validação contextual das opções de todos os menus.
- Uniformizada a mensagem de permissão inválida para o intervalo de 0 a 7.
- Informado o limite exato de u64 no erro de tamanho.
- Informado o intervalo disponível para seleção de item.
- Removida a segunda mensagem desnecessária nas listas vazias.
- Recusados nomes duplicados na criação dentro da mesma lista.
- Recusadas renomeações para nomes de outros itens, inclusive dentro de diretórios.
- Mantida a possibilidade de confirmar o próprio nome sem erro de duplicidade.
- Reutilizada a validação de nomes na movimentação de arquivos.
- Evitada pergunta de nome ao apagar conteúdo de diretório vazio.
- Explicitado no menu que apagar um diretório apaga também seus arquivos.
- Atualizado o README para refletir as validações corrigidas.

## 6. Limites honestos da revisão

Nenhuma entrada testada causou panic ou acesso fora do vetor.
Isso não é uma prova de ausência de falhas em qualquer ambiente.

Não simulei falta de memória, falhas de hardware ou indisponibilidade da saída
do terminal. Uma linha ou quantidade de cadastros suficientemente grande pode
esgotar a memória; `println!` pode causar panic se a saída padrão falhar.
Erros de leitura de stdin são tratados com mensagem e encerramento, mas não
injetei uma falha física de leitura para validar esse ramo.

As funções internas de edição pressupõem o índice validado pelos menus.
Chamadas futuras feitas diretamente por outro código com índice inválido poderiam
panicar; não há esse caminho na execução atual com entrada do usuário.

IDs de usuários e grupos continuam independentes e podem se repetir nos cadastros;
não há sincronização automática de associações nem persistência em disco.
Esses comportamentos já existiam e não foram ampliados nesta correção.
