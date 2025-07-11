import { type FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { schema } from "../../db/schema/index.ts";
import { db } from "../../db/connection.ts";
import { eq, count } from "drizzle-orm";

export const getRoomsRoute: FastifyPluginAsyncZod = async (app) => {
  app.get("/rooms", async () => {
    const results = await db
      .select({
        id: schema.rooms.id,
        name: schema.rooms.name,
        createdAt: schema.rooms.createdAt,
        questionCount: count(schema.questions.id),
      })
      .from(schema.rooms)
      .leftJoin(schema.questions, eq(schema.questions.roomId, schema.rooms.id))
      .groupBy(schema.rooms.id)
      .orderBy(schema.rooms.createdAt);

    return results;
  });
};
