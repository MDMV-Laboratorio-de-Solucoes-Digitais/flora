# Roteiro de testes — Flora staging zerado

**Objetivo:** permitir que o Software Engineer valide, em ambiente isolado e zerado, todas as funcionalidades atualmente implementadas no Flora (ciclos 096–101): First Seed condicionado, login local pt-BR, organização/workspace, canais, mensagens, tarefas, arquivos, busca, notificações, RBAC e reset.

**Ambiente:** URLs e credenciais de teste serão fornecidas pela MDMV-Autonoma por canal seguro. Substitua os placeholders abaixo:

```bash
export WEB_URL='https://<web-staging>'
export API_URL='https://<api-staging>'
export ADMIN_EMAIL='<email-entregue>'
export ADMIN_PASSWORD='<senha-entregue>'
```

> Não registrar valores reais neste arquivo, em PRs, logs ou mensagens.

---

## 0. Pré-condições

- [ ] Staging zerado: `GET $API_URL/api/v1/bootstrap/status` → `{"available":true}`.
- [ ] API responde `GET $API_URL/health` → 200.
- [ ] Frontend responde `GET $WEB_URL/` → 200 e `config.js` aponta para `$API_URL`.
- [ ] Nenhum dado de produção foi copiado para staging.

Comandos rápidos:

```bash
curl -sS -o /dev/null -w 'web=%{http_code}\n' "$WEB_URL/"
curl -sS -o /dev/null -w 'health=%{http_code}\n' "$API_URL/health"
curl -sS "$API_URL/api/v1/bootstrap/status"; echo
curl -sS "$WEB_URL/config.js"; echo
```

---

## 1. First Seed Journey (somente uma vez)

1. Abra `$WEB_URL/` em nova sessão/navegador limpo.
2. Confirme que o link **“Primeira configuração”** aparece (zero state).
3. Clique em **Fazer primeira configuração**.
4. Preencha:
   - Nome do workspace: `QA Staging`
   - E-mail do administrador: `$ADMIN_EMAIL`
   - Senha: `$ADMIN_PASSWORD` (mínimo 15 caracteres, maiúscula/minúscula/número/especial).
5. Clique em **Criar workspace**.
6. **Esperado:** sessão criada, redirecionamento para `/tasks` ou `/select-org`, token no `localStorage` (`flora.auth.v1`).
7. Faça logout e recarregue `$WEB_URL/`.
8. **Esperado:** link **“Primeira configuração”** não aparece mais.
9. Confirme:

```bash
curl -sS "$API_URL/api/v1/bootstrap/status"; echo
# {"available":false}
```

**Falha típica:** se o link continuar aparecendo, limpar cache (`Ctrl+Shift+R`) e conferir `bootstrap/status`.

---

## 2. Login local e casos de erro

### 2.1 Login válido

1. logout;
2. em **E-mail**, informe `$ADMIN_EMAIL`;
3. em **Senha**, informe `$ADMIN_PASSWORD`;
4. clique **Entrar**.

**Esperado:** `/select-org` com a organização criada; after **Selecionar**, `/tasks`.

### 2.2 Senha errada

- Use a mesma senha com último caractere alterado.
- **Esperado:** mensagem pt-BR “E-mail ou senha incorretos.”; sem token; sem vazamento de qual campo falhou.

### 2.3 Usuário inexistente

- Use `ninguém@example.test` + qualquer senha.
- **Esperado:** mesma mensagem genérica do item 2.2.

### 2.4 Limite de tentativas

- Faça 6 tentativas erradas em sequência.
- **Esperado:** a 6ª retorna mensagem pt-BR de limite (“Muitas tentativas...”) / HTTP 429.

### 2.5 SSO

- Clique **Entrar com SSO**.
- **Esperado:** erro do provedor não configurado em staging; documentar como limitação esperada, não como bug do login local.

### 2.6 API direta

```bash
curl -sS -o /tmp/flora-login.json -w 'status=%{http_code}\n' \
  -X POST -H 'Content-Type: application/json' \
  -d "{\"email\":\"$ADMIN_EMAIL\",\"password\":\"$ADMIN_PASSWORD\"}" \
  "$API_URL/api/v1/auth/password/login"
jq '{next_step,expires_in,user_id,has_token:(.session_token!=null),error}' /tmp/flora-login.json
```

**Esperado:** `status=200`, `has_token=true`.

---

## 3. Idioma (pt-BR)

Verifique em todas as telas:

