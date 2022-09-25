import create from "zustand";
import shallow from "zustand/shallow";

interface SessionStore {
  // General session
  sessionToken?: string;
  setSessionToken: (sessionToken: string) => void;
  clearSessionToken: () => void;

  // Project related settings
  selectedProjectId?: string;
  setSelectedProjectId: (projectId: string) => void;
}

const useSessionStore = create<SessionStore>()((set) => ({
  sessionToken: undefined,
  setSessionToken(sessionToken) {
    set({ sessionToken });
  },
  clearSessionToken() {
    set({ sessionToken: undefined });
  },
  setSelectedProjectId(projectId) {
    set({ selectedProjectId: projectId });
  },
}));

const useSession = () =>
  useSessionStore(
    (state) => ({
      // General session
      sessionToken: state.sessionToken,
      setSessionToken: state.setSessionToken,
      clearSessionToken: state.clearSessionToken,

      // Project related settings
      selectedProjectId: state.selectedProjectId,
      setSelectedProjectId: state.setSelectedProjectId,
    }),
    shallow
  );

export { useSessionStore, useSession };
