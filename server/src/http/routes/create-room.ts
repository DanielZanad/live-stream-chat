import { type FastifyPluginAsyncZod } from "fastify-type-provider-zod";
import { z } from "zod/v4";
import { schema } from "../../db/schema/index.ts";
import { db } from "../../db/connection.ts";

export const createRoom: FastifyPluginAsyncZod = async (app) => {
  app.post(
    "/rooms",
    {
      schema: {
        body: z.object({
          name: z.string().min(1),
          description: z.string().optional(),
        }),
      },
    },
    async (req, res) => {
      const { name, description } = req.body;

      const result = await db
        .insert(schema.rooms)
        .values({
          name,
          description,
        })
        .returning();

      const insertedRoom = result[0];

      if (!insertedRoom) {
        throw new Error("Failed to create new room");
      }

      return res.status(201).send({ roomId: insertedRoom.id });
    }
  );
};
