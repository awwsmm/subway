// import React, { createContext, useContext, useEffect, useState } from 'react';
// import keycloak from './keycloak';
//
// interface AuthContextType {
//     isAuthenticated: boolean;
//     username: string | null;
//     login: () => void;
//     logout: () => void;
// }
//
// const AuthContext = createContext<AuthContextType | undefined>(undefined);
//
// export const AuthProvider = ({ children }: { children: React.ReactNode }) => {
//     const [isAuthenticated, setIsAuthenticated] = useState(false);
//     const [username, setUsername] = useState<string | null>(null);
//
//     useEffect(() => {
//         keycloak
//             .init({
//                 onLoad: 'check-sso',
//                 silentCheckSsoRedirectUri: window.location.origin + '/silent-check-sso.html',
//             })
//             .then((auth) => {
//                 setIsAuthenticated(auth);
//                 if (auth) {
//                     setUsername(keycloak.tokenParsed?.preferred_username ?? null);
//                 }
//             });
//     }, []);
//
//     const login = () => keycloak.login();
//     const logout = () => keycloak.logout();
//
//     return (
//         <AuthContext.Provider value={{ isAuthenticated, username, login, logout }}>
//             {children}
//         </AuthContext.Provider>
//     );
// };
//
// export const useAuth = () => {
//     const ctx = useContext(AuthContext);
//     if (!ctx) throw new Error('useAuth must be used within AuthProvider');
//     return ctx;
// };
