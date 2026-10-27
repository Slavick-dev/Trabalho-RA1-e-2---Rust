# Validação final — Yago Landin e Gean Karlo

Validação realizada em 27/10/2026, na ordem solicitada:
primeiro execução como usuário; depois conferência como professor.

O [main.rs completo](src/main.rs) contém toda a implementação.
A única correção necessária no código nesta rodada foi substituir os marcadores
da dupla pelos nomes informados. Os testes não exigiram mudanças de comportamento.

## Papel 1: usuário real

Foram executadas duas sessões com o executável compilado, enviando cada entrada
após o programa apresentar seu pedido e capturando a saída real.

As transcrições completas estão em [SESSAO_NORMAL.txt](SESSAO_NORMAL.txt) e
[SESSAO_ERROS.txt](SESSAO_ERROS.txt). Os marcadores `[DIGITADO]` e os títulos
`=== etapa ===` foram inseridos pelo registrador para distinguir as entradas;
o restante corresponde às linhas efetivamente emitidas pelo programa.
Não são telas inventadas.

### Sessão normal, passo a passo

Execute `cargo run` na pasta do projeto.
O primeiro menu exibido é:

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

Na tabela, cada elemento separado por `→` é uma entrada seguida de Enter.
As etapas pertencem à **mesma execução**: continue na posição deixada pela anterior.
Os resultados são os trechos relevantes da saída; a transcrição integral inclui
todos os prompts e menus repetidos.

| Etapa | O que digitar, em ordem | O que aparece na tela |
| --- | --- | --- |
| 1. Criar usuário | `3 → 1 → Ana → 1000 → 0` | `Nome do usuário:`<br>`Uid:`<br>`MENU PRINCIPAL` |
| 2. Criar grupo | `4 → 1 → alunos → 1000 → 0` | `Nome do grupo:`<br>`Gid:`<br>`MENU PRINCIPAL` |
| 3. Criar diretório | `2 → 1 → docs → 7 → 5 → 5 → 1000 → 0` | `Nome do diretório:`<br>`Uid do dono:`<br>`MENU PRINCIPAL` |
| 4. Criar arquivo | `1 → 1 → teste → 0 → 1000 → 1000 → 0` | `Nome do arquivo:`<br>`Tamanho em bytes:`<br>`Gid do grupo:` |
| 5. Adicionar grupo ao usuário e listar usuário | `3 → 2 → 1 → 3 → 1000 → 0 → 4 → 0` | `Usuário: Ana | Uid: 1000`<br>`Gid: 1000` |
| 6. Adicionar usuário ao grupo e listar grupo | `4 → 2 → 1 → 3 → 1000 → 0 → 4 → 0` | `Grupo: alunos | Gid: 1000`<br>`Uid: 1000` |
| 7. Mover arquivo para diretório e listar conteúdo (stat) | `2 → 2 → 1 → 4 → 1 → 7` | `Arquivo movido para o diretório.`<br>`Arquivo: teste`<br>`Tamanho: 0 bytes`<br>`Permissoes: (664 / rw-rw-r--)`<br>`Uid: 1000`<br>`Gid: 1000` |
| 8. Alterar permissão do arquivo e listar conteúdo (stat) novamente | `6 → 1 → 3 → 7 → 5 → 0 → 0 → 7` | `Permissoes: (750 / rwxr-x---)` |
| 9. Remover arquivo do diretório e confirmar vazio | `5 → teste → 7 → 0 → 0` | `Arquivo apagado do diretório.`<br>`Diretório vazio.` |
| 10. Remover grupo do usuário e listar | `3 → 2 → 1 → 4 → 1000 → 5 → 0 → 0` | `Grupo removido do usuário.`<br>`Sem grupos.` |
| 11. Remover membro do grupo e listar | `4 → 2 → 1 → 4 → 1000 → 5 → 0 → 0` | `Membro removido do grupo.`<br>`Sem membros.` |
| 12. Apagar usuário e listar | `3 → 3 → 1 → 4 → 0` | `Usuário apagado.`<br>`Nenhum usuário cadastrado.` |
| 13. Apagar grupo e listar | `4 → 3 → 1 → 4 → 0` | `Grupo apagado.`<br>`Nenhum grupo cadastrado.` |
| 14. Apagar diretório e listar | `2 → 3 → 1 → 4 → 0` | `Diretório e seus arquivos apagados.`<br>`Nenhum diretório cadastrado.` |
| 15. Confirmar que o arquivo também não está na lista principal | `1 → 4 → 0` | `Nenhum arquivo cadastrado.` |
| 16. Demonstrar também a exclusão pelo menu Arquivos | `1 → 1 → temporario → 0 → 1000 → 1000 → 3 → 1 → 4 → 0` | `Arquivo apagado.`<br>`Nenhum arquivo cadastrado.` |
| 17. Criar, editar, listar e apagar permissão avulsa | `5 → 1 → 6 → 4 → 2 → 7 → 4 → 3 → 4 → 0` | `Permissão: 6 / rw-`<br>`Permissão: 7 / rwx`<br>`Permissão apagada.`<br>`Permissão não encontrada.` |
| 18. Sair pela opção 0 | `0` | `Programa encerrado.` |

