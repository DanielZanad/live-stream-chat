import { useQuery } from "@tanstack/react-query";
import { Link } from "react-router-dom";

type GetRoomsAPIResponse = Array<{
  id: string;
  name: string;
}>;

export function CreateRoom() {
  const { data, isLoading } = useQuery({
    queryKey: ["get-rooms"],
    queryFn: async () => {
      const response = await fetch("http://localhost:3333/rooms");
      const result: GetRoomsAPIResponse = await response.json();

      return result;
    },
  });

  return (
    <div>
      <div>Create rooms</div>
      {isLoading ?? <p>Carregando...</p>}
      <div className="flex flex-col gap-2">
        {data &&
          data.map((room) => {
            return (
              <Link key={room.id} to={`/room/${room.id}`}>
                {room.name}
              </Link>
            );
          })}
      </div>
    </div>
  );
}
