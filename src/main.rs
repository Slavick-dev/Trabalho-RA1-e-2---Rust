// Dupla: [NOME 1] e [NOME 2]
use std::io;

// Guarda as três ações permitidas para uma pessoa.
struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool,
}

// Cria permissões e calcula sua representação octal.
impl Permissao {
    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    // Usa &self porque apenas consulta os valores, sem alterar a permissão.
    fn octal(&self) -> u8 {
        let mut valor = 0;
        if self.leitura {
            valor += 4;
        }
        if self.escrita {
            valor += 2;
        }
        if self.execucao {
            valor += 1;
        }
        valor
    }
}

// Representa um arquivo e suas permissões de dono, grupo e outros.
struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16,
}

// Cria arquivos, troca permissões e mostra seus dados.
impl Arquivo {
    // Recebe nome por valor: o arquivo passa a ser dono da String.
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

    // &mut self permite alterar o arquivo; a tupla recebida passa a pertencer a ele.
    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    // Usa &self para imprimir sem modificar nem consumir o arquivo.
    fn stat(&self) {
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        mostrar_permissoes(&self.permissoes);
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

// Guarda arquivos que pertencem a um diretório.
struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
}

// Gerencia os arquivos de um diretório.
impl Diretorio {
    // Nome e permissões são movidos para o diretório, que passa a ser dono deles.
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

    // &mut self altera o vetor; recebe Arquivo por valor porque o diretório será seu dono.
    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    // &mut self altera o vetor; &str consulta o nome sem tomar posse da String.
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

    // &self e &self.arquivos permitem listar sem mover os arquivos para o laço.
    fn listar_conteudo(&self) {
        if self.arquivos.is_empty() {
            println!("Diretório vazio.");
        }
        for arquivo in &self.arquivos {
            arquivo.stat();
        }
    }
}

// Guarda a identificação do usuário e os IDs de seus grupos.
struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>,
}

// Cria usuários e gerencia seus IDs de grupos.
impl Usuario {
    // Move a String nome para o novo usuário.
    fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }

    // &mut self permite mudar o vetor; u16 é copiado ao entrar no método.
    fn adiciona_grupo(&mut self, gid: u16) {
        if self.grupos_ids.contains(&gid) {
            println!("Grupo já cadastrado neste usuário.");
        } else {
            self.grupos_ids.push(gid);
        }
    }

    // &mut self permite remover um elemento do vetor.
    fn remove_grupo(&mut self, gid: u16) {
        for indice in 0..self.grupos_ids.len() {
            if self.grupos_ids[indice] == gid {
                self.grupos_ids.remove(indice);
                println!("Grupo removido do usuário.");
                return;
            }
        }
        println!("Grupo não encontrado neste usuário.");
    }

    // &self consulta os grupos; o laço empresta o vetor sem consumi-lo.
    fn listar_grupos(&self) {
        if self.grupos_ids.is_empty() {
            println!("Sem grupos.");
        }
        for gid in &self.grupos_ids {
            println!("Gid: {}", gid);
        }
    }
}

// Guarda a identificação do grupo e os IDs de seus membros.
struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16>,
}

// Cria grupos e gerencia seus IDs de membros.
impl Grupo {
    // Move a String nome para o novo grupo.
    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }

    // &mut self permite mudar o vetor; u16 é copiado ao entrar no método.
    fn adiciona_membro(&mut self, uid: u16) {
        if self.membros_uids.contains(&uid) {
            println!("Membro já cadastrado neste grupo.");
        } else {
            self.membros_uids.push(uid);
        }
    }

    // &mut self permite remover um elemento do vetor.
    fn remove_membro(&mut self, uid: u16) {
        for indice in 0..self.membros_uids.len() {
            if self.membros_uids[indice] == uid {
                self.membros_uids.remove(indice);
                println!("Membro removido do grupo.");
                return;
            }
        }
        println!("Membro não encontrado neste grupo.");
    }

    // &self consulta os membros; o laço empresta o vetor sem consumi-lo.
    fn listar_membros(&self) {
        if self.membros_uids.is_empty() {
            println!("Sem membros.");
        }
        for uid in &self.membros_uids {
            println!("Uid: {}", uid);
        }
    }
}

// Lê uma linha, permitindo que cada função valide seu próprio contexto.
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

// Lê nomes não vazios, retirando espaços das extremidades.
fn ler_texto(mensagem: &str) -> String {
    loop {
        let texto = ler_entrada(mensagem);
        if !texto.is_empty() {
            return texto;
        }
        println!("Nome inválido. Digite um nome não vazio.");
    }
}

// Lê números de 0 a 65535, repetindo a pergunta quando há erro.
fn ler_u16(mensagem: &str) -> u16 {
    loop {
        match ler_entrada(mensagem).parse::<u16>() {
            Ok(numero) => return numero,
            Err(_) => println!("Número inválido. Digite de 0 a 65535."),
        }
    }
}