O primeiro stat do arquivo, antes de trocar permissões, é exatamente:

```text
Arquivo: teste
Tamanho: 0 bytes
Permissoes: (664 / rw-rw-r--)
Uid: 1000
Gid: 1000
```

Depois da alteração, a listagem chama stat novamente e imprime:

```text
Arquivo: teste
Tamanho: 0 bytes
Permissoes: (750 / rwxr-x---)
Uid: 1000
Gid: 1000
```

**Sobre apagar o arquivo:** remover do diretório já destrói o arquivo, porque
`remove_arquivo` não devolve um `Arquivo`. Ele não reaparece na lista principal.
Na etapa 16 foi criado um segundo arquivo, `temporario`, apenas para demonstrar
também a opção Apagar do menu Arquivos. Não foi adicionada funcionalidade ao programa.

### Avaliação da experiência de uso

- Os menus têm opções numeradas e `0 - Voltar`; apenas no principal o zero encerra.
- Os pedidos distinguem nome, tamanho, uid, gid e posição do item.
- Nas permissões, o programa pede um dígito de 0 a 7 para dono, grupo e outros,
  explicando os pesos 4, 2 e 1.
- Para mover um arquivo ao diretório, primeiro é preciso criá-lo em Arquivos.
  O texto `Mover arquivo para cá` descreve a transferência de posse.
- A operação stat não tem uma opção separada com esse nome: listar arquivos ou
  conteúdo de diretório chama o método e exibe os dados.
- Criar e adicionar associações não imprimem mensagem explícita de sucesso;
  o menu reaparece, e a listagem confirma o resultado. Isso é uma limitação de
  feedback, sem impedir o uso ou contrariar os requisitos.
- Adicionar um grupo ao usuário e um membro ao grupo são duas operações distintas.
  Não há sincronização automática implícita.
- Não houve travamento, repetição infinita, stderr, panic ou saída inesperada.
  A sessão terminou com `Programa encerrado.` após digitar a opção 0,
  e o processo retornou código 0.

### Sessão repetida com erros intencionais

Comece outra execução com `cargo run`.
`[Enter]` significa apertar somente Enter; `[3 espaços]` significa digitar
três espaços e Enter. Não digite literalmente essas marcações.