- [ ] `<html lang="pt-BR">`.
- [ ] Rótulos: Entrar, E-mail, Senha, Primeira configuração, Tarefas, Canais, Mensagens diretas, Configurações.
- [ ] Nenhuma phrase em inglês fora de termos técnicos: OIDC, OAuth, API, JWT, REST, SSO, AI, slug.
- [ ] Erros do backend aparecem traduzidos; não mostrar `invalid_credentials`, `redirect_uri is not allowed`, etc.

---

## 4. Organização e workspace

1. Entre e vá a **Selecionar organização**.
2. Confirme a organização criada no First Seed.
3. Acesse **Configurações → Workspaces** (ou equivalente).
4. Crie um segundo workspace `QA Workspace 2`.
5. Troque de workspace pelo seletor.
6. **Esperado:** contexto muda sem logout; tarefas/canais ficam escopados ao workspace selecionado.

---

## 5. Canais e mensagens

1. Abra **Canais**.
2. Crie o canal `#qa-geral`.
3. Envie uma mensagem: `mensagem de teste staging`.
4. Responda em thread.
5. **Esperado:** mensagem e thread aparecem; sem erro no console; overlay de reconexão não aparece.
6. Abra outra sessão com o mesmo usuário e confirme atualização em tempo real (WebSocket/notificação in-app).

---

## 6. Mensagens diretas (DMs)

1. Aplique o fixture opcional com segundo usuário.
2. Abra **Mensagens diretas**.
3. Inicie conversa com o segundo usuário.
4. Envie mensagem.
5. **Esperado:** mensagem persistida; destinatário vê notificação in-app.

---

## 7. Tarefas

1. Abra **Tarefas**.
2. Crie tarefa `QA-001 — validar staging`.
3. Atribua a você (`Assign to me` / equivalente pt-BR).
4. Mude status: To Do → In Progress → Done.
5. Filtre por responsável e por status.
6. **Esperado:** persistência após reload; colunas Kanban em pt-BR; nenhuma tarefa de outra organização aparece.

---

## 8. Arquivos

1. Faça upload de um arquivo pequeno (< 50 MB).
2. Abra preview/download.
3. Tente upload de arquivo acima do limite configurado.
4. **Esperado:** upload válido funciona; inválido apresenta erro pt-BR “Arquivo muito grande...”; nada é gravado.

---

## 9. Busca

1. Indexe/aguarde mensagens e tarefas criadas.
2. Busque por `dashboard`, `QA-001` e por termo sem resultado.
3. **Esperado:** resultados com tipo traduzido (Mensagem/Tarefa/Arquivo); estado vazio pt-BR “Nenhum resultado para ...”.
4. Com a segunda organização do fixture, confirme que resultados de outra org não aparecem.

---

## 10. Notificações e realtime

1. Com duas sessões abertas, gere mensagem/tarefa para o segundo usuário.
2. **Esperado:** notificação in-app aparece; deduplicação evita duplicatas; sem polling perceptível.

---

## 11. RBAC / isolamento

1. Com o segundo usuário (fixture), tente acessar configurações administrativas.
2. **Esperado:** ações administrativas bloqueadas (403) ou escondidas; nenhuma mudança de dados de outra organização.
3. Confirme que IDs fixos de seed não permitem cross-org.

---

## 12. Reset e fixture

A MDMV-Autonoma fornecerá comandos concretos. Valide:

```bash
# 1) aplicar fixture opcional
<comando-entregue>

# 2) executar cenários 6–11

# 3) remover fixture / zerar staging
<comando-entregue>

curl -sS "$API_URL/api/v1/bootstrap/status"; echo
# {"available":true}
```

**Esperado:** volta ao zero state; First Seed aparece novamente em navegador limpo.

---

## 13. Critério de aceite do SE

- [ ] Todos os cenários 0–12 executados com data/hora.
- [ ] Evidências: screenshots dos fluxos críticos + saídas `curl`.
- [ ] Nenhum dado sensível/registro de credencial em arquivos.
- [ ] Falhas registradas com: cenário, passo, esperado, obtido, URL, timestamp, browser/versão.
- [ ] `[NV]` em qualquer item não executado por falta de fixture/ambiente.

---

## Modelo de registro de falha

```markdown
### F-001 — <título>
- Data/hora UTC:
- Cenário/roteiro item:
- URL:
- Passos:
- Esperado:
- Obtido:
- Evidência (screenshot/log):
- Impacto:
```
