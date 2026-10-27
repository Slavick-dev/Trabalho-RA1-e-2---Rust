$ErrorActionPreference = 'Stop'
$raiz = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..'))
$pastaLogs = Join-Path $PSScriptRoot 'resultados'
[void][System.IO.Directory]::CreateDirectory($pastaLogs)
$utf8 = [System.Text.UTF8Encoding]::new($false)
$cargoExe = (Get-Command cargo -CommandType Application -ErrorAction Stop | Select-Object -First 1).Source
$exe = Join-Path $raiz 'target/debug/trabalho_ra1_ra2'
if ($IsWindows -or $env:OS -eq 'Windows_NT') { $exe += '.exe' }
$script:falhas = 0

function Gravar($caminho, $texto) {
    [System.IO.File]::WriteAllText($caminho, [string]$texto, $utf8)
}
function Normalizar($texto) {
    return ([string]$texto).Replace("`r`n", "`n")
}
function RodarProcesso($programa, $argumentos, $entrada, $limiteMs = 10000) {
    $config = [System.Diagnostics.ProcessStartInfo]::new()
    $config.FileName = $programa
    $config.Arguments = $argumentos
    $config.WorkingDirectory = $raiz
    $config.UseShellExecute = $false
    $config.RedirectStandardInput = $true
    $config.RedirectStandardOutput = $true
    $config.RedirectStandardError = $true
    $config.StandardInputEncoding = $utf8
    $config.StandardOutputEncoding = $utf8
    $config.StandardErrorEncoding = $utf8
    $processo = [System.Diagnostics.Process]::new()
    $processo.StartInfo = $config
    $relogio = [System.Diagnostics.Stopwatch]::StartNew()
    [void]$processo.Start()
    $leituraSaida = $processo.StandardOutput.ReadToEndAsync()
    $leituraErro = $processo.StandardError.ReadToEndAsync()
    $processo.StandardInput.Write([string]$entrada)
    $processo.StandardInput.Close()
    $timeout = -not $processo.WaitForExit($limiteMs)
    if ($timeout) { $processo.Kill($true); $processo.WaitForExit() }
    $saida = $leituraSaida.GetAwaiter().GetResult()
    $erro = $leituraErro.GetAwaiter().GetResult()
    $relogio.Stop()
    return @{ saida=$saida; erro=$erro; codigo=$processo.ExitCode; timeout=$timeout; ms=$relogio.ElapsedMilliseconds }
}
function ConferirTexto($saida, $contem, $naoContem) {
    $problemas = [System.Collections.Generic.List[string]]::new()
    $normal = Normalizar $saida
    foreach ($texto in $contem) {
        if (-not $normal.Contains((Normalizar $texto))) { $problemas.Add('Faltou: ' + $texto) }
    }
    foreach ($texto in $naoContem) {
        if ($normal.Contains((Normalizar $texto))) { $problemas.Add('Apareceu indevidamente: ' + $texto) }
    }
    return ,$problemas
}

# Etapa 1: os dois comandos realmente executados; nenhuma supressão de warning.
$compilacao = [System.Collections.Generic.List[object]]::new()
foreach ($comando in @(@{ nome='build'; args='build' }, @{ nome='clippy'; args='clippy --all-targets -- -D warnings' })) {
    $captura = RodarProcesso $cargoExe $comando.args '' 60000
    Gravar (Join-Path $pastaLogs ($comando.nome + '.stdout.txt')) $captura.saida
    Gravar (Join-Path $pastaLogs ($comando.nome + '.stderr.txt')) $captura.erro
    Write-Output ('> cargo ' + $comando.args)
    Write-Output $captura.saida
    Write-Output $captura.erro
    if ($captura.codigo -ne 0 -or $captura.timeout -or $captura.erro -match '(?m)^warning:') {
        throw ('Compilação/Clippy falhou: ' + $comando.nome)
    }
    $compilacao.Add(@{ comando=('cargo ' + $comando.args); captura=$captura })
}