| Etapa | O que digitar, em ordem | O que aparece na tela |
| --- | --- | --- |
| 1. Entradas inválidas no principal | `abc → [Enter] → [3 espaços] → @ → -1 → 1.5 → 70000 → 9` | `Opção inválida. Digite um número de 0 a 5.` |
| 2. Criar usuário após erros de menu, nome e uid | `3 → abc → 5 → 2 → 1 → [Enter] → [3 espaços] → Ana → abc → -1 → 70000 → 1000 → 0` | `Opção inválida. Digite um número de 0 a 4.`<br>`Nenhum usuário cadastrado.`<br>`Nome inválido. Digite um nome não vazio.`<br>`Número inválido. Digite de 0 a 65535.`<br>`MENU PRINCIPAL` |
| 3. Criar grupo após nome e gid inválidos | `4 → 3 → 1 → [Enter] → alunos → 70000 → 1000 → 0` | `Nenhum grupo cadastrado.`<br>`Nome inválido.`<br>`Número inválido.`<br>`MENU PRINCIPAL` |
| 4. Criar diretório após permissão inválida | `2 → 1 → docs → 8 → abc → [Enter] → -1 → 70000 → 7 → 5 → 5 → 1000 → 0` | `Permissão inválida. Digite um número de 0 a 7.`<br>`MENU PRINCIPAL` |
| 5. Criar arquivo após tamanho inválido e rejeitar duplicado | `1 → 1 → teste → abc → [Enter] → -1 → 1.5 → 18446744073709551616 → 0 → 1000 → 1000 → 1 → teste → 0` | `Tamanho inválido. Digite um inteiro de 0 a 18446744073709551615.`<br>`Já existe um arquivo com esse nome nesta lista.`<br>`MENU PRINCIPAL` |
| 6. Índices inválidos e gid repetido no usuário | `3 → 2 → 999 → 2 → abc → 2 → 0 → 2 → 1 → 3 → 1000 → 3 → 1000 → 0 → 4 → 0` | `Item inválido. Escolha um número de 1 a 1.`<br>`Grupo já cadastrado neste usuário.`<br>`Gid: 1000` |
| 7. Membro repetido no grupo | `4 → 2 → 1 → 3 → 1000 → 3 → 1000 → 0 → 4 → 0` | `Membro já cadastrado neste grupo.`<br>`Uid: 1000` |
| 8. Mover após índice inválido e listar stat | `2 → 2 → 1 → 4 → 999 → 4 → 1 → 7` | `Item inválido. Escolha um número de 1 a 1.`<br>`Arquivo movido para o diretório.`<br>`Permissoes: (664 / rw-rw-r--)` |
| 9. Alterar permissão após erros e listar stat | `6 → 999 → 6 → 1 → 9 → 3 → 8 → abc → [Enter] → 70000 → 7 → 5 → 0 → 0 → 7` | `Item inválido. Escolha um número de 1 a 1.`<br>`Opção inválida. Digite um número de 0 a 5.`<br>`Permissão inválida. Digite um número de 0 a 7.`<br>`Permissoes: (750 / rwxr-x---)` |
| 10. Remover arquivo ausente, depois remover o correto | `5 → ausente → 5 → teste → 7 → 5 → 0 → 0` | `Arquivo não encontrado.`<br>`Arquivo apagado do diretório.`<br>`Diretório vazio.` |
| 11. Remover gid ausente, depois correto | `3 → 2 → 1 → 4 → 999 → 4 → 1000 → 5 → 0 → 0` | `Grupo não encontrado neste usuário.`<br>`Grupo removido do usuário.`<br>`Sem grupos.` |
| 12. Remover membro ausente, depois correto | `4 → 2 → 1 → 4 → 999 → 4 → 1000 → 5 → 0 → 0` | `Membro não encontrado neste grupo.`<br>`Membro removido do grupo.`<br>`Sem membros.` |
| 13. Apagar usuário após índice inválido e tentar apagar vazio | `3 → 3 → 999 → 3 → 1 → 4 → 3 → 0` | `Item inválido. Escolha um número de 1 a 1.`<br>`Usuário apagado.`<br>`Nenhum usuário cadastrado.` |
| 14. Apagar grupo após índice inválido | `4 → 3 → abc → 3 → 1 → 4 → 0` | `Item inválido. Escolha um número de 1 a 1.`<br>`Grupo apagado.`<br>`Nenhum grupo cadastrado.` |
| 15. Apagar diretório após índice inválido | `2 → 3 → 0 → 3 → 1 → 4 → 0` | `Item inválido. Escolha um número de 1 a 1.`<br>`Diretório e seus arquivos apagados.`<br>`Nenhum diretório cadastrado.` |
| 16. Apagar arquivo pelo menu após erro e sair normalmente | `1 → 3 → 1 → temporario → 0 → 1000 → 1000 → 3 → 999 → 3 → 1 → 4 → 0 → 0` | `Item inválido. Escolha um número de 1 a 1.`<br>`Arquivo apagado.`<br>`Nenhum arquivo cadastrado.`<br>`Programa encerrado.` |

Todas as etapas foram concluídas. Depois dos erros, os cadastros, associações,
movimentação, stat, permissões e exclusões continuaram funcionando.
O processo saiu pela opção 0, com código 0 e sem saída de erro.

## Papel 2: professor corrigindo

A tabela usa os requisitos e as assinaturas **transcritos na conversa**.
Não foi fornecido o documento original da Tabela 2; as interpretações que dependem
dele estão explicitadas ao final. “Sim” significa conformidade com os requisitos
fornecidos e com essas interpretações, sem afirmar comparação com um documento ausente.

