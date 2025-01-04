import React, {
  createContext,
  useContext,
  useReducer,
  useCallback,
  useMemo,
  useEffect,
} from "react";
import { AuthClient } from "@dfinity/auth-client";
import { Identity } from "@dfinity/agent";
import { useSiwbIdentity } from "ic-siwb-lasereyes-connector";

// Constants
const IDENTITY_TYPES = {
  II: "II",
  SIWB: "SIWB",
} as const;

type IdentityType = (typeof IDENTITY_TYPES)[keyof typeof IDENTITY_TYPES] | null;

// State and Action types
type State = {
  identity: Identity | null;
  identityType: IdentityType;
  isAuthenticated: boolean;
};

type Action =
  | {
      type: "SET_IDENTITY";
      payload: { identity: Identity | null; identityType: IdentityType };
    }
  | { type: "SET_AUTHENTICATED"; payload: boolean };

// Context type
interface IdentityContextType extends State {
  loginII: (identityProvider?: string) => Promise<void>;
  loginSIWB: () => Promise<void>;
  logout: () => Promise<void>;
}

const IdentityContext = createContext<IdentityContextType | undefined>(
  undefined,
);

// Helper function to create AuthClient
const createAuthClient = async (): Promise<AuthClient> => {
  return await AuthClient.create();
};

// Reducer function
const identityReducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "SET_IDENTITY":
      return {
        ...state,
        identity: action.payload.identity,
        identityType: action.payload.identityType,
        isAuthenticated: !!action.payload.identity,
      };
    case "SET_AUTHENTICATED":
      return { ...state, isAuthenticated: action.payload };
    default:
      return state;
  }
};

export const useIdentity = () => {
  const context = useContext(IdentityContext);
  if (!context) {
    throw new Error("useIdentity must be used within an IdentityProvider");
  }
  return context;
};

export const IdentityProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [state, dispatch] = useReducer(identityReducer, {
    identity: null,
    identityType: null,
    isAuthenticated: false,
  });

  const {
    login: siwbLogin,
    identity: siwbIdentity,
    clear: siwbClear,
  } = useSiwbIdentity();

  useEffect(() => {
    const checkAuth = async () => {
      const authClient = await createAuthClient();
      const isIIAuthenticated = await authClient.isAuthenticated();
      if (isIIAuthenticated) {
        dispatch({
          type: "SET_IDENTITY",
          payload: {
            identity: authClient.getIdentity(),
            identityType: IDENTITY_TYPES.II,
          },
        });
      } else if (siwbIdentity) {
        dispatch({
          type: "SET_IDENTITY",
          payload: {
            identity: siwbIdentity,
            identityType: IDENTITY_TYPES.SIWB,
          },
        });
      }
    };
    checkAuth();
  }, [siwbIdentity]);

  const loginII = useCallback(async (identityProvider?: string) => {
    const authClient = await createAuthClient();
    const iiUrl =
      identityProvider ||
      `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;
    await new Promise<void>((resolve, reject) => {
      authClient.login({
        identityProvider: iiUrl,
        onSuccess: () => {
          dispatch({
            type: "SET_IDENTITY",
            payload: {
              identity: authClient.getIdentity(),
              identityType: IDENTITY_TYPES.II,
            },
          });
          resolve();
        },
        onError: reject,
      });
    });
  }, []);

  const loginSIWB = useCallback(async () => {
    const result = await siwbLogin();
    if (result && siwbIdentity) {
      dispatch({
        type: "SET_IDENTITY",
        payload: { identity: siwbIdentity, identityType: IDENTITY_TYPES.SIWB },
      });
    } else {
      console.error("SIWB login failed or identity is undefined");
    }
  }, [siwbLogin, siwbIdentity]);

  const logout = useCallback(async () => {
    if (state.identityType === IDENTITY_TYPES.II) {
      const authClient = await createAuthClient();
      await authClient.logout();
    } else if (state.identityType === IDENTITY_TYPES.SIWB) {
      siwbClear();
    }
    dispatch({
      type: "SET_IDENTITY",
      payload: { identity: null, identityType: null },
    });
  }, [state.identityType, siwbClear]);

  const contextValue = useMemo(
    () => ({
      ...state,
      loginII,
      loginSIWB,
      logout,
    }),
    [state, loginII, loginSIWB, logout],
  );

  return (
    <IdentityContext.Provider value={contextValue}>
      {children}
    </IdentityContext.Provider>
  );
};
