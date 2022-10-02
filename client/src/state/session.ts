import create from "zustand";
import shallow from "zustand/shallow";

interface SessionStore {
  // General session
  sessionToken?: string;
  setSessionToken: (sessionToken: string) => void;
  clearSessionToken: () => void;

  // Project related settings
  // TODO: Should be handled in URL
  selectedProjectId?: string;
}

const useSessionStore = create<SessionStore>()((set) => ({
  sessionToken: undefined,
  setSessionToken(sessionToken) {
    set({ sessionToken });
  },
  clearSessionToken() {
    set({ sessionToken: undefined });
  },
}));

const useSession = () =>
  useSessionStore(
    (state) => ({
      // General session
      sessionToken: state.sessionToken,
      setSessionToken: state.setSessionToken,
      clearSessionToken: state.clearSessionToken,
    }),
    shallow
  );

export { useSessionStore, useSession };
