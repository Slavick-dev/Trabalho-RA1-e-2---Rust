// Dupla: Yago Landin e Gean Karlo
use std::io;

// ===== PERMISSAO =====
// Um bool para cada ação: assim dá para combinar as três.
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

// Deixo a criação e a conta da permissão junto do tipo.
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    // &self basta porque a conta só lê os campos.
    // Cada ação tem um peso: r=4, w=2 e x=1, igual ao chmod.
    // MEXER: pesos do dígito de permissão
    fn octal(&self) -> u8 {
        let leitura = if self.leitura { 4 } else { 0 };
        let escrita = if self.escrita { 2 } else { 0 };
        let execucao = if self.execucao { 1 } else { 0 };
        leitura + escrita + execucao
    }
}

// ===== ARQUIVO =====
// A tupla segue a ordem dono, grupo e outros.
// MEXER: campos do arquivo; conferir também new e stat
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}

// O arquivo nasce com permissões padrão, mas pode trocar depois.
impl Arquivo {
    // O nome vem por valor porque a String vai pertencer ao arquivo.
    // MEXER: aqui ficam as permissões padrão
    fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes: (
                Permissao::new(true, true, false),
                Permissao::new(true, true, false),
                Permissao::new(true, false, false),
            ),
            uid_dono: uid,
            gid_grupo: gid,
        }
    }

    // &mut self deixa trocar a tupla, que passa a pertencer ao arquivo.
    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    // &self deixa mostrar os dados sem alterar nem perder o arquivo.
    // MEXER: formato do stat
    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        mostrar_permissoes(&self.permissoes);
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

// ===== DIRETORIO =====
// O vetor guarda os próprios arquivos, não referências a eles.
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

// O diretório começa vazio; os arquivos entram depois.
impl Diretorio {
    // Nome e permissões vêm por valor porque vão pertencer ao diretório.
    fn new(
        nome: String,
        permissoes: (Permissao, Permissao, Permissao),
        uid_dono: u16,
    ) -> Diretorio {
        Diretorio {
            nome,
            arquivos: Vec::new(),
            permissoes,
            uid_dono,
        }
    }

    // &mut self muda o vetor; Arquivo vem por valor para o diretório ficar dono dele.
    // MEXER: entrada do arquivo por move
    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    // &mut self deixa remover; &str empresta só o nome para comparar.
    // Paro no primeiro nome encontrado, antes de usar outro índice.
    // MEXER: retorno das remoções; ver também remove_grupo e remove_membro
    fn remove_arquivo(&mut self, nome_arquivo: &str) {
        for indice in 0..self.arquivos.len() {
            if self.arquivos[indice].nome == nome_arquivo {
                self.arquivos.remove(indice);
                println!("Arquivo apagado do diretório.");
                return;
            }
        }
        println!("Arquivo não encontrado.");
    }

    // &self e o & no vetor deixam listar sem tirar os arquivos do diretório.
    fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("Diretório vazio.");
        }
        for arquivo in &self.arquivos {
            arquivo.stat();
        }
    }
}

// ===== USUARIO =====
// Aqui ficam só os gids; os grupos completos ficam em outra lista.
struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}

// Os gids do usuário são alterados aqui, sem mexer no vetor de grupos.
impl Usuario {
    // A String vem por valor para o usuário guardar o próprio nome.
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }

    // &mut self permite incluir; o gid é u16 e entra como cópia.
    // O contains evita repetir o mesmo gid neste usuário.
    fn adiciona_grupo(&mut self, gid: u16) {
        if self.grupos_ids.contains(&gid) {
            println!("Grupo já cadastrado neste usuário.");
        } else {
            self.grupos_ids.push(gid);
        }
    }

    // &mut self é necessário para remover; position procura a primeira ocorrência do gid.
    fn remove_grupo(&mut self, gid: u16) {
        if let Some(indice) = self.grupos_ids.iter().position(|&grupo| grupo == gid) {
            self.grupos_ids.remove(indice);
            println!("Grupo removido do usuário.");
        } else {
            println!("Grupo não encontrado neste usuário.");
        }
    }

    // &self só consulta; o for pega os gids emprestados e mantém o vetor.
    fn listar_grupos(&self) {
        if self.grupos_ids.is_empty() {
            println!("Sem grupos.");
        }
        for gid in &self.grupos_ids {
            println!("Gid: {}", gid);
        }
    }
}