Foram também compilados quatro testes de conferência, fora do projeto entregue:
os 17 métodos foram atribuídos a ponteiros de função com os tipos esperados;
os 18 campos foram conferidos por atribuições tipadas; e os construtores,
permissões e operações de vetores foram verificados. Os quatro testes passaram.
A implementação do projeto continua somente em src/main.rs.

| Requisito | Atende (sim/não/parcial) | Trecho do código que prova | Correção necessária |
| --- | --- | --- | --- |
| Implementado em Rust, com métodos em blocos impl | sim | `impl Permissao; impl Arquivo; impl Diretorio; impl Usuario; impl Grupo` ([código](src/main.rs#L12)) | Nenhuma. |
| Menu permite criar, editar, apagar e listar Arquivo | sim | `fn menu_arquivos(...)` ([código](src/main.rs#L438)) | Nenhuma. |
| Menu permite criar, editar, apagar e listar Diretorio | sim | `fn menu_diretorios(...)` ([código](src/main.rs#L537)) | Nenhuma. |
| Menu permite criar, editar, apagar e listar Usuario | sim | `fn menu_usuarios(...)` ([código](src/main.rs#L612)) | Nenhuma. |
| Menu permite criar, editar, apagar e listar Grupo | sim | `fn menu_grupos(...)` ([código](src/main.rs#L686)) | Nenhuma. |
| Menu permite criar, editar, apagar e listar Permissao | sim | `fn menu_permissao(...)` ([código](src/main.rs#L720)) | Nenhuma. |
| Borrowing imutável para leitura | sim | `fn stat(&self); fn listar_conteudo(&self)` ([código](src/main.rs#L69)) | Nenhuma. |
| Borrowing mutável para alterações | sim | `fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao))` ([código](src/main.rs#L64)) | Nenhuma. |
| Move ao entregar um arquivo ao diretório | sim | `fn adiciona_arquivo(&mut self, arquivo: Arquivo)` ([código](src/main.rs#L103)) | Nenhuma. |
| Nome real da dupla no comentário do código | sim | `// Dupla: Yago Landin e Gean Karlo` ([código](src/main.rs#L1)) | Aplicada: substituídos os dois marcadores pelos nomes informados. |
| Arquivo: campo nome tem o tipo exato | sim | `nome: String` ([código](src/main.rs#L39)) | Nenhuma. |
| Arquivo: campo tamanho tem o tipo exato | sim | `tamanho: u64` ([código](src/main.rs#L40)) | Nenhuma. |
| Arquivo: campo permissoes tem o tipo exato | sim | `permissoes: (Permissao, Permissao, Permissao)` ([código](src/main.rs#L41)) | Nenhuma. |
| Arquivo: campo uid_dono tem o tipo exato | sim | `uid_dono: u16` ([código](src/main.rs#L42)) | Nenhuma. |
| Arquivo: campo gid_grupo tem o tipo exato | sim | `gid_grupo: u16` ([código](src/main.rs#L43)) | Nenhuma. |
| Permissao: campo leitura tem o tipo exato | sim | `leitura: bool` ([código](src/main.rs#L6)) | Nenhuma. |
| Permissao: campo escrita tem o tipo exato | sim | `escrita: bool` ([código](src/main.rs#L7)) | Nenhuma. |
| Permissao: campo execucao tem o tipo exato | sim | `execucao: bool` ([código](src/main.rs#L8)) | Nenhuma. |
| Diretorio: campo nome tem o tipo exato | sim | `nome: String` ([código](src/main.rs#L80)) | Nenhuma. |
| Diretorio: campo arquivos tem o tipo exato | sim | `arquivos: Vec<Arquivo>` ([código](src/main.rs#L81)) | Nenhuma. |
| Diretorio: campo permissoes tem o tipo exato | sim | `permissoes: (Permissao, Permissao, Permissao)` ([código](src/main.rs#L82)) | Nenhuma. |
| Diretorio: campo uid_dono tem o tipo exato | sim | `uid_dono: u16` ([código](src/main.rs#L83)) | Nenhuma. |
| Usuario: campo nome tem o tipo exato | sim | `nome: String` ([código](src/main.rs#L132)) | Nenhuma. |
| Usuario: campo uid tem o tipo exato | sim | `uid: u16` ([código](src/main.rs#L133)) | Nenhuma. |
| Usuario: campo grupos_ids tem o tipo exato | sim | `grupos_ids: Vec<u16>` ([código](src/main.rs#L134)) | Nenhuma. |
| Grupo: campo nome tem o tipo exato | sim | `nome: String` ([código](src/main.rs#L182)) | Nenhuma. |
| Grupo: campo gid tem o tipo exato | sim | `gid: u16` ([código](src/main.rs#L183)) | Nenhuma. |
| Grupo: campo membros_uids tem o tipo exato | sim | `membros_uids: Vec<u16>` ([código](src/main.rs#L184)) | Nenhuma. |
| Arquivo::new: nome, parâmetros, retorno e autorreferência | sim | `fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo` ([código](src/main.rs#L49)) | Nenhuma. |
| Arquivo::alterar_permissao: nome, parâmetros, retorno e autorreferência | sim | `fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao))` ([código](src/main.rs#L64)) | Nenhuma. |
| Arquivo::stat: nome, parâmetros, retorno e autorreferência | sim | `fn stat(&self)` ([código](src/main.rs#L69)) | Nenhuma. |
| Permissao::new: nome, parâmetros, retorno e autorreferência | sim | `fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao` ([código](src/main.rs#L13)) | Nenhuma. |
| Permissao::octal: nome, parâmetros, retorno e autorreferência | sim | `fn octal(&self) -> u8` ([código](src/main.rs#L22)) | Nenhuma. |
| Diretorio::new: nome, parâmetros, retorno e autorreferência | sim | `fn new( nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio` ([código](src/main.rs#L89)) | Nenhuma. |
| Diretorio::adiciona_arquivo: nome, parâmetros, retorno e autorreferência | sim | `fn adiciona_arquivo(&mut self, arquivo: Arquivo)` ([código](src/main.rs#L103)) | Nenhuma. |
| Diretorio::remove_arquivo: nome, parâmetros, retorno e autorreferência | sim | `fn remove_arquivo(&mut self, nome_arquivo: &str)` ([código](src/main.rs#L108)) | Nenhuma. |
| Diretorio::listar_conteudo: nome, parâmetros, retorno e autorreferência | sim | `fn listar_conteudo(&self)` ([código](src/main.rs#L120)) | Nenhuma. |
| Usuario::new: nome, parâmetros, retorno e autorreferência | sim | `fn new(nome: String, uid: u16) -> Usuario` ([código](src/main.rs#L140)) | Nenhuma. |
| Usuario::adiciona_grupo: nome, parâmetros, retorno e autorreferência | sim | `fn adiciona_grupo(&mut self, gid: u16)` ([código](src/main.rs#L149)) | Nenhuma. |
| Usuario::remove_grupo: nome, parâmetros, retorno e autorreferência | sim | `fn remove_grupo(&mut self, gid: u16)` ([código](src/main.rs#L158)) | Nenhuma. |
| Usuario::listar_grupos: nome, parâmetros, retorno e autorreferência | sim | `fn listar_grupos(&self)` ([código](src/main.rs#L170)) | Nenhuma. |
| Grupo::new: nome, parâmetros, retorno e autorreferência | sim | `fn new(nome: String, gid: u16) -> Grupo` ([código](src/main.rs#L190)) | Nenhuma. |
| Grupo::adiciona_membro: nome, parâmetros, retorno e autorreferência | sim | `fn adiciona_membro(&mut self, uid: u16)` ([código](src/main.rs#L199)) | Nenhuma. |
| Grupo::remove_membro: nome, parâmetros, retorno e autorreferência | sim | `fn remove_membro(&mut self, uid: u16)` ([código](src/main.rs#L208)) | Nenhuma. |
| Grupo::listar_membros: nome, parâmetros, retorno e autorreferência | sim | `fn listar_membros(&self)` ([código](src/main.rs#L220)) | Nenhuma. |
| Permissao::new somente atribui os parâmetros aos campos | sim | `Permissao { leitura, escrita, execucao }` ([código](src/main.rs#L14)) | Nenhuma. |
| octal usa leitura=4, escrita=2, execução=1 | sim | `valor += 4; valor += 2; valor += 1;` ([código](src/main.rs#L22)) | Nenhuma. |
| Arquivo::new: padrão rw- para dono e grupo, r-- para outros | sim | `Permissao::new(true, true, false), Permissao::new(true, true, false), Permissao::new(true, false, false)` ([código](src/main.rs#L53)) | Nenhuma. |
| Arquivo::new: demais campos vêm dos parâmetros | sim | `nome, tamanho, uid_dono: uid, gid_grupo: gid` ([código](src/main.rs#L49)) | Nenhuma. |
| stat imprime as cinco linhas no formato exato, consultando self | sim | `println!("Arquivo: {}", self.nome); mostrar_permissoes(&self.permissoes);` ([código](src/main.rs#L69)) | Nenhuma. |
| Diretorio::new começa com vetor vazio | sim | `arquivos: Vec::new()` ([código](src/main.rs#L96)) | Nenhuma. |
| Usuario::new começa com vetor vazio | sim | `grupos_ids: Vec::new()` ([código](src/main.rs#L144)) | Nenhuma. |
| Grupo::new começa com vetor vazio | sim | `membros_uids: Vec::new()` ([código](src/main.rs#L194)) | Nenhuma. |
| Diretorio::adiciona_arquivo usa vetor mutável | sim | `self.arquivos.push(arquivo);` ([código](src/main.rs#L104)) | Nenhuma. |
| Diretorio::remove_arquivo usa vetor mutável | sim | `self.arquivos.remove(indice);` ([código](src/main.rs#L111)) | Nenhuma. |
| Usuario::adiciona_grupo usa vetor mutável | sim | `self.grupos_ids.push(gid);` ([código](src/main.rs#L153)) | Nenhuma. |
| Usuario::remove_grupo usa vetor mutável | sim | `self.grupos_ids.remove(indice);` ([código](src/main.rs#L161)) | Nenhuma. |
| Grupo::adiciona_membro usa vetor mutável | sim | `self.membros_uids.push(uid);` ([código](src/main.rs#L203)) | Nenhuma. |
| Grupo::remove_membro usa vetor mutável | sim | `self.membros_uids.remove(indice);` ([código](src/main.rs#L211)) | Nenhuma. |
| Diretorio::listar_conteudo percorre o vetor com for | sim | `for arquivo in &self.arquivos {` ([código](src/main.rs#L124)) | Nenhuma. |
| Usuario::listar_grupos percorre o vetor com for | sim | `for gid in &self.grupos_ids {` ([código](src/main.rs#L174)) | Nenhuma. |
| Grupo::listar_membros percorre o vetor com for | sim | `for uid in &self.membros_uids {` ([código](src/main.rs#L224)) | Nenhuma. |
| Os quatro Vec do enunciado estão no main | sim | `Vec<Arquivo>; Vec<Diretorio>; Vec<Usuario>; Vec<Grupo>` ([código](src/main.rs#L760)) | Nenhuma. |
| Menu com loop, match e std::io::stdin().read_line | sim | `match io::stdin().read_line(&mut entrada)` ([código](src/main.rs#L234)) | Nenhuma. |
| Entrada numérica inválida tratada com match | sim | `match ler_entrada(mensagem).parse::<u16>()` ([código](src/main.rs#L261)) | Nenhuma. |
| Nenhum unwrap() ou expect() em entrada do usuário | sim | `match ...; if let Some(indice) = ler_indice(...)` ([código](src/main.rs#L289)) | Nenhuma. |
| Indexações dos menus só após conferir o tamanho | sim | `numero >= 1 && numero <= quantidade` ([código](src/main.rs#L295)) | Nenhuma. |
| Indexações nas buscas de remoção usam intervalo válido | sim | `for indice in 0..self.arquivos.len()` ([código](src/main.rs#L109)) | Nenhuma. |
| Após remove durante busca, a função retorna imediatamente | sim | `self.arquivos.remove(indice); println!(...); return;` ([código](src/main.rs#L111)) | Nenhuma. |
| Nenhum campo renomeado ou com tipo diferente | sim | `struct Arquivo; struct Permissao; struct Diretorio; struct Usuario; struct Grupo` ([código](src/main.rs#L5)) | Nenhuma. |
| Métodos correspondem às assinaturas transcritas nesta conversa | sim | `17 referências de função conferidas pelo compilador; assinaturas listadas acima` ([código](src/main.rs#L13)) | Nenhuma. |
| Código simples e concentrado em um main.rs, sem lifetimes explícitas | sim | `use std::io;` ([código](src/main.rs#L2)) | Nenhuma. |
| Comentários curtos antes de structs, impl e funções do menu | sim | `// Guarda as três ações permitidas para uma pessoa.` ([código](src/main.rs#L4)) | Nenhuma. |
| Somente biblioteca padrão, sem crates externas | sim | `use std::io;` e `[dependencies]` vazio em [Cargo.toml](Cargo.toml) | Nenhuma. |
| cargo build sem erros e sem warnings | sim | Executado: `cargo build --offline`, código de saída 0. | Nenhuma. |
| cargo clippy sem erros e sem warnings | sim | Executado: `cargo clippy --offline --all-targets -- -D warnings`, código de saída 0. | Nenhuma. |

Métodos sem `-> ...` explícito retornam `()`, o retorno unitário de Rust.
As funções auxiliares de entrada e de menu ficam fora de impl; os métodos e
construtores das cinco estruturas estão dentro dos respectivos blocos impl.

### Verificações executadas na versão final

- `cargo build --offline`: passou, sem erros e sem warnings.
- `cargo clippy --offline --all-targets -- -D warnings`: passou, sem erros e sem warnings.
- Sessão normal: 18 etapas, término por opção 0 e saída de processo 0.
- Sessão com erros: 16 etapas, término por opção 0 e saída de processo 0.
- Reexecução dos 58 cenários da revisão anterior: passaram, incluindo as
  512 combinações de permissões de dono, grupo e outros.
- Quatro testes de conferência de campos, assinaturas e comportamento: passaram.
- O formato de stat foi verificado como cinco linhas consecutivas, não apenas
  como palavras isoladas na saída.

### Interpretações que devem ser confirmadas se a Tabela 2 original disser algo diferente

1. **Tipo de nome_arquivo:** foi usado `&str`, pois o enunciado transcrito não fixa
   o tipo desse parâmetro e o método apenas compara o nome, sem guardá-lo.
   Se a tabela original exigir `String` ou `&String`, essa assinatura precisará mudar.
2. **Parâmetro nome nos construtores:** foi usado `String`, transferindo a posse
   do nome para o campo `nome: String`, conforme o objetivo de praticar move.
3. **Retorno das mutações:** adicionar, remover e alterar retornam `()`,
   pois o enunciado não pede um resultado nesses métodos.
4. **Remover arquivo do diretório:** significa apagar esse valor do vetor,
   não devolvê-lo à lista principal. Um método sem retorno de Arquivo não oferece
   a recuperação do objeto removido.
5. **Associação usuário/grupo:** os dois vetores de IDs são independentes;
   o enunciado define operações separadas e não exige atualização automática.
6. **stat pelo menu:** é chamado nas listagens, que já exibem os metadados.
   Não foi adicionada uma opção nova somente para o mesmo resultado.
7. **CRUD de Permissao:** foi mantida a permissão avulsa já existente, além das
   permissões dos arquivos e diretórios, para atender ao pedido original de CRUD
   de cada estrutura. O enunciado não determina uma coleção de permissões no main.
8. **“Sem macros” do pedido inicial:** foi interpretado como sem macros próprias;
   são usadas `println!` da biblioteca padrão para a saída no terminal.

### O que foi alterado nesta validação

- Substituído o comentário inicial por `// Dupla: Yago Landin e Gean Karlo`.
- Atualizada a identificação da dupla no README.
- Acrescentado este relatório com a tabela de conformidade e os resultados.
- Acrescentadas as duas transcrições de execução, com entradas e saídas reais.
- Nenhum campo, assinatura de método ou comportamento foi alterado nesta rodada.

### Limites honestos

Não encontrei entrada capaz de causar panic nos testes executados.
Isso não prova ausência de falhas em qualquer circunstância.

Não simulei esgotamento de memória, falhas físicas de stdin nem indisponibilidade
de stdout. Um volume extremo de dados pode esgotar a memória; `println!` pode panicar
se a saída padrão falhar. As funções internas de edição pressupõem índices
validados pelos menus; não há caminho atual de entrada do usuário que as chame
com índice fora do vetor.

A conformidade literal com uma Tabela 2 que não foi enviada continua sujeita
à confirmação das interpretações acima. Nenhum item foi marcado “sim” com base
em uma afirmação de acesso a esse documento.