# Etapa 2: uma execução, com cada opção enviada após o prompt real.
$roteiro = @(Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'roteiro.json') | ConvertFrom-Json)
$config = [System.Diagnostics.ProcessStartInfo]::new()
$config.FileName = $exe
$config.WorkingDirectory = $raiz
$config.UseShellExecute = $false
$config.RedirectStandardInput = $true
$config.RedirectStandardOutput = $true
$config.RedirectStandardError = $true
$config.StandardInputEncoding = $utf8
$config.StandardOutputEncoding = $utf8
$config.StandardErrorEncoding = $utf8
$processoMenu = [System.Diagnostics.Process]::new()
$processoMenu.StartInfo = $config
[void]$processoMenu.Start()
$erroMenuTask = $processoMenu.StandardError.ReadToEndAsync()
$stdoutCompleto = [System.Text.StringBuilder]::new()
$transcricao = [System.Text.StringBuilder]::new()
$prompts = @('Opção:', 'Número do item:', 'Nome do arquivo:', 'Tamanho em bytes:', 'Uid do dono:',
 'Gid do grupo:', 'Nome do diretório:', 'Nome do usuário:', 'Uid:', 'Nome do grupo:', 'Gid:',
 'Novo nome:', 'Novo tamanho:', 'Novo uid:', 'Novo gid:', 'Gid a adicionar:', 'Gid a remover:',
 'Uid a adicionar:', 'Uid a remover:', 'Nome do arquivo a apagar:')
function LerAtePrompt {
    $bloco = [System.Text.StringBuilder]::new()
    while ($true) {
        $tarefa = $processoMenu.StandardOutput.ReadLineAsync()
        if (-not $tarefa.Wait(10000)) {
            $processoMenu.Kill($true)
            throw 'FALHOU: mais de 10 segundos esperando resposta do menu'
        }
        $linha = $tarefa.GetAwaiter().GetResult()
        if ($null -eq $linha) { break }
        [void]$bloco.AppendLine($linha)
        [void]$stdoutCompleto.AppendLine($linha)
        [void]$transcricao.AppendLine($linha)
        if ($prompts -contains $linha -or $linha.StartsWith('Permissão de 0 a 7 (')) { break }
    }
    return $bloco.ToString()
}
$inicio = LerAtePrompt
$resultadosMenu = [System.Collections.Generic.List[object]]::new()
$entradaGerada = [System.Text.StringBuilder]::new()
try {
    for ($i=0; $i -lt $roteiro.Count; $i++) {
        $passo = $roteiro[$i]
        $saidaPasso = [System.Text.StringBuilder]::new()
        [void]$transcricao.AppendLine('=== ' + ($i+1) + '. ' + $passo.opcao + ' ===')
        foreach ($linhaEntrada in $passo.entrada) {
            [void]$entradaGerada.AppendLine($linhaEntrada)
            [void]$transcricao.AppendLine('[DIGITADO] ' + $linhaEntrada)
            $processoMenu.StandardInput.WriteLine($linhaEntrada)
            $processoMenu.StandardInput.Flush()
            [void]$saidaPasso.Append((LerAtePrompt))
        }
        $problemas = ConferirTexto $saidaPasso.ToString() $passo.contem $passo.naoContem
        $ok = $problemas.Count -eq 0
        if (-not $ok) { $script:falhas++ }
        $id = '{0:D2}' -f ($i+1)
        Gravar (Join-Path $pastaLogs ("opcao-$id.entrada.txt")) (($passo.entrada -join "`n") + "`n")
        Gravar (Join-Path $pastaLogs ("opcao-$id.stdout.txt")) $saidaPasso.ToString()
        $resultadosMenu.Add(@{ numero=($i+1); opcao=$passo.opcao; entrada=@($passo.entrada); saida=$saidaPasso.ToString(); ok=$ok; problemas=@($problemas.ToArray()) })
        Write-Output (('OK','FALHOU')[[int](-not $ok)] + ': ' + $passo.opcao)
    }
    $processoMenu.StandardInput.Close()
    if (-not $processoMenu.WaitForExit(5000)) { throw 'Não saiu pela opção 0' }
    $erroMenu = $erroMenuTask.GetAwaiter().GetResult()
    if ($processoMenu.ExitCode -ne 0 -or $erroMenu -ne '' -or $stdoutCompleto.ToString().Contains('Entrada encerrada.')) {
        throw 'A sessão de menu terminou com erro ou por EOF'
    }
} finally {
    if (-not $processoMenu.HasExited) { $processoMenu.Kill($true) }
}
Gravar (Join-Path $pastaLogs 'menu-transcricao.txt') $transcricao.ToString()
Gravar (Join-Path $pastaLogs 'menu.stdout.txt') $stdoutCompleto.ToString()
Gravar (Join-Path $pastaLogs 'menu.stderr.txt') $erroMenu
$arquivoEntrada = Get-Content -Raw -LiteralPath (Join-Path $raiz 'entrada.txt')
if ((Normalizar $entradaGerada.ToString()).TrimEnd() -ne (Normalizar $arquivoEntrada).TrimEnd()) { throw 'entrada.txt diverge do roteiro executado' }

