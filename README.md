# NLW Agents

Este projeto tem como principal objetivo gerar respostas automáticas utilizando IA (Google Gemini) sobre assuntos previamente abordados em uma sala, como por exemplo em uma livestream ou vídeo. Usuários podem criar salas, enviar áudios para transcrição e fazer perguntas relacionadas ao conteúdo dessas salas. As respostas são geradas com base nas transcrições dos áudios enviados.

## Principais Funcionalidades

- **Salas:** Crie salas temáticas para organizar perguntas e respostas.
- **Envio de Áudio:** Envie áudios (ex: trechos de lives ou vídeos) para serem transcritos e utilizados como contexto.
- **Perguntas:** Usuários podem fazer perguntas sobre o conteúdo já transcrito.
- **Respostas com IA:** As respostas são geradas pela IA apenas com base no conteúdo enviado para a sala.
- **Interface Web:** Frontend moderno em React + Vite + TailwindCSS.
- **Backend:** API em Node.js com Fastify, Drizzle ORM e PostgreSQL (pgvector).

---

## Estrutura do Projeto

- `server/`: Backend Fastify + Drizzle ORM + PostgreSQL
- `web/`: Frontend React + Vite + TailwindCSS

---

## Como rodar o projeto

### 1. Banco de Dados

O projeto utiliza PostgreSQL com a extensão pgvector.

#### Usando Docker

No diretório `server/`:

```sh
docker compose up -d
```

Isso irá subir o banco de dados PostgreSQL na porta 5432.

---

### 2. Backend (server)

#### Instalar dependências

```sh
cd server
npm install
```

#### Configurar variáveis de ambiente

Edite o arquivo `.env` conforme necessário. Exemplo padrão:

```
PORT=3333
DATABASE_URL="postgresql://docker:docker@localhost:5432/agents"
GEMINI_API_KEY="sua-chave-google-genai"
```

#### Rodar as migrações e seed

```sh
npm run db:migrate
npm run db:seed
```

#### Iniciar o servidor

```sh
npm run dev
```

O backend estará disponível em `http://localhost:3333`.

---

### 3. Frontend (web)

#### Instalar dependências

```sh
cd web
npm install
```

#### Rodar o frontend

```sh
npm run dev
```

Acesse em `http://localhost:5173`.

---

## Fluxo de Uso

1. **Crie uma sala**: Informe nome e descrição.
2. **Envie áudios**: Grave ou faça upload de áudios relevantes ao tema da sala.
3. **Faça perguntas**: Usuários podem perguntar sobre o conteúdo dos áudios.
4. **Receba respostas da IA**: A IA responde com base apenas no conteúdo transcrito dos áudios enviados para a sala.

---

## Endpoints principais

- `GET /health`: Health check
- `GET /rooms`: Lista de salas
- `POST /rooms`: Criação de sala
- `POST /rooms/:roomId/audio`: Upload de áudio para transcrição
- `GET /rooms/:roomId/questions`: Perguntas da sala
- `POST /rooms/:roomId/questions`: Criar pergunta

---

## Scripts úteis

- Backend:
  - `npm run dev`: Inicia o servidor em modo desenvolvimento
  - `npm run db:migrate`: Executa as migrações do banco
  - `npm run db:seed`: Popula o banco com dados de exemplo
- Frontend:
  - `npm run dev`: Inicia o frontend em modo desenvolvimento

---

## Tecnologias

- Node.js, Fastify, Drizzle ORM, PostgreSQL, pgvector, Google Gemini
- React, Vite, TailwindCSS

---

## Licença

MIT
