import Keycloak, {
    type KeycloakAccountOptions,
    type KeycloakInitOptions,
    type KeycloakLoginOptions,
    type KeycloakLogoutOptions,
    type KeycloakProfile,
    type KeycloakRegisterOptions
} from "keycloak-js";

// This is a fake Keycloak instance to be used for in-memory (non-containerized) testing.

type FakeUser = {
    username: string;
    email: string;
    roles: string[];
};

// try to keep this and realm-export.json in sync for ease of in-memory vs. docker testing
const mockUsers: Record<string, FakeUser> = {
    'admin': {
        username: 'admin',
        email: 'admin@user.com',
        roles: ['admin', 'user'],
    },
    'bob': {
        username: 'bob',
        email: 'bob@user.com',
        roles: ['user'],
    },
    'clara': {
        username: 'clara',
        email: 'clara@user.com',
        roles: ['user'],
    },
};

// Internal state
let currentUser: FakeUser | null = null;

const keycloak: Keycloak = {

    didInitialize: false, accountManagement(): Promise<void> {
        return Promise.resolve(undefined);
    },

    clearToken(): void {
        return;
    },

    // @ts-expect-error: unused parameter
    createAccountUrl(options?: KeycloakAccountOptions): string {
        return "";
    },

    // @ts-expect-error: unused parameter
    createLoginUrl(options?: KeycloakLoginOptions): Promise<string> {
        return Promise.resolve("");
    },

    // @ts-expect-error: unused parameter
    createLogoutUrl(options?: KeycloakLogoutOptions): string {
        return "";
    },

    // @ts-expect-error: unused parameter
    createRegisterUrl(options?: KeycloakRegisterOptions): Promise<string> {
        return Promise.resolve("");
    },

    hasRealmRole(role: string): boolean {
        return currentUser?.roles.includes(role) ?? false;
    },

    // @ts-expect-error: unused parameters
    hasResourceRole(role: string, resource?: string): boolean {
        return false;
    },

    init(initOptions?: KeycloakInitOptions): Promise<boolean> {
        const onLoad = initOptions?.onLoad ?? 'check-sso';

        const savedUser = localStorage.getItem('fakeUser');
        if (savedUser && mockUsers[savedUser]) {
            currentUser = mockUsers[savedUser];
            this.authenticated = true;
            this.token = 'fake-jwt-token';
            this.tokenParsed = {
                preferred_username: currentUser.username,
                email: currentUser.email,
                realm_access: {
                    roles: currentUser.roles,
                },
            };
        } else {
            this.authenticated = false;
            this.token = undefined;
            this.tokenParsed = undefined;

            if (onLoad === 'login-required') {
                // Simulate interactive login (prompt or auto-login)
                const username = prompt('Enter username to login (e.g., admin, bob, clara):');
                if (username && mockUsers[username]) {
                    currentUser = mockUsers[username];
                    localStorage.setItem('fakeUser', username);
                    this.authenticated = true;
                    this.token = 'fake-jwt-token';
                    this.tokenParsed = {
                        preferred_username: currentUser.username,
                        email: currentUser.email,
                        realm_access: {
                            roles: currentUser.roles,
                        },
                    };
                } else {
                    alert('Invalid user. Staying unauthenticated.');
                    this.authenticated = false;
                }
            }
        }
        return Promise.resolve(this.authenticated || false);
    },

    // @ts-expect-error: unused parameter
    isTokenExpired(minValidity?: number): boolean {
        return false;
    },

    loadUserInfo(): Promise<{}> {
        return Promise.resolve({});
    },

    loadUserProfile(): Promise<KeycloakProfile> {
        if (!currentUser) {
            throw new Error('Not authenticated');
        }

        const profile: KeycloakProfile = {
            username: currentUser.username,
            email: currentUser.email,
            firstName: '',
            lastName: '',
        };

        this.profile = profile;
        return Promise.resolve(profile);
    },

    // @ts-expect-error: unused parameter
    login(options?: KeycloakLoginOptions): Promise<void> {
        // Simulate login UI (for dev you can auto-login or use prompt)
        const username = prompt('Enter username to login (e.g., admin, bob, clara):');

        if (username && mockUsers[username]) {
            currentUser = mockUsers[username];
            localStorage.setItem('fakeUser', username);
            this.authenticated = true;
            this.token = 'fake-jwt-token';
            this.tokenParsed = {
                preferred_username: currentUser.username,
                email: currentUser.email,
                realm_access: {
                    roles: currentUser.roles,
                },
            };

            // ✅ Redirect after login
            const redirectUrl = options?.redirectUri;
            if (redirectUrl) {
                window.location.href = redirectUrl;
            }

        } else {
            alert('Invalid user. Login aborted.');
            this.authenticated = false;
        }
    },

    // @ts-expect-error: unused parameter
    logout(options?: KeycloakLogoutOptions): Promise<void> {
        currentUser = null;
        this.authenticated = false;
        this.token = undefined;
        this.tokenParsed = undefined;
        localStorage.removeItem('fakeUser');

        const redirectUrl = options?.redirectUri;

        if (redirectUrl) {
            window.location.href = redirectUrl;
        }
    },

    // @ts-expect-error: unused parameter
    register(options?: KeycloakRegisterOptions): Promise<void> {
        return Promise.resolve(undefined);
    },

    // @ts-expect-error: unused parameter
    updateToken(minValidity?: number): Promise<boolean> {
        return Promise.resolve(true); // Token is always valid in dev
    }
}

export default keycloak;