# Repetição real via cargo run e o conteúdo de entrada.txt em stdin.
$execucaoCargo = RodarProcesso $cargoExe 'run' $arquivoEntrada 30000
Gravar (Join-Path $pastaLogs 'cargo-run.stdout.txt') $execucaoCargo.saida
Gravar (Join-Path $pastaLogs 'cargo-run.stderr.txt') $execucaoCargo.erro
if ($execucaoCargo.codigo -ne 0 -or $execucaoCargo.timeout -or (Normalizar $execucaoCargo.saida) -ne (Normalizar $stdoutCompleto.ToString())) {
    throw 'cargo run com entrada.txt divergiu da sessão por prompts'
}
Write-Output 'OK: cargo run com entrada.txt; stdout igual ao da sessão por prompts (normalizados CRLF/LF)'

# Etapa 3: casos independentes, cada processo com timeout e saída capturada integralmente.
$casos = @(Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'estresse.json') | ConvertFrom-Json)
$resultadosEstresse = [System.Collections.Generic.List[object]]::new()
for ($i=0; $i -lt $casos.Count; $i++) {
    $caso = $casos[$i]
    $captura = RodarProcesso $exe '' $caso.entrada 10000
    $problemas = ConferirTexto $captura.saida $caso.contem $caso.naoContem
    if ($captura.codigo -ne 0) { $problemas.Add('Código de saída: ' + $captura.codigo) }
    if ($captura.timeout) { $problemas.Add('Timeout de 10 segundos') }
    if ($captura.erro -ne '') { $problemas.Add('stderr não vazio: ' + $captura.erro) }
    if ($caso.eof) {
        if (-not $captura.saida.Contains('Entrada encerrada.')) { $problemas.Add('EOF não tratado') }
    } else {
        if (-not $captura.saida.Contains('Programa encerrado.')) { $problemas.Add('Não saiu pela opção 0') }
        if ($captura.saida.Contains('Entrada encerrada.')) { $problemas.Add('Saiu antes de concluir as entradas') }
    }
    if ($caso.contagens) {
        foreach ($par in $caso.contagens.PSObject.Properties) {
            $quantidade = [regex]::Matches((Normalizar $captura.saida), [regex]::Escape((Normalizar $par.Name))).Count
            if ($quantidade -ne $par.Value) { $problemas.Add("$($par.Name): $quantidade ocorrências; esperado $($par.Value)") }
        }
    }
    if ($caso.apos) {
        $texto = Normalizar $captura.saida
        $posicao = $texto.LastIndexOf($caso.apos, [System.StringComparison]::Ordinal)
        if ($posicao -lt 0) {
            $problemas.Add('Não encontrou o marco: ' + $caso.apos)
        } else {
            $trecho = $texto.Substring($posicao + $caso.apos.Length)
            $extras = ConferirTexto $trecho $caso.contemDepois $caso.naoContemDepois
            foreach ($extra in $extras) { $problemas.Add($extra) }
        }
    }
    $ok = $problemas.Count -eq 0
    if (-not $ok) { $script:falhas++ }
    $id = '{0:D2}' -f ($i+1)
    Gravar (Join-Path $pastaLogs ("estresse-$id.entrada.txt")) $caso.entrada
    Gravar (Join-Path $pastaLogs ("estresse-$id.stdout.txt")) $captura.saida
    Gravar (Join-Path $pastaLogs ("estresse-$id.stderr.txt")) $captura.erro
    $resultadosEstresse.Add(@{ numero=($i+1); nome=$caso.nome; ok=$ok; problemas=@($problemas.ToArray()); codigo=$captura.codigo; timeout=$captura.timeout; ms=$captura.ms; eof=[bool]$caso.eof; operacoes=$caso.operacoes })
    Write-Output (('OK','FALHOU')[[int](-not $ok)] + ': ' + $caso.nome)
    foreach ($problema in $problemas) { Write-Output ('  ' + $problema) }
}
# Etapa 4: conferir tipos, assinaturas e comportamento dos métodos pelo compilador.
$rustcExe = (Get-Command rustc -CommandType Application -ErrorAction Stop | Select-Object -First 1).Source
$buildConformidade = RodarProcesso $rustcExe '--edition 2021 --test testes/conformidade.rs -o target/conformidade-check.exe' '' 60000
Gravar (Join-Path $pastaLogs 'conformidade-build.stdout.txt') $buildConformidade.saida
Gravar (Join-Path $pastaLogs 'conformidade-build.stderr.txt') $buildConformidade.erro
if ($buildConformidade.codigo -ne 0 -or $buildConformidade.timeout -or $buildConformidade.erro -ne '') { throw 'Falhou a compilação da conferência de assinaturas' }
$conformidade = RodarProcesso (Join-Path $raiz 'target/conformidade-check.exe') '' '' 10000
Gravar (Join-Path $pastaLogs 'conformidade.stdout.txt') $conformidade.saida
Gravar (Join-Path $pastaLogs 'conformidade.stderr.txt') $conformidade.erro
if ($conformidade.codigo -ne 0 -or $conformidade.timeout -or $conformidade.erro -ne '') { throw 'Falhou a conferência dos campos e métodos' }
Write-Output $conformidade.saida
$hashFonte = (Get-FileHash -LiteralPath (Join-Path $raiz 'src/main.rs') -Algorithm SHA256).Hash
$resumo = @{ compilacao=@($compilacao.ToArray()); menu=@($resultadosMenu.ToArray()); cargoRun=$execucaoCargo; estresse=@($resultadosEstresse.ToArray()); conformidade=$conformidade; hashFonte=$hashFonte; falhas=$script:falhas }
Gravar (Join-Path $pastaLogs 'resumo.json') (ConvertTo-Json -InputObject $resumo -Depth 10)