// ===== GRUPO =====
// Guardo os uids dos membros, sem mover os usuários para cá.
struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}

// Mudar estes membros não atualiza os gids dos usuários automaticamente.
impl Grupo {
    // O grupo fica com a String recebida por valor; o gid é copiado.
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }

    // &mut self deixa incluir o uid copiado, depois de conferir se já existe.
    fn adiciona_membro(&mut self, uid: u16) {
        if self.membros_uids.contains(&uid) {
            println!("Membro já cadastrado neste grupo.");
        } else {
            self.membros_uids.push(uid);
        }
    }

    // &mut self deixa remover; position devolve None quando não acha o uid.
    fn remove_membro(&mut self, uid: u16) {
        if let Some(indice) = self.membros_uids.iter().position(|&membro| membro == uid) {
            self.membros_uids.remove(indice);
            println!("Membro removido do grupo.");
        } else {
            println!("Membro não encontrado neste grupo.");
        }
    }

    // &self basta para ler; o for não tira os membros do vetor.
    fn listar_membros(&self) {
        if self.membros_uids.is_empty() {
            println!("Sem membros.");
        }
        for uid in &self.membros_uids {
            println!("Uid: {}", uid);
        }
    }
}

// ===== LEITURA DE ENTRADA (funções ler_*) =====
// Centralizo a leitura para todos os pedidos tratarem EOF do mesmo jeito.
// &str empresta o texto do pedido; a String devolvida pertence a quem chamou.
// Ok(0) é fim da entrada, então saio em vez de repetir a pergunta para sempre.
fn ler_entrada(mensagem: &str) -> String {
    println!("{}", mensagem);
    let mut entrada = String::new();
    match io::stdin().read_line(&mut entrada) {
        Ok(0) => {
            println!("Entrada encerrada.");
            std::process::exit(0);
        }
        Ok(_) => entrada.trim().to_string(),
        Err(_) => {
            println!("Não foi possível ler a entrada.");
            std::process::exit(1);
        }
    }
}

// Enter e espaços sozinhos não servem como nome; ler_entrada já usa trim.
// MEXER: validação e tamanho máximo dos nomes
fn ler_texto(mensagem: &str) -> String {
    loop {
        let texto = ler_entrada(mensagem);
        if !texto.is_empty() {
            return texto;
        }
        println!("Nome inválido. Digite um nome não vazio.");
    }
}

// O parse barra negativos e valores fora de u16, sem dar panic.
// MEXER: limite do uid/gid; esta leitura é usada para os dois
fn ler_u16(mensagem: &str) -> u16 {
    loop {
        match ler_entrada(mensagem).parse::<u16>() {
            Ok(numero) => return numero,
            Err(_) => println!("Número inválido. Digite de 0 a 65535."),
        }
    }
}

// Tamanho não pode ser negativo; se não couber em u64, pergunto de novo.
fn ler_u64(mensagem: &str) -> u64 {
    loop {
        match ler_entrada(mensagem).parse::<u64>() {
            Ok(numero) => return numero,
            Err(_) => println!("Tamanho inválido. Digite um inteiro de 0 a 18446744073709551615."),
        }
    }
}

// Cada menu passa seu maior número para não aceitar uma opção que não existe.
fn ler_opcao(maximo: u16) -> u16 {
    loop {
        match ler_entrada("Opção:").parse::<u16>() {
            Ok(opcao) if opcao <= maximo => return opcao,
            _ => println!("Opção inválida. Digite um número de 0 a {}.", maximo),
        }
    }
}

// A tela começa em 1, mas o vetor começa em 0.
// Só devolvo um índice depois de checar os limites; lista vazia devolve None.
fn ler_indice(quantidade: usize) -> Option<usize> {
    if quantidade == 0 {
        return None;
    }
    println!("Escolha um item de 1 a {}.", quantidade);
    match ler_entrada("Número do item:").parse::<usize>() {
        Ok(numero) if numero >= 1 && numero <= quantidade => Some(numero - 1),
        _ => {
            println!("Item inválido. Escolha um número de 1 a {}.", quantidade);
            None
        }
    }
}

// Só aceito 0 a 7: os bits 4, 2 e 1 dizem quais ações ficam ligadas.
fn ler_permissao() -> Permissao {
    loop {
        let entrada = ler_entrada("Permissão de 0 a 7 (leitura=4, escrita=2, execução=1):");
        match entrada.parse::<u8>() {
            Ok(valor) if valor <= 7 => {
                return Permissao::new(valor & 4 != 0, valor & 2 != 0, valor & 1 != 0);
            }
            _ => println!("Permissão inválida. Digite um número de 0 a 7."),
        }
    }
}

