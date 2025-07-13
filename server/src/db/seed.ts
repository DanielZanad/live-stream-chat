import { reset, seed } from "drizzle-seed";
import { schema } from "./schema/index.ts";
import { db, sql } from "./connection.ts";

await reset(db, schema);

await seed(db, schema).refine((f) => {
  return {
    rooms: {
      count: 5,
      columns: {
        name: f.companyName(),
        description: f.loremIpsum(),
      },
      with: {
        questions: 5,
      },
    },
  };
});
await sql.end();

console.log("Database seeded");
