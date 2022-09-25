import create from "zustand";
import { Session } from "../interfaces/profile";

interface SessionStore {
  session?: Session;
  setSession: (session: Session) => void;
}

const useSessionStore = create<SessionStore>()((set) => ({
  session: undefined,
  setSession(session) {
    set({ session });
  },
}));

export { useSessionStore };