// Mantenho a ordem da tupla para não trocar dono, grupo e outros.
fn ler_permissoes() -> (Permissao, Permissao, Permissao) {
    println!("Dono:");
    let dono = ler_permissao();
    println!("Grupo:");
    let grupo = ler_permissao();
    println!("Outros:");
    let outros = ler_permissao();
    (dono, grupo, outros)
}

// ===== FUNÇÕES AUXILIARES (permissões, nomes repetidos) =====
// Empresto a permissão porque só preciso ler; cada ação desligada vira um hífen.
fn texto_permissao(permissao: &Permissao) -> String {
    let mut texto = String::new();
    texto.push(if permissao.leitura { 'r' } else { '-' });
    texto.push(if permissao.escrita { 'w' } else { '-' });
    texto.push(if permissao.execucao { 'x' } else { '-' });
    texto
}

// Empresto a tupla para mostrar os três dígitos sem consumir as permissões.
// MEXER: exibição do octal completo e do texto rwx
fn mostrar_permissoes(permissoes: &(Permissao, Permissao, Permissao)) {
    println!(
        "Permissoes: ({}{}{} / {}{}{})",
        permissoes.0.octal(),
        permissoes.1.octal(),
        permissoes.2.octal(),
        texto_permissao(&permissoes.0),
        texto_permissao(&permissoes.1),
        texto_permissao(&permissoes.2)
    );
}

// Empresto a lista; any para no primeiro outro arquivo com esse nome.
// None compara todos; Some(indice) pula o próprio item ao renomear.
// MEXER: regra de nomes repetidos nas listas e dentro do diretório
fn nome_arquivo_repetido(arquivos: &[Arquivo], nome: &str, ignorar: Option<usize>) -> bool {
    arquivos
        .iter()
        .enumerate()
        .any(|(indice, arquivo)| Some(indice) != ignorar && arquivo.nome == nome)
}

// Mesma checagem de nomes, agora na lista emprestada de diretórios.
fn nome_diretorio_repetido(diretorios: &[Diretorio], nome: &str, ignorar: Option<usize>) -> bool {
    diretorios
        .iter()
        .enumerate()
        .any(|(indice, diretorio)| Some(indice) != ignorar && diretorio.nome == nome)
}

// Para usuários, também pulo o próprio item ao renomear.
fn nome_usuario_repetido(usuarios: &[Usuario], nome: &str, ignorar: Option<usize>) -> bool {
    usuarios
        .iter()
        .enumerate()
        .any(|(indice, usuario)| Some(indice) != ignorar && usuario.nome == nome)
}

// Nos grupos, a busca só consulta a lista emprestada.
fn nome_grupo_repetido(grupos: &[Grupo], nome: &str, ignorar: Option<usize>) -> bool {
    grupos
        .iter()
        .enumerate()
        .any(|(indice, grupo)| Some(indice) != ignorar && grupo.nome == nome)
}

// ===== MENUS =====
// A lista é emprestada só para conferir o nome.
// Devolvo o arquivo por valor para quem chamou poder guardá-lo no vetor.
fn criar_arquivo(arquivos: &[Arquivo]) -> Option<Arquivo> {
    let nome = ler_texto("Nome do arquivo:");
    if nome_arquivo_repetido(arquivos, &nome, None) {
        println!("Já existe um arquivo com esse nome nesta lista.");
        return None;
    }
    let tamanho = ler_u64("Tamanho em bytes:");
    let uid = ler_u16("Uid do dono:");
    let gid = ler_u16("Gid do grupo:");
    Some(Arquivo::new(nome, tamanho, uid, gid))
}