// Lê um tamanho sem sinal, respeitando o limite do tipo u64.
fn ler_u64(mensagem: &str) -> u64 {
    loop {
        match ler_entrada(mensagem).parse::<u64>() {
            Ok(numero) => return numero,
            Err(_) => println!("Tamanho inválido. Digite um inteiro de 0 a 18446744073709551615."),
        }
    }
}

// Lê apenas opções existentes no menu que está sendo exibido.
fn ler_opcao(maximo: u16) -> u16 {
    loop {
        match ler_entrada("Opção:").parse::<u16>() {
            Ok(opcao) if opcao <= maximo => return opcao,
            _ => println!("Opção inválida. Digite um número de 0 a {}.", maximo),
        }
    }
}

// Seleciona uma posição exibida no menu e verifica os limites antes do acesso.
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

// Lê uma permissão como dígito octal e converte para três booleanos.
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

// Lê as permissões na ordem dono, grupo e outros.
fn ler_permissoes() -> (Permissao, Permissao, Permissao) {
    println!("Dono:");
    let dono = ler_permissao();
    println!("Grupo:");
    let grupo = ler_permissao();
    println!("Outros:");
    let outros = ler_permissao();
    (dono, grupo, outros)
}

// Converte uma permissão emprestada para o texto rwx, usando hífens nas ausências.
fn texto_permissao(permissao: &Permissao) -> String {
    let mut texto = String::new();
    texto.push(if permissao.leitura { 'r' } else { '-' });
    texto.push(if permissao.escrita { 'w' } else { '-' });
    texto.push(if permissao.execucao { 'x' } else { '-' });
    texto
}

// Imprime a tupla emprestada no formato octal e simbólico solicitado.
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

// Verifica nomes repetidos na mesma lista, ignorando o item que está sendo editado.
fn nome_arquivo_repetido(arquivos: &[Arquivo], nome: &str, ignorar: Option<usize>) -> bool {
    for (indice, arquivo) in arquivos.iter().enumerate() {
        if Some(indice) != ignorar && arquivo.nome == nome {
            return true;
        }
    }
    false
}

// Verifica nomes repetidos na mesma lista, ignorando o item que está sendo editado.
fn nome_diretorio_repetido(diretorios: &[Diretorio], nome: &str, ignorar: Option<usize>) -> bool {
    for (indice, diretorio) in diretorios.iter().enumerate() {
        if Some(indice) != ignorar && diretorio.nome == nome {
            return true;
        }
    }
    false
}

// Verifica nomes repetidos na mesma lista, ignorando o item que está sendo editado.
fn nome_usuario_repetido(usuarios: &[Usuario], nome: &str, ignorar: Option<usize>) -> bool {
    for (indice, usuario) in usuarios.iter().enumerate() {
        if Some(indice) != ignorar && usuario.nome == nome {
            return true;
        }
    }
    false
}

// Verifica nomes repetidos na mesma lista, ignorando o item que está sendo editado.
fn nome_grupo_repetido(grupos: &[Grupo], nome: &str, ignorar: Option<usize>) -> bool {
    for (indice, grupo) in grupos.iter().enumerate() {
        if Some(indice) != ignorar && grupo.nome == nome {
            return true;
        }
    }
    false
}

// Recusa nomes repetidos e cria um arquivo com as permissões padrão do construtor.
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

// Edita um arquivo já selecionado, mantendo nomes distintos na mesma lista.
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

// Mostra os arquivos e sua numeração sem consumir o vetor.
fn listar_arquivos(arquivos: &[Arquivo]) {
    if arquivos.is_empty() {
        println!("Nenhum arquivo cadastrado.");
    }
    for (indice, arquivo) in arquivos.iter().enumerate() {
        println!("\nItem {}", indice + 1);
        arquivo.stat();
    }
}

// Gerencia os arquivos que ainda não foram movidos para um diretório.
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

// Lista os dados e o conteúdo de todos os diretórios.
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

// Edita o diretório selecionado e gerencia os arquivos que ele possui.
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

// Cria, edita, apaga e lista diretórios usando o vetor mantido no main.
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

// Mostra os usuários e seus IDs de grupos com empréstimos imutáveis.
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

// Edita o usuário selecionado e adiciona ou remove IDs de grupos.
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

// Cria, edita, apaga e lista usuários.
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

// Mostra os grupos e seus IDs de membros com empréstimos imutáveis.
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

// Edita o grupo selecionado e adiciona ou remove IDs de membros.
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

// Cria, edita, apaga e lista grupos.
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

// Mantém uma permissão avulsa para praticar criação, edição, exclusão e consulta.
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

// Mantém os quatro vetores e repete o menu principal até a opção de saída.
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
