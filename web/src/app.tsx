import { BrowserRouter, Route, Routes } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { CreateRoom } from "./pages/create-room";
import { Room } from "./pages/room";
import { RecordRoomAudio } from "./pages/record-room-audio";

const queryClient = new QueryClient();

export function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <Routes>
          <Route index element={<CreateRoom />} />
          <Route path="/room/:roomId" element={<Room />} />
          <Route path="/room/:roomId/audio" element={<RecordRoomAudio />} />
        </Routes>
      </BrowserRouter>
    </QueryClientProvider>
  );
}