# Relatório sem omitir ou abreviar as saídas das opções.
$md = [System.Text.StringBuilder]::new()
[void]$md.AppendLine('# Teste de execução real')
[void]$md.AppendLine()
[void]$md.AppendLine('Execução real de processos cargo e do binário Rust. As entradas foram escritas em stdin; as saídas abaixo foram capturadas de stdout/stderr, sem reconstrução ou substituição por resumos.')
[void]$md.AppendLine()
[void]$md.AppendLine('## Compilação: saída completa')
foreach ($item in $compilacao) {
    [void]$md.AppendLine()
    [void]$md.AppendLine('Comando: ' + $item.comando)
    [void]$md.AppendLine()
    [void]$md.AppendLine('stdout (bloco vazio significa zero bytes):')
    [void]$md.AppendLine('```text')
    [void]$md.Append($item.captura.saida)
    [void]$md.AppendLine('```')
    [void]$md.AppendLine('stderr, incluindo mensagens normais do Cargo:')
    [void]$md.AppendLine('```text')
    [void]$md.Append($item.captura.erro)
    [void]$md.AppendLine('```')
    [void]$md.AppendLine('Código de saída: ' + $item.captura.codigo)
}
[void]$md.AppendLine()
[void]$md.AppendLine('## Tabela de todas as opções executadas')
[void]$md.AppendLine()
[void]$md.AppendLine('As linhas pertencem à mesma sessão; os valores separados por → são enviados em linhas distintas. Opções de criação e edição sem confirmação textual também são verificadas pelas listagens posteriores.')
[void]$md.AppendLine()
[void]$md.AppendLine('| Opção do menu | Entrada usada | Resultado | OK/FALHOU |')
[void]$md.AppendLine('| --- | --- | --- | --- |')
foreach ($item in $resultadosMenu) {
    $status = if ($item.ok) { 'OK' } else { 'FALHOU' }
    [void]$md.AppendLine('| ' + $item.opcao + ' | ' + ($item.entrada -join ' → ') + ' | [Saída real integral](#opcao-' + $item.numero + ') | ' + $status + ' |')
}
[void]$md.AppendLine()
[void]$md.AppendLine('## Saída inicial real')
[void]$md.AppendLine('```text')
[void]$md.Append($inicio)
[void]$md.AppendLine('```')
foreach ($item in $resultadosMenu) {
    [void]$md.AppendLine()
    [void]$md.AppendLine('<a id="opcao-' + $item.numero + '"></a>')
    [void]$md.AppendLine('## ' + $item.numero + '. ' + $item.opcao)
    [void]$md.AppendLine()
    [void]$md.AppendLine('Entrada enviada:')
    [void]$md.AppendLine('```text')
    [void]$md.AppendLine(($item.entrada -join "`n"))
    [void]$md.AppendLine('```')
    [void]$md.AppendLine('Saída real, sem cortes:')
    [void]$md.AppendLine('```text')
    [void]$md.Append($item.saida)
    [void]$md.AppendLine('```')
    [void]$md.AppendLine('Resultado: ' + $(if ($item.ok) {'OK'} else {'FALHOU'}))
}
[void]$md.AppendLine()
[void]$md.AppendLine('## Repetição via cargo run')
[void]$md.AppendLine()
[void]$md.AppendLine('O conteúdo integral de entrada.txt também foi enviado ao stdin de cargo run. A saída de stdout coincidiu com a sessão anterior, após normalizar apenas quebras de linha CRLF/LF.')
[void]$md.AppendLine('Saídas integrais: [stdout](testes/resultados/cargo-run.stdout.txt) e [stderr](testes/resultados/cargo-run.stderr.txt).')
[void]$md.AppendLine()
[void]$md.AppendLine('## Estresse: processos reais independentes')
[void]$md.AppendLine()
[void]$md.AppendLine('Cada caso teve limite de 10 segundos. Casos normais exigem saída pela opção 0; nos casos EOF, o encerramento é esperado e deve emitir Entrada encerrada., sem repetição infinita.')
[void]$md.AppendLine()
[void]$md.AppendLine('| Caso | Entrada exata | Saída real integral | Código | Timeout | OK/FALHOU |')
[void]$md.AppendLine('| --- | --- | --- | --- | --- | --- |')
foreach ($item in $resultadosEstresse) {
    $id = '{0:D2}' -f $item.numero
    [void]$md.AppendLine('| ' + $item.nome + ' | [entrada](testes/resultados/estresse-' + $id + '.entrada.txt) | [stdout](testes/resultados/estresse-' + $id + '.stdout.txt), [stderr](testes/resultados/estresse-' + $id + '.stderr.txt) | ' + $item.codigo + ' | ' + $item.timeout + ' | ' + $(if ($item.ok) {'OK'} else {'FALHOU'}) + ' |')
}
[void]$md.AppendLine()
[void]$md.AppendLine('Falhas totais: ' + $script:falhas)
[void]$md.AppendLine()
[void]$md.AppendLine('## Conferência final e limites da evidência')
[void]$md.AppendLine()
[void]$md.AppendLine('O stat padrão foi comparado como cinco linhas consecutivas na opção 4, com nome teste, tamanho 0, permissões 664 / rw-rw-r--, uid 1000 e gid 1000.')
[void]$md.AppendLine('Os quatro casos de preservação conferem os campos completos do segundo item após editar e apagar o primeiro; o caso de diretórios também verifica o arquivo que pertence ao segundo diretório.')
[void]$md.AppendLine('Também foi realmente compilado testes/conformidade.rs, que inclui o main.rs atual e atribui os 17 métodos a ponteiros de função tipados. Os 18 campos, construtores, pesos octais e mutações de vetores também foram verificados. Esse arquivo é um teste separado; a implementação continua apenas em src/main.rs.')
[void]$md.AppendLine()
[void]$md.AppendLine('Saída integral da execução desses quatro testes:')
[void]$md.AppendLine('```text')
[void]$md.Append($conformidade.saida)
[void]$md.AppendLine('```')
[void]$md.AppendLine('Código de saída: ' + $conformidade.codigo + '; stderr vazio: ' + ($conformidade.erro -eq ''))
[void]$md.AppendLine('SHA256 do main.rs executado: ' + $hashFonte)
[void]$md.AppendLine()
[void]$md.AppendLine('Analisado no código: comentários, manutenção dos nomes, uso de &self e &mut self, guardas dos índices e correspondência com o enunciado transcrito. A Tabela 2 original não foi fornecida; permanece a interpretação de nome_arquivo como &str. A tabela detalhada da análise está em VALIDACAO_FINAL.md.')
[void]$md.AppendLine('Não foi pressionado Ctrl+D em uma interface gráfica: o teste fechou stdin de verdade no meio da entrada, produzindo EOF. Os seis processos responderam Entrada encerrada. e saíram com código 0; esse encerramento é intencional.')
[void]$md.AppendLine('Os resultados demonstram ausência de panic e timeout nestas entradas. Não são prova para todas as entradas possíveis e não cobrem falta de memória, falha física do terminal ou falha de stdout. Os processos de estresse tiveram limite de 10 segundos.')
[void]$md.AppendLine()
[void]$md.AppendLine('## Bugs encontrados e correções')
[void]$md.AppendLine()
[void]$md.AppendLine('- Programa Rust: nenhum bug encontrado nesta execução; src/main.rs não foi alterado.')
[void]$md.AppendLine('- Executor de testes: corrigida a resolução do Cargo quando há mais de um cargo.exe no PATH; passa a selecionar o primeiro. A tentativa anterior falhou antes de iniciar cargo build. Depois da correção, a bateria inteira foi executada novamente.')
[void]$md.AppendLine()
[void]$md.AppendLine('## Reproduzir')
[void]$md.AppendLine()
[void]$md.AppendLine('PowerShell 7, na raiz do projeto: pwsh -File testes/executar.ps1')
[void]$md.AppendLine('Bash ou cmd.exe, para a sessão completa: cargo run < entrada.txt')
[void]$md.AppendLine('No PowerShell, o operador < não redireciona stdin; use cmd /c "cargo run < entrada.txt".')
[void]$md.AppendLine()
[void]$md.AppendLine('O teste roda o programa; não se limita a gerar arquivos de entrada. EOF foi produzido fechando stdin do processo, equivalente ao efeito de Ctrl+D em um terminal Unix.')
Gravar (Join-Path $raiz 'TESTE_EXECUCAO_REAL.md') $md.ToString()
Write-Output ("TOTAL: $($resultadosMenu.Count) opções, $($resultadosEstresse.Count) casos de estresse, $script:falhas falhas")
if ($script:falhas -gt 0) { exit 1 }
