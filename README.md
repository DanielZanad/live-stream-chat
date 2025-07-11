## Este projeto é composto por duas aplicações: um backend em Node.js usando Fastify e Drizzle ORM, e um frontend em React com Vite e TailwindCSS.

## Estrutura

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
```

#### Rodar as migrações e seed

```sh
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

## Endpoints principais

- `GET /health`: Health check
- `GET /rooms`: Lista de salas

---

## Scripts úteis

- Backend:
  - `npm run dev`: Inicia o servidor em modo desenvolvimento
  - `npm run db:seed`: Roda o seed do banco de dados
- Frontend:
  - `npm run dev`: Inicia o frontend em modo desenvolvimento

---

## Tecnologias

- Node.js, Fastify, Drizzle ORM, PostgreSQL
- React, Vite, TailwindCSS

---

## Licença

MIT