// &mut deixa editar o arquivo escolhido sem tomar a lista inteira.
// O índice já foi conferido por ler_indice antes de chegar aqui.
// MEXER: opções da edição de arquivo; manter texto e match juntos
fn editar_arquivo(arquivos: &mut [Arquivo], indice: usize) {
    loop {
        println!("\nEditar arquivo: {}", arquivos[indice].nome);
        println!("1 - Nome\n2 - Tamanho\n3 - Permissões\n4 - Uid\n5 - Gid\n0 - Voltar");
        match ler_opcao(5) {
            1 => {
                let nome = ler_texto("Novo nome:");
                if nome_arquivo_repetido(arquivos, &nome, Some(indice)) {
                    println!("Já existe um arquivo com esse nome nesta lista.");
                } else {
                    arquivos[indice].nome = nome;
                }
            }
            2 => arquivos[indice].tamanho = ler_u64("Novo tamanho:"),
            3 => arquivos[indice].alterar_permissao(ler_permissoes()),
            4 => arquivos[indice].uid_dono = ler_u16("Novo uid:"),
            5 => arquivos[indice].gid_grupo = ler_u16("Novo gid:"),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// A lista é emprestada para mostrar os arquivos sem tirá-los dela.
// MEXER: ordem de listagem dos arquivos
fn listar_arquivos(arquivos: &[Arquivo]) {
    if arquivos.is_empty() {
        println!("Nenhum arquivo cadastrado.");
    }
    for (indice, arquivo) in arquivos.iter().enumerate() {
        println!("\nItem {}", indice + 1);
        arquivo.stat();
    }
}

// Preciso de &mut Vec porque criar e apagar mudam o tamanho da lista.
// Aqui ficam os arquivos que ainda não foram movidos para um diretório.
fn menu_arquivos(arquivos: &mut Vec<Arquivo>) {
    loop {
        println!("\nARQUIVOS\n1 - Criar\n2 - Editar\n3 - Apagar\n4 - Listar\n0 - Voltar");
        match ler_opcao(4) {
            1 => {
                if let Some(arquivo) = criar_arquivo(arquivos) {
                    arquivos.push(arquivo);
                }
            }
            2 => {
                listar_arquivos(arquivos);
                if let Some(indice) = ler_indice(arquivos.len()) {
                    editar_arquivo(arquivos, indice);
                }
            }
            3 => {
                listar_arquivos(arquivos);
                if let Some(indice) = ler_indice(arquivos.len()) {
                    arquivos.remove(indice);
                    println!("Arquivo apagado.");
                }
            }
            4 => listar_arquivos(arquivos),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// Só empresto os diretórios para mostrar seus dados e os arquivos de cada um.
fn listar_diretorios(diretorios: &[Diretorio]) {
    if diretorios.is_empty() {
        println!("Nenhum diretório cadastrado.");
    }
    for (indice, diretorio) in diretorios.iter().enumerate() {
        println!("\nItem {} - Diretório: {}", indice + 1, diretorio.nome);
        println!("Uid: {}", diretorio.uid_dono);
        mostrar_permissoes(&diretorio.permissoes);
        diretorio.listar_conteudo();
    }
}

// As duas listas são mutáveis porque mover tira o arquivo de uma e põe na outra.
// O diretório fica dono do arquivo; não crio uma segunda cópia.
fn editar_diretorio(diretorios: &mut [Diretorio], indice: usize, arquivos: &mut Vec<Arquivo>) {
    loop {
        println!("\nEditar diretório: {}", diretorios[indice].nome);
        println!("1 - Nome\n2 - Permissões\n3 - Uid\n4 - Mover arquivo para cá");
        println!(
            "5 - Apagar arquivo pelo nome\n6 - Editar arquivo\n7 - Listar conteúdo\n0 - Voltar"
        );
        match ler_opcao(7) {
            1 => {
                let nome = ler_texto("Novo nome:");
                if nome_diretorio_repetido(diretorios, &nome, Some(indice)) {
                    println!("Já existe um diretório com esse nome nesta lista.");
                } else {
                    diretorios[indice].nome = nome;
                }
            }
            2 => diretorios[indice].permissoes = ler_permissoes(),
            3 => diretorios[indice].uid_dono = ler_u16("Novo uid:"),
            4 => {
                listar_arquivos(arquivos);
                if let Some(indice_arquivo) = ler_indice(arquivos.len()) {
                    if nome_arquivo_repetido(
                        &diretorios[indice].arquivos,
                        &arquivos[indice_arquivo].nome,
                        None,
                    ) {
                        println!("Já existe um arquivo com esse nome no diretório.");
                    } else {
                        let arquivo = arquivos.remove(indice_arquivo);
                        diretorios[indice].adiciona_arquivo(arquivo);
                        println!("Arquivo movido para o diretório.");
                    }
                }
            }
            5 => {
                if diretorios[indice].arquivos.is_empty() {
                    println!("Diretório vazio.");
                } else {
                    let nome = ler_texto("Nome do arquivo a apagar:");
                    diretorios[indice].remove_arquivo(&nome);
                }
            }
            6 => {
                listar_arquivos(&diretorios[indice].arquivos);
                if let Some(indice_arquivo) = ler_indice(diretorios[indice].arquivos.len()) {
                    editar_arquivo(&mut diretorios[indice].arquivos, indice_arquivo);
                }
            }
            7 => diretorios[indice].listar_conteudo(),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// &mut deixa criar ou apagar diretórios e mover arquivos para dentro deles.
fn menu_diretorios(diretorios: &mut Vec<Diretorio>, arquivos: &mut Vec<Arquivo>) {
    loop {
        println!("\nDIRETÓRIOS\n1 - Criar\n2 - Editar\n3 - Apagar diretório e seus arquivos\n4 - Listar\n0 - Voltar");
        match ler_opcao(4) {
            1 => {
                let nome = ler_texto("Nome do diretório:");
                if nome_diretorio_repetido(diretorios, &nome, None) {
                    println!("Já existe um diretório com esse nome nesta lista.");
                } else {
                    let permissoes = ler_permissoes();
                    let uid = ler_u16("Uid do dono:");
                    diretorios.push(Diretorio::new(nome, permissoes, uid));
                }
            }
            2 => {
                listar_diretorios(diretorios);
                if let Some(indice) = ler_indice(diretorios.len()) {
                    editar_diretorio(diretorios, indice, arquivos);
                }
            }
            3 => {
                listar_diretorios(diretorios);
                if let Some(indice) = ler_indice(diretorios.len()) {
                    diretorios.remove(indice);
                    println!("Diretório e seus arquivos apagados.");
                }
            }
            4 => listar_diretorios(diretorios),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// Empresto os usuários para listar; os dados continuam na lista do main.
fn listar_usuarios(usuarios: &[Usuario]) {
    if usuarios.is_empty() {
        println!("Nenhum usuário cadastrado.");
    }
    for (indice, usuario) in usuarios.iter().enumerate() {
        println!(
            "\nItem {} - Usuário: {} | Uid: {}",
            indice + 1,
            usuario.nome,
            usuario.uid
        );
        usuario.listar_grupos();
    }
}

// &mut deixa mudar os dados do usuário selecionado, sem mover o usuário.
fn editar_usuario(usuarios: &mut [Usuario], indice: usize) {
    loop {
        println!("\nEditar usuário: {}", usuarios[indice].nome);
        println!("1 - Nome\n2 - Uid\n3 - Adicionar grupo\n4 - Remover grupo\n5 - Listar grupos\n0 - Voltar");
        match ler_opcao(5) {
            1 => {
                let nome = ler_texto("Novo nome:");
                if nome_usuario_repetido(usuarios, &nome, Some(indice)) {
                    println!("Já existe um usuário com esse nome nesta lista.");
                } else {
                    usuarios[indice].nome = nome;
                }
            }
            2 => usuarios[indice].uid = ler_u16("Novo uid:"),
            3 => usuarios[indice].adiciona_grupo(ler_u16("Gid a adicionar:")),
            4 => usuarios[indice].remove_grupo(ler_u16("Gid a remover:")),
            5 => usuarios[indice].listar_grupos(),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// Uso &mut Vec porque as opções podem incluir ou remover usuários.
fn menu_usuarios(usuarios: &mut Vec<Usuario>) {
    loop {
        println!("\nUSUÁRIOS\n1 - Criar\n2 - Editar\n3 - Apagar\n4 - Listar\n0 - Voltar");
        match ler_opcao(4) {
            1 => {
                let nome = ler_texto("Nome do usuário:");
                if nome_usuario_repetido(usuarios, &nome, None) {
                    println!("Já existe um usuário com esse nome nesta lista.");
                } else {
                    let uid = ler_u16("Uid:");
                    usuarios.push(Usuario::new(nome, uid));
                }
            }
            2 => {
                listar_usuarios(usuarios);
                if let Some(indice) = ler_indice(usuarios.len()) {
                    editar_usuario(usuarios, indice);
                }
            }
            3 => {
                listar_usuarios(usuarios);
                if let Some(indice) = ler_indice(usuarios.len()) {
                    usuarios.remove(indice);
                    println!("Usuário apagado.");
                }
            }
            4 => listar_usuarios(usuarios),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// Empresto a lista para consultar os grupos e os uids que eles guardam.
fn listar_grupos(grupos: &[Grupo]) {
    if grupos.is_empty() {
        println!("Nenhum grupo cadastrado.");
    }
    for (indice, grupo) in grupos.iter().enumerate() {
        println!(
            "\nItem {} - Grupo: {} | Gid: {}",
            indice + 1,
            grupo.nome,
            grupo.gid
        );
        grupo.listar_membros();
    }
}

// &mut permite mudar o grupo escolhido e seu vetor de membros.
fn editar_grupo(grupos: &mut [Grupo], indice: usize) {
    loop {
        println!("\nEditar grupo: {}", grupos[indice].nome);
        println!("1 - Nome\n2 - Gid\n3 - Adicionar membro\n4 - Remover membro\n5 - Listar membros\n0 - Voltar");
        match ler_opcao(5) {
            1 => {
                let nome = ler_texto("Novo nome:");
                if nome_grupo_repetido(grupos, &nome, Some(indice)) {
                    println!("Já existe um grupo com esse nome nesta lista.");
                } else {
                    grupos[indice].nome = nome;
                }
            }
            2 => grupos[indice].gid = ler_u16("Novo gid:"),
            3 => grupos[indice].adiciona_membro(ler_u16("Uid a adicionar:")),
            4 => grupos[indice].remove_membro(ler_u16("Uid a remover:")),
            5 => grupos[indice].listar_membros(),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// A lista vem com &mut porque criar e apagar mudam o vetor do main.
fn menu_grupos(grupos: &mut Vec<Grupo>) {
    loop {
        println!("\nGRUPOS\n1 - Criar\n2 - Editar\n3 - Apagar\n4 - Listar\n0 - Voltar");
        match ler_opcao(4) {
            1 => {
                let nome = ler_texto("Nome do grupo:");
                if nome_grupo_repetido(grupos, &nome, None) {
                    println!("Já existe um grupo com esse nome nesta lista.");
                } else {
                    let gid = ler_u16("Gid:");
                    grupos.push(Grupo::new(nome, gid));
                }
            }
            2 => {
                listar_grupos(grupos);
                if let Some(indice) = ler_indice(grupos.len()) {
                    editar_grupo(grupos, indice);
                }
            }
            3 => {
                listar_grupos(grupos);
                if let Some(indice) = ler_indice(grupos.len()) {
                    grupos.remove(indice);
                    println!("Grupo apagado.");
                }
            }
            4 => listar_grupos(grupos),
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// None quer dizer que ainda não há permissão avulsa; Some guarda uma.
// Uso &mut para criar, trocar ou apagar esse valor no main.
fn menu_permissao(permissao: &mut Option<Permissao>) {
    loop {
        println!("\nPERMISSÃO AVULSA\n1 - Criar\n2 - Editar\n3 - Apagar\n4 - Listar\n0 - Voltar");
        match ler_opcao(4) {
            1 => {
                if permissao.is_some() {
                    println!("Já existe uma permissão. Use Editar ou Apagar.");
                } else {
                    *permissao = Some(ler_permissao());
                }
            }
            2 => {
                if let Some(atual) = permissao {
                    *atual = ler_permissao();
                } else {
                    println!("Permissão não encontrada.");
                }
            }
            3 => {
                if permissao.is_some() {
                    *permissao = None;
                    println!("Permissão apagada.");
                } else {
                    println!("Permissão não encontrada.");
                }
            }
            4 => {
                if let Some(atual) = permissao {
                    println!("Permissão: {} / {}", atual.octal(), texto_permissao(atual));
                } else {
                    println!("Permissão não encontrada.");
                }
            }
            0 => break,
            _ => println!("Opção inválida."),
        }
    }
}

// ===== MAIN =====
// As listas ficam aqui para os menus usarem os mesmos dados até sair.
// MEXER: opções do menu principal e chamada de cada submenu
fn main() {
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut permissao: Option<Permissao> = None;

    loop {
        println!("\nMENU PRINCIPAL");
        println!("1 - Arquivos\n2 - Diretórios\n3 - Usuários\n4 - Grupos\n5 - Permissão avulsa\n0 - Sair");
        match ler_opcao(5) {
            1 => menu_arquivos(&mut arquivos),
            2 => menu_diretorios(&mut diretorios, &mut arquivos),
            3 => menu_usuarios(&mut usuarios),
            4 => menu_grupos(&mut grupos),
            5 => menu_permissao(&mut permissao),
            0 => {
                println!("Programa encerrado.");
                break;
            }
            _ => println!("Opção inválida."),
        }
    }
}
