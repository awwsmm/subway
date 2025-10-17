export default interface AuthContext {
    loggedIn: () => boolean;
    login: () => void;
    logout: (redirectUri: string) => void;
    username: () => string | undefined;
    hasRole: (roles: string[]) => boolean;
}