export default interface AuthContext {
    init: () => void;
    loggedIn: () => boolean;
    login: () => void;
    logout: (redirectUri: string) => void;
    username: () => string | undefined;
    hasRole: (role: string) => boolean;
}