include!("../src/main.rs");

#[cfg(test)]
mod conferencia {
    use super::*;

    #[test]
    fn assinaturas_exatas_do_enunciado() {
        let _: fn(bool, bool, bool) -> Permissao = Permissao::new;
        let _: fn(&Permissao) -> u8 = Permissao::octal;
        let _: fn(String, u64, u16, u16) -> Arquivo = Arquivo::new;
        let _: fn(&mut Arquivo, (Permissao, Permissao, Permissao)) = Arquivo::alterar_permissao;
        let _: fn(&Arquivo) = Arquivo::stat;
        let _: fn(String, (Permissao, Permissao, Permissao), u16) -> Diretorio = Diretorio::new;
        let _: fn(&mut Diretorio, Arquivo) = Diretorio::adiciona_arquivo;
        let _: fn(&mut Diretorio, &str) = Diretorio::remove_arquivo;
        let _: fn(&Diretorio) = Diretorio::listar_conteudo;
        let _: fn(String, u16) -> Usuario = Usuario::new;
        let _: fn(&mut Usuario, u16) = Usuario::adiciona_grupo;
        let _: fn(&mut Usuario, u16) = Usuario::remove_grupo;
        let _: fn(&Usuario) = Usuario::listar_grupos;
        let _: fn(String, u16) -> Grupo = Grupo::new;
        let _: fn(&mut Grupo, u16) = Grupo::adiciona_membro;
        let _: fn(&mut Grupo, u16) = Grupo::remove_membro;
        let _: fn(&Grupo) = Grupo::listar_membros;
    }

    #[test]
    fn campos_e_tipos_exatos() {
        let permissao = Permissao::new(true, false, true);
        let _: &bool = &permissao.leitura;
        let _: &bool = &permissao.escrita;
        let _: &bool = &permissao.execucao;
        let arquivo = Arquivo::new(String::from("teste"), 0, 1000, 1000);
        let _: &String = &arquivo.nome;
        let _: &u64 = &arquivo.tamanho;
        let _: &(Permissao, Permissao, Permissao) = &arquivo.permissoes;
        let _: &u16 = &arquivo.uid_dono;
        let _: &u16 = &arquivo.gid_grupo;
        let diretorio = Diretorio::new(String::from("docs"), ler_tupla(), 1000);
        let _: &String = &diretorio.nome;
        let _: &Vec<Arquivo> = &diretorio.arquivos;
        let _: &(Permissao, Permissao, Permissao) = &diretorio.permissoes;
        let _: &u16 = &diretorio.uid_dono;
        let usuario = Usuario::new(String::from("Ana"), 1000);
        let _: &String = &usuario.nome;
        let _: &u16 = &usuario.uid;
        let _: &Vec<u16> = &usuario.grupos_ids;
        let grupo = Grupo::new(String::from("alunos"), 1000);
        let _: &String = &grupo.nome;
        let _: &u16 = &grupo.gid;
        let _: &Vec<u16> = &grupo.membros_uids;
    }

    fn ler_tupla() -> (Permissao, Permissao, Permissao) {
        (Permissao::new(true, true, true), Permissao::new(true, false, true),
            Permissao::new(true, false, true))
    }

    #[test]
    fn construtores_e_permissoes() {
        for leitura in [false, true] {
            for escrita in [false, true] {
                for execucao in [false, true] {
                    let permissao = Permissao::new(leitura, escrita, execucao);
                    assert_eq!(permissao.leitura, leitura);
                    assert_eq!(permissao.escrita, escrita);
                    assert_eq!(permissao.execucao, execucao);
                    let esperado = (if leitura {4} else {0}) + (if escrita {2} else {0})
                        + (if execucao {1} else {0});
                    assert_eq!(permissao.octal(), esperado);
                }
            }
        }
        let arquivo = Arquivo::new(String::from("teste"), 0, 1000, 1000);
        assert_eq!(arquivo.nome, "teste");
        assert_eq!(arquivo.tamanho, 0);
        assert_eq!(arquivo.uid_dono, 1000);
        assert_eq!(arquivo.gid_grupo, 1000);
        assert_eq!(arquivo.permissoes.0.octal(), 6);
        assert_eq!(arquivo.permissoes.1.octal(), 6);
        assert_eq!(arquivo.permissoes.2.octal(), 4);
        assert_eq!(texto_permissao(&arquivo.permissoes.0), "rw-");
        assert_eq!(texto_permissao(&arquivo.permissoes.1), "rw-");
        assert_eq!(texto_permissao(&arquivo.permissoes.2), "r--");
    }

    #[test]
    fn vetores_inicialmente_vazios_e_operacoes() {
        let mut diretorio = Diretorio::new(String::from("docs"), ler_tupla(), 1000);
        let mut usuario = Usuario::new(String::from("Ana"), 1000);
        let mut grupo = Grupo::new(String::from("alunos"), 1000);
        assert!(diretorio.arquivos.is_empty());
        assert!(usuario.grupos_ids.is_empty());
        assert!(grupo.membros_uids.is_empty());
        let mut arquivo = Arquivo::new(String::from("teste"), 0, 1000, 1000);
        arquivo.alterar_permissao(ler_tupla());
        assert_eq!(arquivo.permissoes.0.octal(), 7);
        diretorio.adiciona_arquivo(arquivo);
        assert_eq!(diretorio.arquivos.len(), 1);
        diretorio.remove_arquivo("ausente");
        assert_eq!(diretorio.arquivos.len(), 1);
        diretorio.remove_arquivo("teste");
        assert!(diretorio.arquivos.is_empty());
        usuario.adiciona_grupo(1000);
        usuario.adiciona_grupo(1000);
        assert_eq!(usuario.grupos_ids, [1000]);
        usuario.remove_grupo(999);
        assert_eq!(usuario.grupos_ids, [1000]);
        usuario.remove_grupo(1000);
        assert!(usuario.grupos_ids.is_empty());
        grupo.adiciona_membro(1000);
        grupo.adiciona_membro(1000);
        assert_eq!(grupo.membros_uids, [1000]);
        grupo.remove_membro(999);
        assert_eq!(grupo.membros_uids, [1000]);
        grupo.remove_membro(1000);
        assert!(grupo.membros_uids.is_empty());
    }
}
